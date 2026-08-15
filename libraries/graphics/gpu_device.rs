//! The GPU device, and the registry mapping a `DrawCommand`'s handles back to wgpu resources.
//!
//! **The device is global** because `Texture::load_from_surface(&surface)` is called from ~80
//! places and takes no context: the choice was threading a `&Device` through all of them or
//! keeping the coupling and writing it down. Creating a texture with no device is not an
//! error - it knows its size and owns nothing, which is what lets `cargo test` build one.
//!
//! **Resources are handles** because a `DrawCommand` outlives the borrow of whatever recorded
//! it. Dropping one parks the id and the backend sweeps after the frame executes, which is not
//! an edge case: menus build a text texture inside `render_inner` and drop it there, and every
//! chunk replaces its whole `RectArray` when it changes. **An immediate removal would make
//! those draws silently vanish.**

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};

use crate::libraries::graphics as gfx;

use super::wgpu_backend::VERTEX_FLOATS;

/// Flattens a surface into the `Rgba8Unorm` byte order the GPU wants. A copy rather than a
/// reinterpret: `bytemuck::Pod` on `Color` would pull a dependency into `color.rs`, which the
/// build script compiles and which deliberately has none. Textures are never built per frame.
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
    /// The blur's layout: the same two bindings, declared filterable.
    pub filtering_texture_bind_group_layout: wgpu::BindGroupLayout,
    /// Linear, for the blur alone. Everything else samples `sampler`.
    pub filtering_sampler: wgpu::Sampler,
    /// The bind group is all a draw needs; wgpu keeps the texture behind it alive.
    textures: Mutex<HashMap<u32, wgpu::BindGroup>>,
    meshes: Mutex<HashMap<u32, MeshEntry>>,
    next_id: AtomicU32,
    pending_textures: Mutex<Vec<u32>>,
    pending_meshes: Mutex<Vec<u32>>,
}

static GPU: OnceLock<GpuDevice> = OnceLock::new();

/// Publishes the device, once, as soon as wgpu hands one over. A second call is ignored rather
/// than an error: two `GraphicsContext`s in one process share the first device.
pub(super) fn init(device: wgpu::Device, queue: wgpu::Queue) {
    let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("texture"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    // Nearest only. Declaring it non-filterable makes smoothing a pixel art
                    // game by accident a validation error rather than a blur nobody notices.
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

    // The one exception, and it gets its own layout to stay one: the blur is stretched back
    // from a reduced-resolution copy, which `NEAREST` would turn into visible blocks.
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
            // Not the sRGB variant: the surfaces are raw bytes blended in that space, and
            // converting in hardware would change every colour the game ever drew.
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

    /// Uploads raw vertex data and returns its registry id. The count is derived from the data
    /// rather than passed alongside, so a mesh cannot claim more vertices than it was given.
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

    /// Locks the texture registry for a frame's encoding: a guard rather than a closure per
    /// lookup, so the backend can hold bind group references across a render pass. Meshes are
    /// always locked after this, and nothing else takes both, so the order is fixed.
    pub(super) fn lock_textures(&self) -> MutexGuard<'_, HashMap<u32, wgpu::BindGroup>> {
        lock(&self.textures)
    }

    pub(super) fn lock_meshes(&self) -> MutexGuard<'_, HashMap<u32, MeshEntry>> {
        lock(&self.meshes)
    }

    /// Releases everything parked since the last sweep. Only the backend calls it, and only
    /// once the frame's commands are submitted.
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
