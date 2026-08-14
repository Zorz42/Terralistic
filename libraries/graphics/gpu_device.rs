//! The GPU device, and the registry that maps a `DrawCommand`'s handles back to real wgpu
//! resources.
//!
//! # Why the device is global
//!
//! `Texture::load_from_surface(&surface)` is called from about eighty places and takes no
//! context. The choice was to thread a `&Device` through every one of those call sites or to
//! keep the coupling and write it down; this is the latter. Creating a texture with no device
//! is not an error, it produces a `Texture` that knows its size but owns nothing, which is
//! what lets layout code and `cargo test` run with no window.
//!
//! # Why resources are handles into a registry
//!
//! A `DrawCommand` has to outlive the borrow of whatever recorded it, so it names resources by
//! id. The registry owns the wgpu objects and hands out ids; `Texture` and `VertexBuffer` are
//! RAII wrappers over one.
//!
//! Dropping one parks the id rather than removing the entry, and the backend sweeps once the
//! frame's commands have executed. That gap is not an edge case: `login.rs` builds a text
//! texture inside `render_inner` and drops it there, every world chunk replaces its whole
//! `RectArray` when it changes, and the golden-image cases draw from temporaries that die at
//! the end of the statement. **Removing an entry at drop time would make those draws silently
//! vanish.**

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};

use crate::libraries::graphics as gfx;

use super::wgpu_backend::VERTEX_FLOATS;

/// Flattens a surface's pixels into the `Rgba8Unorm` byte order the GPU wants.
///
/// A copy rather than a reinterpret of the `Vec<Color>`: making `Color` `bytemuck::Pod` would
/// pull a dependency into `color.rs`, which `build_main.rs` compiles into the build script and
/// which deliberately has none. Textures are created when a menu or a font is built, never per
/// frame, so the extra pass does not matter.
fn surface_bytes(surface: &gfx::Surface) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(surface.pixels.len() * 4);
    for pixel in &surface.pixels {
        bytes.extend_from_slice(&[pixel.r, pixel.g, pixel.b, pixel.a]);
    }
    bytes
}

/// An uploaded vertex buffer. There is no index buffer: the indices would always be `0..n`.
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
    /// The blur's layout: the same two bindings, but declared filterable. See `filtering_sampler`.
    pub filtering_texture_bind_group_layout: wgpu::BindGroupLayout,
    /// Linear, for the blur alone. Everything else in the toolkit samples `sampler`.
    pub filtering_sampler: wgpu::Sampler,
    /// The bind group is all a draw needs; wgpu keeps the texture and the view behind it alive
    /// by reference.
    textures: Mutex<HashMap<u32, wgpu::BindGroup>>,
    meshes: Mutex<HashMap<u32, MeshEntry>>,
    next_id: AtomicU32,
    pending_textures: Mutex<Vec<u32>>,
    pending_meshes: Mutex<Vec<u32>>,
}

static GPU: OnceLock<GpuDevice> = OnceLock::new();

/// Publishes the device. Called once, by the renderer, as soon as wgpu hands one over. A
/// second call is ignored rather than an error: two `GraphicsContext`s in one process share
/// the first device.
pub(super) fn init(device: wgpu::Device, queue: wgpu::Queue) {
    let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("texture"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    // Nearest only, everywhere. Filtering is what a pixel art game must not do,
                    // and declaring it non-filterable makes that a validation error rather than
                    // a soft blur nobody notices.
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

    // The one exception to "nearest only", and it needs a layout of its own to stay one: the
    // blur runs on a reduced-resolution copy of the region and is stretched back over it, which
    // `NEAREST` would turn into visible blocks. Interpolating *is* the effect here rather than
    // an accident, so it gets its own layout and sampler and nothing else can reach them.
    let filtering_texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("filtering texture"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
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

    let filtering_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("linear clamp"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::MipmapFilterMode::Nearest,
        ..Default::default()
    });

    drop(GPU.set(GpuDevice {
        device,
        queue,
        texture_bind_group_layout,
        sampler,
        filtering_texture_bind_group_layout,
        filtering_sampler,
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

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Removes every parked id from its registry, taking the registry lock only if there is
/// anything to remove.
fn sweep<T>(pending: &Mutex<Vec<u32>>, registry: &Mutex<HashMap<u32, T>>) {
    let ids = std::mem::take(&mut *lock(pending));
    if ids.is_empty() {
        return;
    }
    let mut registry = lock(registry);
    for id in ids {
        registry.remove(&id);
    }
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
            // Not the sRGB variant: the game's surfaces are raw bytes blended in that space,
            // and asking the hardware to convert would change every colour it ever drew.
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
        lock(&self.textures).insert(id, bind_group);
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

    /// The same view bound for linear sampling. Only the blur's pipelines take this layout.
    pub(super) fn create_filtering_texture_bind_group(&self, view: &wgpu::TextureView) -> wgpu::BindGroup {
        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("filtering texture"),
            layout: &self.filtering_texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.filtering_sampler),
                },
            ],
        })
    }

    /// Uploads raw vertex data and returns its registry id.
    ///
    /// The vertex count is derived from the data rather than passed alongside it, so a mesh
    /// cannot be registered claiming to hold more vertices than it was given.
    pub(super) fn create_mesh(&self, vertices: &[f32]) -> u32 {
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
        lock(&self.meshes).insert(
            id,
            MeshEntry {
                buffer,
                vertex_count: (vertices.len() / VERTEX_FLOATS) as u32,
            },
        );
        id
    }

    /// Locks the texture registry for the length of a frame's encoding.
    ///
    /// A guard rather than a closure per lookup, so the backend can hold the bind group
    /// references it needs across a whole render pass. The mesh registry is always locked
    /// after this one, and nothing else takes both, so the order is fixed.
    pub(super) fn lock_textures(&self) -> MutexGuard<'_, HashMap<u32, wgpu::BindGroup>> {
        lock(&self.textures)
    }

    pub(super) fn lock_meshes(&self) -> MutexGuard<'_, HashMap<u32, MeshEntry>> {
        lock(&self.meshes)
    }

    /// Releases everything parked since the last sweep. Only the backend calls this, and only
    /// once the frame's commands have been submitted.
    pub(super) fn collect(&self) {
        sweep(&self.pending_textures, &self.textures);
        sweep(&self.pending_meshes, &self.meshes);
    }
}

/// Parks a texture id. See the module docs for why this is not an immediate removal.
pub(super) fn delete_texture_later(id: u32) {
    if let Some(gpu) = get() {
        lock(&gpu.pending_textures).push(id);
    }
}

pub(super) fn delete_mesh_later(id: u32) {
    if let Some(gpu) = get() {
        lock(&gpu.pending_meshes).push(id);
    }
}

/// How many ids are parked, for tests. Zero in a process with no device, since there is
/// nothing to park.
#[cfg(test)]
pub fn get_pending_counts() -> (usize, usize) {
    get().map_or((0, 0), |gpu| (lock(&gpu.pending_textures).len(), lock(&gpu.pending_meshes).len()))
}
