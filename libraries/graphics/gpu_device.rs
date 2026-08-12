//! The GPU device, and the registry that maps a `DrawCommand`'s handles back to real
//! wgpu resources.
//!
//! # Why the device is global
//!
//! `Texture::load_from_surface(&surface)` is called from about eighty places and takes no
//! context. Under OpenGL that worked because the context was an implicit thread-global; wgpu
//! has no such thing, so the choice was to thread a `&Device` through every one of those
//! call sites or to keep the implicit global and make it explicit. This is the latter. It is
//! the same coupling the code already had, just written down.
//!
//! One consequence is an improvement: creating a texture with no device no longer explodes,
//! it produces a `Texture` that knows its size but owns nothing. Layout code works headlessly
//! and `cargo test` can construct real textures.
//!
//! # Why resources are handles into a registry
//!
//! A `DrawCommand` has to outlive the borrow of whatever recorded it, so it names resources
//! by id. The registry owns the wgpu objects and hands out ids. `Texture` and `VertexBuffer`
//! are then just RAII wrappers over an id.
//!
//! Dropping one does not remove its entry immediately - it parks the id, and the backend
//! sweeps after the frame's commands have executed. That gap is not an edge case: `login.rs`
//! builds a text texture inside `render_inner` and drops it there, every world chunk replaces
//! its whole `RectArray` when it changes, and the golden-image cases draw from temporaries
//! that die at the end of the statement. Removing the entry at drop time would leave those
//! commands pointing at nothing.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock, PoisonError};

use crate::libraries::graphics as gfx;

/// Flattens a surface's pixels into the `Rgba8Unorm` byte order the GPU wants.
///
/// A copy rather than a reinterpret of the `Vec<Color>`. Making `Color` `bytemuck::Pod`
/// would be free, but `color.rs` is one of the leaf files `build_main.rs` compiles into the
/// build script, and that deliberately has none of the game's dependencies. Textures are
/// created when a menu or a font is built, never per frame, so the extra pass does not
/// matter.
fn surface_bytes(surface: &gfx::Surface) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(surface.pixels.len() * 4);
    for pixel in &surface.pixels {
        bytes.extend_from_slice(&[pixel.r, pixel.g, pixel.b, pixel.a]);
    }
    bytes
}

/// An uploaded vertex buffer. There is no index buffer: the OpenGL version had one, but its
/// indices were always `0..n`, so it described nothing the vertex order did not.
pub(super) struct MeshEntry {
    pub buffer: wgpu::Buffer,
    pub vertex_count: u32,
}

pub(super) struct GpuDevice {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    /// Shared by every texture bind group and by the backend's pipeline layout.
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub sampler: wgpu::Sampler,
    /// The bind group is all a draw needs. wgpu keeps the texture and the view behind it
    /// alive by reference, so there is nothing else to hold on to.
    textures: Mutex<HashMap<u32, wgpu::BindGroup>>,
    meshes: Mutex<HashMap<u32, MeshEntry>>,
    next_id: AtomicU32,
    pending_textures: Mutex<Vec<u32>>,
    pending_meshes: Mutex<Vec<u32>>,
}

static GPU: OnceLock<GpuDevice> = OnceLock::new();

/// Publishes the device. Called once, by the renderer, as soon as wgpu hands one over.
///
/// A second call is ignored rather than an error: two `GraphicsContext`s in one process
/// would share the first device, which is exactly what the OpenGL version did with its
/// context.
pub(super) fn init(device: wgpu::Device, queue: wgpu::Queue) {
    let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("texture"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    // Nearest only, everywhere. Filtering is what a pixel art game must not
                    // do, and declaring it non-filterable makes that a validation error
                    // rather than a soft blur nobody notices.
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                count: None,
            },
        ],
    });

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("nearest clamp"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::MipmapFilterMode::Nearest,
        ..Default::default()
    });

    // A second call is a no-op: the first device wins.
    drop(GPU.set(GpuDevice {
        device,
        queue,
        texture_bind_group_layout,
        sampler,
        textures: Mutex::new(HashMap::new()),
        meshes: Mutex::new(HashMap::new()),
        // 0 is never handed out, so it can mean "no resource" in a handle.
        next_id: AtomicU32::new(1),
        pending_textures: Mutex::new(Vec::new()),
        pending_meshes: Mutex::new(Vec::new()),
    }));
}

/// The device, or `None` in a process that never opened a window.
pub(super) fn get() -> Option<&'static GpuDevice> {
    GPU.get()
}

impl GpuDevice {
    fn take_id(&self) -> u32 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }

    /// Uploads a surface and returns its registry id.
    pub(super) fn create_texture(&self, surface: &gfx::Surface) -> u32 {
        let size = surface.get_size();
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("texture"),
            size: wgpu::Extent3d {
                width: size.0.max(1),
                height: size.1.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Not the sRGB variant. The game's surfaces are raw bytes that the OpenGL
            // renderer uploaded as RGBA8 and blended in that space; asking the hardware to
            // convert would change every colour it ever drew.
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        if size.0 > 0 && size.1 > 0 {
            self.queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &surface_bytes(surface),
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(size.0 * 4),
                    rows_per_image: Some(size.1),
                },
                wgpu::Extent3d {
                    width: size.0,
                    height: size.1,
                    depth_or_array_layers: 1,
                },
            );
        }

        let id = self.take_id();
        let bind_group = self.create_texture_bind_group(&texture.create_view(&wgpu::TextureViewDescriptor::default()));
        self.textures.lock().unwrap_or_else(PoisonError::into_inner).insert(id, bind_group);
        id
    }

    pub(super) fn create_texture_bind_group(&self, view: &wgpu::TextureView) -> wgpu::BindGroup {
        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("texture"),
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        })
    }

    /// Uploads raw vertex data and returns its registry id.
    pub(super) fn create_mesh(&self, vertices: &[f32], vertex_count: u32) -> u32 {
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("mesh"),
            // A zero sized buffer is not allowed, and an empty mesh draws nothing anyway.
            size: (std::mem::size_of_val(vertices) as u64).max(4),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        if !vertices.is_empty() {
            self.queue.write_buffer(&buffer, 0, bytemuck::cast_slice(vertices));
        }

        let id = self.take_id();
        self.meshes.lock().unwrap_or_else(PoisonError::into_inner).insert(id, MeshEntry { buffer, vertex_count });
        id
    }

    /// Locks the texture registry for the length of a frame's encoding.
    ///
    /// Taking a guard rather than a closure per lookup is what lets the backend hold the
    /// bind group references it needs across a whole render pass. The mesh registry is
    /// always locked after this one, and nothing else takes both, so the order is fixed.
    pub(super) fn lock_textures(&self) -> std::sync::MutexGuard<'_, HashMap<u32, wgpu::BindGroup>> {
        self.textures.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub(super) fn lock_meshes(&self) -> std::sync::MutexGuard<'_, HashMap<u32, MeshEntry>> {
        self.meshes.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Releases everything parked since the last sweep. Only the backend calls this, and
    /// only once the frame's commands have been submitted.
    pub(super) fn collect(&self) {
        let textures = std::mem::take(&mut *self.pending_textures.lock().unwrap_or_else(PoisonError::into_inner));
        let meshes = std::mem::take(&mut *self.pending_meshes.lock().unwrap_or_else(PoisonError::into_inner));

        if !textures.is_empty() {
            let mut registry = self.textures.lock().unwrap_or_else(PoisonError::into_inner);
            for id in textures {
                registry.remove(&id);
            }
        }
        if !meshes.is_empty() {
            let mut registry = self.meshes.lock().unwrap_or_else(PoisonError::into_inner);
            for id in meshes {
                registry.remove(&id);
            }
        }
    }
}

/// Parks a texture id. See the module docs for why this is not an immediate removal.
pub(super) fn delete_texture_later(id: u32) {
    if let Some(gpu) = get() {
        gpu.pending_textures.lock().unwrap_or_else(PoisonError::into_inner).push(id);
    }
}

pub(super) fn delete_mesh_later(id: u32) {
    if let Some(gpu) = get() {
        gpu.pending_meshes.lock().unwrap_or_else(PoisonError::into_inner).push(id);
    }
}

/// How many ids are parked, for tests. Zero in a process with no device, since there is
/// nothing to park.
#[cfg(test)]
pub fn get_pending_counts() -> (usize, usize) {
    get().map_or((0, 0), |gpu| {
        (
            gpu.pending_textures.lock().unwrap_or_else(PoisonError::into_inner).len(),
            gpu.pending_meshes.lock().unwrap_or_else(PoisonError::into_inner).len(),
        )
    })
}
