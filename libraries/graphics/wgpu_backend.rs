//! The wgpu half of the renderer: how a `DrawList` becomes pixels.
//!
//! `execute` runs in two phases. The first walks the command list and turns it into a flat
//! list of `Segment`s plus one `Uniforms` entry per draw, because wgpu wants all the uniform
//! data written before any of it is encoded. The second encodes the segments into render
//! passes. A `Blur` command splits the frame, since blurring means sampling what has already
//! been drawn and wgpu cannot sample the texture it is currently drawing into.
//!
//! Two things about the output are deliberate and easy to undo by accident:
//!
//! - **Rectangle outlines are four thin quads, not line primitives.** Line rasterisation rules
//!   differ between Metal, Vulkan and DX12, which would make the output depend on the machine
//!   - the opposite of what the golden images are for. Quads put the border exactly on the
//!   rectangle's own pixels.
//! - **Texture draws are snapped to a whole pixel** of the offscreen. See `plan_command`.

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, Result};

use crate::libraries::graphics as gfx;

use super::draw_list::{BlendMode, DrawCommand, DrawList};
use super::gpu_device::{self, GpuDevice, MeshEntry};
use super::transformation::Transformation;

/// One vertex: position, colour, texture coordinate. Matches `VertexBuffer`'s packing so a
/// `RectArray` and the built-in quad can share a pipeline.
pub(super) const VERTEX_FLOATS: usize = 8;

const SHADER: &str = include_str!("shaders.wgsl");

/// The per-draw uniform block. Laid out by hand to match WGSL's rules: a `mat3x3` occupies
/// three 16 byte columns, and the whole struct aligns to 16.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    transform: [[f32; 4]; 3],
    texture_transform: [[f32; 4]; 3],
    color: [f32; 4],
    /// Blur only: the region outside which sampling is clamped, as (max u, max v, min u, min v).
    limit: [f32; 4],
    /// Blur only: how far apart the thirteen taps are.
    blur_offset: [f32; 2],
    has_texture: u32,
    _padding: u32,
}

impl Uniforms {
    fn new(transform: &Transformation, texture_transform: &Transformation, color: gfx::Color, has_texture: bool) -> Self {
        Self {
            transform: expand(transform),
            texture_transform: expand(texture_transform),
            color: [color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0, color.a as f32 / 255.0],
            limit: [0.0; 4],
            blur_offset: [0.0; 2],
            has_texture: u32::from(has_texture),
            _padding: 0,
        }
    }
}

/// A column major 3x3 becomes three padded 4-float columns.
const fn expand(transform: &Transformation) -> [[f32; 4]; 3] {
    let m = &transform.matrix;
    [[m[0], m[1], m[2], 0.0], [m[3], m[4], m[5], 0.0], [m[6], m[7], m[8], 0.0]]
}

/// What to draw with, once a command has been resolved against the registry.
#[derive(Clone, Copy)]
enum Geometry {
    /// The built-in unit quad, which every rect and texture draw is a transform of.
    Quad,
    /// A mesh from the registry, by id.
    Mesh(u32),
}

enum Segment {
    Draw {
        uniform: u32,
        blend: BlendMode,
        /// Registry id of the texture to sample, or `None` for the white dummy.
        texture: Option<u32>,
        geometry: Geometry,
    },
    /// One gaussian pass. `to_back` picks which of the ping-pong pair is the attachment.
    BlurPass { uniform: u32, to_back: bool },
}

/// A frame being planned: what to encode, and the uniform data it indexes into.
struct Plan {
    uniforms: Vec<Uniforms>,
    segments: Vec<Segment>,
    /// The blend mode in force at this point in the list.
    blend: BlendMode,
}

impl Plan {
    const fn new() -> Self {
        Self {
            uniforms: Vec::new(),
            segments: Vec::new(),
            blend: BlendMode::Alpha,
        }
    }

    fn draw(&mut self, uniform: Uniforms, texture: Option<u32>, geometry: Geometry) {
        self.segments.push(Segment::Draw {
            uniform: self.uniforms.len() as u32,
            blend: self.blend,
            texture,
            geometry,
        });
        self.uniforms.push(uniform);
    }

    fn blur_pass(&mut self, uniform: Uniforms, to_back: bool) {
        self.segments.push(Segment::BlurPass {
            uniform: self.uniforms.len() as u32,
            to_back,
        });
        self.uniforms.push(uniform);
    }
}

/// The two offscreen textures the frame is drawn into and blurred between.
struct Offscreen {
    /// Only the golden-image readback needs the texture itself; the view keeps it alive.
    #[cfg_attr(not(feature = "render-tests"), allow(dead_code, reason = "only read back when capturing goldens"))]
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
}

impl Offscreen {
    fn new(gpu: &GpuDevice, size: gfx::IntSize, label: &str) -> Self {
        let texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width: size.0.max(1),
                height: size.1.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = gpu.create_texture_bind_group(&view);
        Self { texture, view, bind_group }
    }
}

pub struct WgpuBackend {
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    surface_size: gfx::IntSize,
    present_mode: wgpu::PresentMode,

    front: Offscreen,
    back: Offscreen,
    size: gfx::IntSize,

    /// Indexed by `BlendMode`; the blend state is baked into a wgpu pipeline, so switching
    /// modes mid-frame means switching pipeline.
    alpha_pipeline: wgpu::RenderPipeline,
    multiply_pipeline: wgpu::RenderPipeline,
    blur_pipeline: wgpu::RenderPipeline,
    present_pipeline: wgpu::RenderPipeline,

    quad_buffer: wgpu::Buffer,
    white_bind_group: wgpu::BindGroup,

    uniform_layout: wgpu::BindGroupLayout,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    uniform_capacity: u64,
    /// Distance between consecutive uniform entries, rounded up to the device's alignment.
    uniform_stride: u32,
    /// Staging bytes for `write_uniforms`, kept so a frame does not allocate.
    uniform_bytes: Vec<u8>,

    normalization_transform: Transformation,
    blur_enabled: bool,
    blur_intensity: f32,
    blur_animation_timer: gfx::AnimationTimer,
    /// Set by the golden-image harness so the next frame starts from a known buffer.
    clear_next_frame: bool,
}

impl WgpuBackend {
    /// Brings up the device, publishes it for resource creation, and builds the pipelines.
    ///
    /// Takes the window by `Arc` so the surface can own a share of it. That is what makes
    /// the surface `'static` without the unsafe raw-handle constructor, and it means the
    /// window cannot be dropped out from under wgpu no matter what order anything else
    /// holding one is destroyed in.
    pub(super) fn new(window: &Arc<winit::window::Window>, size: gfx::IntSize, drawable_size: gfx::IntSize) -> Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: wgpu::InstanceFlags::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options: wgpu::BackendOptions::default(),
            display: None,
        });

        let surface = instance.create_surface(Arc::clone(window))?;

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
            apply_limit_buckets: false,
        }))
        .map_err(|error| anyhow!("no graphics adapter available: {error}"))?;

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("terralistic"),
            required_features: wgpu::Features::empty(),
            // The toolkit draws textured triangles and nothing else, so the lowest common
            // denominator is enough and keeps the oldest hardware working - except for
            // texture size, which is not a matter of taste: the offscreen pair and the
            // surface are as big as the window, and `downlevel_defaults` caps a texture at
            // 2048. A 1670x1050 window on a 2x display is already 3340 pixels across, so the
            // default would refuse to configure the surface at all. `using_resolution` keeps
            // everything else conservative and takes only the dimension limits from the
            // adapter.
            required_limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: wgpu::MemoryHints::default(),
            trace: wgpu::Trace::Off,
        }))?;

        gpu_device::init(device, queue);
        let gpu = gpu_device::get().ok_or_else(|| anyhow!("the gpu device disappeared right after being published"))?;

        let capabilities = surface.get_capabilities(&adapter);
        let surface_format = *capabilities
            .formats
            .iter()
            .find(|format| !format.is_srgb())
            .or_else(|| capabilities.formats.first())
            .ok_or_else(|| anyhow!("the surface supports no texture formats"))?;

        let Pipelines {
            uniform_layout,
            alpha_pipeline,
            multiply_pipeline,
            blur_pipeline,
            present_pipeline,
        } = build_pipelines(gpu, surface_format);

        let quad_buffer = build_quad_buffer(gpu);
        let white_bind_group = build_white_bind_group(gpu);

        // Rounded up to the alignment rather than just `max`ed against it: a dynamic offset has
        // to be a multiple of it, and `max` only happens to give one because the requested
        // limits pin the alignment at 256 and `Uniforms` is smaller than that.
        let uniform_stride = (std::mem::size_of::<Uniforms>() as u32).next_multiple_of(gpu.device.limits().min_uniform_buffer_offset_alignment);
        let uniform_capacity = u64::from(uniform_stride) * 256;
        let uniform_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniforms"),
            size: uniform_capacity,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let uniform_bind_group = build_uniform_bind_group(gpu, &uniform_layout, &uniform_buffer);

        let result = Self {
            surface,
            surface_format,
            surface_size: drawable_size,
            present_mode: wgpu::PresentMode::AutoVsync,
            front: Offscreen::new(gpu, size, "window texture"),
            back: Offscreen::new(gpu, size, "window texture back"),
            size,
            alpha_pipeline,
            multiply_pipeline,
            blur_pipeline,
            present_pipeline,
            quad_buffer,
            white_bind_group,
            uniform_layout,
            uniform_buffer,
            uniform_bind_group,
            uniform_capacity,
            uniform_stride,
            uniform_bytes: Vec::new(),
            normalization_transform: Transformation::new(),
            blur_enabled: true,
            blur_intensity: 0.0,
            blur_animation_timer: gfx::AnimationTimer::new(10),
            clear_next_frame: false,
        };
        result.configure_surface();
        Ok(result)
    }

    fn configure_surface(&self) {
        let Some(gpu) = gpu_device::get() else { return };
        if self.surface_size.0 == 0 || self.surface_size.1 == 0 {
            return;
        }
        self.surface.configure(
            &gpu.device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: self.surface_format,
                color_space: wgpu::SurfaceColorSpace::Auto,
                width: self.surface_size.0,
                height: self.surface_size.1,
                present_mode: self.present_mode,
                desired_maximum_frame_latency: 2,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![],
            },
        );
    }

    /// Reallocates the offscreen pair and reconfigures the surface.
    ///
    /// `offscreen_size` is the resolution the frame is *drawn* at and `surface_size` the one
    /// it is presented at. The game normally asks for both to be the display's real pixel
    /// size, so nothing is upscaled; the golden-image harness asks for a logical-sized
    /// offscreen instead - see `GraphicsContext::handle_window_resize`.
    pub(super) fn resize(&mut self, offscreen_size: gfx::IntSize, surface_size: gfx::IntSize) {
        let Some(gpu) = gpu_device::get() else { return };
        if offscreen_size != self.size {
            self.front = Offscreen::new(gpu, offscreen_size, "window texture");
            self.back = Offscreen::new(gpu, offscreen_size, "window texture back");
            self.size = offscreen_size;
        }
        if surface_size != self.surface_size {
            self.surface_size = surface_size;
            self.configure_surface();
        }
    }

    /// Rounds a destination position onto a whole offscreen pixel.
    ///
    /// The coordinates a command carries are logical, and the offscreen usually has more
    /// than one pixel per logical unit, so this is a finer grid than "whole logical pixel" -
    /// which is the point. It is what a smooth animation gets to move along.
    fn snap_to_pixel(&self, pos: gfx::FloatPos, window_size: gfx::FloatSize) -> gfx::FloatPos {
        let per_logical_x = self.size.0 as f32 / window_size.0;
        let per_logical_y = self.size.1 as f32 / window_size.1;
        if per_logical_x <= 0.0 || per_logical_y <= 0.0 {
            return pos;
        }
        gfx::FloatPos((pos.0 * per_logical_x).round() / per_logical_x, (pos.1 * per_logical_y).round() / per_logical_y)
    }

    pub(super) fn set_vsync(&mut self, enable: bool) {
        let wanted = if enable { wgpu::PresentMode::AutoVsync } else { wgpu::PresentMode::AutoNoVsync };
        if wanted != self.present_mode {
            self.present_mode = wanted;
            self.configure_surface();
        }
    }

    /// Recomputes the transform that maps window pixel coordinates onto clip space.
    ///
    /// The negative y scale is the graphics convention: clip space is y up, while every
    /// coordinate in this toolkit is y down from the top left.
    pub(super) fn update_normalization_transform(&mut self, window_size: gfx::FloatSize) {
        self.normalization_transform = Transformation::new();
        self.normalization_transform.translate(gfx::FloatPos(-1.0, 1.0));
        self.normalization_transform.stretch((2.0 / window_size.0, -2.0 / window_size.1));
    }

    /// Draws a whole frame, then releases anything the frame dropped.
    pub(super) fn execute(&mut self, list: &DrawList, window_size: gfx::FloatSize) {
        let Some(gpu) = gpu_device::get() else { return };

        let mut plan = Plan::new();
        for command in list.get_commands() {
            self.plan_command(command, window_size, &mut plan);
        }

        let clear = std::mem::take(&mut self.clear_next_frame);
        if plan.uniforms.is_empty() && !clear {
            gpu.collect();
            return;
        }

        self.write_uniforms(gpu, &plan.uniforms);

        let mut encoder = gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("frame") });
        self.encode(gpu, &mut encoder, &plan.segments, clear);
        gpu.queue.submit(Some(encoder.finish()));

        gpu.collect();
    }

    fn plan_command(&self, command: &DrawCommand, window_size: gfx::FloatSize, plan: &mut Plan) {
        match *command {
            DrawCommand::Rect { rect, color } => {
                plan.draw(self.rect_uniform(rect, color), None, Geometry::Quad);
            }
            DrawCommand::RectOutline { rect, color } => {
                // Four one pixel quads on the rectangle's own edge pixels. See the module
                // docs: a line primitive would rasterise differently per backend.
                for edge in outline_edges(rect) {
                    plan.draw(self.rect_uniform(edge, color), None, Geometry::Quad);
                }
            }
            DrawCommand::Texture {
                texture,
                texture_size,
                src_rect,
                pos,
                scale,
                flipped,
                color,
            } => {
                // **Snapped to a whole pixel**, which keeps a texel from coming out a pixel
                // wider than its neighbour.
                //
                // Sampling is `NEAREST`, so a destination pixel takes whichever texel its
                // centre falls in. With the quad starting on a whole pixel and an integer
                // scale, those centres land halfway through a texel and every texel gets the
                // same number of pixels. Off by half and they land exactly *on* the boundaries
                // instead, where which side they fall on comes down to the last bit of a float
                // interpolated across the quad - so a 3x glyph pixel comes out 2 or 4 wide,
                // and differently along the string, which reads as uneven, faintly slanted
                // text. Layout puts things on half pixels constantly.
                //
                // The grid is the offscreen's, not the logical one, so on a `HiDPI` display
                // this still leaves half a logical pixel of movement to animate along. Meshes
                // are **not** snapped: `RectArray` maps texture coordinates per vertex, and
                // the world would jitter against the camera.
                let pos = self.snap_to_pixel(pos, window_size);

                let mut transform = self.normalization_transform.clone();
                if flipped {
                    transform.translate(gfx::FloatPos(src_rect.size.0 * scale + pos.0 * 2.0, 0.0));
                    transform.stretch((-1.0, 1.0));
                }
                transform.translate(pos);
                transform.stretch((src_rect.size.0 * scale, src_rect.size.1 * scale));

                // The unit square maps onto the source rectangle and then onto [0,1] of the
                // whole texture. The stretch is by exactly `src_rect.size` - **no fudge
                // factor**. Sampling happens at texel centres, so the last output column
                // already lands strictly inside the region; inflating the region pushes it
                // into the neighbouring texel, which shows up as a sliver of the wrong sprite.
                let mut texture_transform = texel_scale(texture_size);
                texture_transform.translate(src_rect.pos);
                texture_transform.stretch((src_rect.size.0, src_rect.size.1));

                plan.draw(Uniforms::new(&transform, &texture_transform, color, true), Some(texture.get_id()), Geometry::Quad);
            }
            DrawCommand::Mesh { mesh, texture, pos } => {
                // A hundredth of a pixel, to keep a vertex that lands exactly on a pixel
                // boundary from rasterising into the wrong one and seaming the chunk grid.
                let pos = gfx::FloatPos(pos.0 + 0.01, pos.1 + 0.01);
                let mut transform = self.normalization_transform.clone();
                transform.translate(pos);

                let (texture_id, texture_transform) = match texture {
                    Some((handle, size)) => (Some(handle.get_id()), texel_scale(size)),
                    None => (None, Transformation::new()),
                };

                plan.draw(
                    Uniforms::new(&transform, &texture_transform, gfx::Color::new(255, 255, 255, 255), texture.is_some()),
                    texture_id,
                    Geometry::Mesh(mesh.get_id()),
                );
            }
            DrawCommand::Blur { rect, radius } => self.plan_blur(rect, radius, window_size, plan),
            DrawCommand::SetBlendMode(mode) => plan.blend = mode,
        }
    }

    fn rect_uniform(&self, rect: gfx::Rect, color: gfx::Color) -> Uniforms {
        let mut transform = self.normalization_transform.clone();
        transform.translate(rect.pos);
        transform.stretch((rect.size.0, rect.size.1));
        Uniforms::new(&transform, &Transformation::new(), color, false)
    }

    /// Turns one blur command into its ping-pong passes: two per axis, plus a wider pair once
    /// the radius is large enough to need it.
    fn plan_blur(&self, rect: gfx::Rect, radius: i32, window_size: gfx::FloatSize, plan: &mut Plan) {
        let radius = radius as f32 * self.blur_intensity / 5.0;
        if radius < 1.0 {
            return;
        }

        let begin_x = rect.pos.0.max(0.0);
        let begin_y = rect.pos.1.max(0.0);
        let end_x = (rect.pos.0 + rect.size.0).min(window_size.0);
        let end_y = (rect.pos.1 + rect.size.1).min(window_size.1);
        let rect = gfx::Rect::new(gfx::FloatPos(begin_x, begin_y), gfx::FloatSize(end_x - begin_x, end_y - begin_y));
        if rect.size.0 <= 0.0 || rect.size.1 <= 0.0 {
            return;
        }

        let x1 = (rect.pos.0 + 1.0) / window_size.0;
        let y1 = (rect.pos.1 + 1.0) / window_size.1;
        let x2 = (rect.pos.0 + rect.size.0 - 1.0) / window_size.0;
        let y2 = (rect.pos.1 + rect.size.1 - 1.0) / window_size.1;

        let mut transform = self.normalization_transform.clone();
        transform.translate(rect.pos);
        transform.stretch((rect.size.0, rect.size.1));

        let mut texture_transform = Transformation::new();
        texture_transform.stretch((1.0 / window_size.0, 1.0 / window_size.1));
        texture_transform.translate(rect.pos);
        texture_transform.stretch((rect.size.0, rect.size.1));

        let mut offsets = vec![
            [0.0, radius / window_size.1 / 10.0],
            [radius / window_size.0 / 10.0, 0.0],
            [0.0, radius / window_size.1],
            [radius / window_size.0, 0.0],
        ];
        if radius > 5.0 {
            offsets.push([0.0, 2.0 / window_size.1]);
            offsets.push([2.0 / window_size.0, 0.0]);
        }

        for (index, offset) in offsets.into_iter().enumerate() {
            let mut uniform = Uniforms::new(&transform, &texture_transform, gfx::Color::new(255, 255, 255, 255), true);
            uniform.limit = [x2, y2, x1, y1];
            uniform.blur_offset = offset;

            // Even passes read the front and write the back, odd ones the other way round. The
            // count is always even, so the result ends up in the front texture.
            plan.blur_pass(uniform, index % 2 == 0);
        }
    }

    /// Packs one frame's uniforms into the buffer, one per stride, growing it if needed.
    fn write_uniforms(&mut self, gpu: &GpuDevice, uniforms: &[Uniforms]) {
        let needed = u64::from(self.uniform_stride) * uniforms.len() as u64;
        if needed > self.uniform_capacity {
            self.uniform_capacity = needed.next_power_of_two();
            self.uniform_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("uniforms"),
                size: self.uniform_capacity,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.uniform_bind_group = build_uniform_bind_group(gpu, &self.uniform_layout, &self.uniform_buffer);
        }

        // Kept between frames so a steady state frame does not allocate.
        let stride = self.uniform_stride as usize;
        self.uniform_bytes.clear();
        self.uniform_bytes.resize(stride * uniforms.len(), 0);
        for (index, uniform) in uniforms.iter().enumerate() {
            let start = index * stride;
            if let Some(slot) = self.uniform_bytes.get_mut(start..start + std::mem::size_of::<Uniforms>()) {
                slot.copy_from_slice(bytemuck::bytes_of(uniform));
            }
        }
        gpu.queue.write_buffer(&self.uniform_buffer, 0, &self.uniform_bytes);
    }

    /// Turns the planned segments into render passes.
    ///
    /// A frame is one pass per run of draws, split wherever a blur needs to read back what
    /// has been drawn so far.
    fn encode(&self, gpu: &GpuDevice, encoder: &mut wgpu::CommandEncoder, segments: &[Segment], clear: bool) {
        // The clear gets a pass of its own, on the frame's own target. Folding it into
        // whichever pass happens to come first is wrong for a frame that opens with a blur:
        // the first pass of one writes the *back* texture, so the clear landed there, the
        // front kept the previous frame, and the blur then read that back in as its source.
        // It also covers the frame that records nothing at all and still has to come out
        // empty rather than showing what was there before.
        if clear {
            drop(begin_pass(encoder, &self.front.view, true));
        }

        let textures = gpu.lock_textures();
        let meshes = gpu.lock_meshes();

        let mut rest = segments;
        while let Some(head) = rest.first() {
            let consumed = match *head {
                Segment::BlurPass { uniform, to_back } => {
                    self.encode_blur(encoder, uniform, to_back);
                    1
                }
                Segment::Draw { .. } => {
                    let run = rest.iter().position(|segment| matches!(segment, Segment::BlurPass { .. })).unwrap_or(rest.len());
                    self.encode_draws(encoder, rest.get(..run).unwrap_or_default(), &textures, &meshes);
                    run
                }
            };
            rest = rest.get(consumed..).unwrap_or_default();
        }
    }

    /// One gaussian pass, reading whichever offscreen texture is not being written.
    fn encode_blur(&self, encoder: &mut wgpu::CommandEncoder, uniform: u32, to_back: bool) {
        let (target, source) = if to_back { (&self.back, &self.front) } else { (&self.front, &self.back) };
        let mut pass = begin_pass(encoder, &target.view, false);
        pass.set_pipeline(&self.blur_pipeline);
        pass.set_vertex_buffer(0, self.quad_buffer.slice(..));
        pass.set_bind_group(1, &source.bind_group, &[]);
        pass.set_bind_group(0, &self.uniform_bind_group, &[uniform * self.uniform_stride]);
        pass.draw(0..6, 0..1);
    }

    /// One pass covering an unbroken run of draws.
    fn encode_draws(&self, encoder: &mut wgpu::CommandEncoder, segments: &[Segment], textures: &HashMap<u32, wgpu::BindGroup>, meshes: &HashMap<u32, MeshEntry>) {
        let mut pass = begin_pass(encoder, &self.front.view, false);
        for segment in segments {
            let &Segment::Draw { uniform, blend, texture, geometry } = segment else {
                continue;
            };

            // A missing entry means the resource was created without a device, which can
            // only happen in a process that cannot render anyway.
            let bind_group = match texture {
                None => &self.white_bind_group,
                Some(id) => match textures.get(&id) {
                    Some(bind_group) => bind_group,
                    None => continue,
                },
            };
            let (buffer, vertices) = match geometry {
                Geometry::Quad => (self.quad_buffer.slice(..), 6),
                Geometry::Mesh(id) => match meshes.get(&id) {
                    Some(mesh) if mesh.vertex_count > 0 => (mesh.buffer.slice(..), mesh.vertex_count),
                    _ => continue,
                },
            };

            pass.set_pipeline(match blend {
                BlendMode::Alpha => &self.alpha_pipeline,
                BlendMode::Multiply => &self.multiply_pipeline,
            });
            pass.set_bind_group(1, bind_group, &[]);
            pass.set_bind_group(0, &self.uniform_bind_group, &[uniform * self.uniform_stride]);
            pass.set_vertex_buffer(0, buffer);
            pass.draw(0..vertices, 0..1);
        }
    }

    /// Copies the offscreen texture onto the window.
    ///
    /// The surface is configured at the real drawable size and the offscreen is drawn over it
    /// as a nearest-sampled quad, so this is normally a copy rather than a scale.
    pub(super) fn present(&mut self) -> Result<()> {
        let Some(gpu) = gpu_device::get() else { return Ok(()) };

        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
            // Lost or outdated just means the window changed under us, so reconfigure and
            // let the next frame have it. The others are all "try again later".
            wgpu::CurrentSurfaceTexture::Lost | wgpu::CurrentSurfaceTexture::Outdated => {
                self.configure_surface();
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => return Ok(()),
            other @ wgpu::CurrentSurfaceTexture::Validation => return Err(anyhow!("could not acquire a frame to present: {other:?}")),
        };

        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut transform = Transformation::new();
        transform.translate(gfx::FloatPos(-1.0, 1.0));
        transform.stretch((2.0, -2.0));
        let uniform = Uniforms::new(&transform, &Transformation::new(), gfx::Color::new(255, 255, 255, 255), true);
        self.write_uniforms(gpu, std::slice::from_ref(&uniform));

        let mut encoder = gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("present") });
        {
            let mut pass = begin_pass(&mut encoder, &view, true);
            pass.set_pipeline(&self.present_pipeline);
            pass.set_vertex_buffer(0, self.quad_buffer.slice(..));
            pass.set_bind_group(1, &self.front.bind_group, &[]);
            pass.set_bind_group(0, &self.uniform_bind_group, &[0]);
            pass.draw(0..6, 0..1);
        }
        gpu.queue.submit(Some(encoder.finish()));
        gpu.queue.present(frame);
        Ok(())
    }

    /// Advances the blur fade by however many 10 ms frames have elapsed.
    pub(super) fn update_blur(&mut self) {
        let target = if self.blur_enabled { 1.0 } else { 0.0 };
        while self.blur_animation_timer.frame_ready() {
            self.blur_intensity = gfx::approach(self.blur_intensity, target, 10.0, 0.001);
        }
    }

    pub(super) const fn set_blur_enabled(&mut self, enable: bool) {
        self.blur_enabled = enable;
    }

    /// Jumps the blur fade to its target, so a golden does not depend on how many 10 ms
    /// frames happened to elapse before the capture.
    #[cfg(feature = "render-tests")]
    pub(super) const fn settle_blur(&mut self) {
        self.blur_intensity = if self.blur_enabled { 1.0 } else { 0.0 };
    }

    /// Asks for the next executed frame to start from a cleared buffer.
    #[cfg(feature = "render-tests")]
    pub(super) const fn clear_next_frame(&mut self) {
        self.clear_next_frame = true;
    }

    /// Reads the offscreen texture back into a `Surface`.
    ///
    /// Upstream of `present`, so what a golden records is what the game drew rather than what
    /// the display scaled it to.
    #[cfg(feature = "render-tests")]
    pub(super) fn read_pixels(&self) -> Result<gfx::Surface> {
        let gpu = gpu_device::get().ok_or_else(|| anyhow!("no gpu device"))?;
        let size = self.size;
        // Buffer rows have to start on a 256 byte boundary, so the readback is usually
        // wider than the image and has to be un-padded row by row.
        let bytes_per_row = (size.0 * 4).div_ceil(256) * 256;

        let buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("readback"),
            size: u64::from(bytes_per_row) * u64::from(size.1),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder = gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("readback") });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &self.front.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(size.1),
                },
            },
            wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
        );
        gpu.queue.submit(Some(encoder.finish()));

        let slice = buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |result| {
            // Nothing to do if the receiver has gone: the capture was abandoned.
            drop(sender.send(result));
        });
        gpu.device.poll(wgpu::PollType::wait_indefinitely())?;
        receiver.recv()??;

        let data = slice.get_mapped_range()?;
        let mut result = gfx::Surface::new(size);
        for (pos, pixel) in result.iter_mut() {
            let offset = (pos.1 as u32 * bytes_per_row + pos.0 as u32 * 4) as usize;
            if let Some(bytes) = data.get(offset..offset + 4) {
                *pixel = gfx::Color::new(*bytes.first().unwrap_or(&0), *bytes.get(1).unwrap_or(&0), *bytes.get(2).unwrap_or(&0), *bytes.get(3).unwrap_or(&0));
            }
        }
        drop(data);
        buffer.unmap();
        Ok(result)
    }
}

/// Everything built once from the shader module.
struct Pipelines {
    uniform_layout: wgpu::BindGroupLayout,
    alpha_pipeline: wgpu::RenderPipeline,
    multiply_pipeline: wgpu::RenderPipeline,
    blur_pipeline: wgpu::RenderPipeline,
    present_pipeline: wgpu::RenderPipeline,
}

/// Compiles the shader and builds one pipeline per blend mode, plus the blur and present
/// pipelines. Blend state and target format are baked into a wgpu pipeline, which is why
/// there are four rather than one with switchable state.
fn build_pipelines(gpu: &GpuDevice, surface_format: wgpu::TextureFormat) -> Pipelines {
    let shader = gpu.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("terralistic"),
        source: wgpu::ShaderSource::Wgsl(SHADER.into()),
    });

    let uniform_layout = gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("uniforms"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: true,
                min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<Uniforms>() as u64),
            },
            count: None,
        }],
    });

    let pipeline_layout = gpu.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("terralistic"),
        bind_group_layouts: &[Some(&uniform_layout), Some(&gpu.texture_bind_group_layout)],
        immediate_size: 0,
    });

    let make_pipeline = |label: &str, fragment_entry: &str, format: wgpu::TextureFormat, blend: Option<wgpu::BlendState>| {
        gpu.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: (VERTEX_FLOATS * 4) as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4, 2 => Float32x2],
                })],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some(fragment_entry),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        })
    };

    let offscreen = wgpu::TextureFormat::Rgba8Unorm;
    Pipelines {
        alpha_pipeline: make_pipeline("alpha", "fragment_main", offscreen, Some(blend_state(BlendMode::Alpha))),
        multiply_pipeline: make_pipeline("multiply", "fragment_main", offscreen, Some(blend_state(BlendMode::Multiply))),
        // The blur writes a finished pixel rather than compositing one.
        blur_pipeline: make_pipeline("blur", "fragment_blur", offscreen, None),
        present_pipeline: make_pipeline("present", "fragment_main", surface_format, None),
        uniform_layout,
    }
}

/// The four one pixel edges of a rectangle border, as rectangles.
///
/// They cover the rectangle's own footprint and nothing outside it, which is both what a UI
/// border should look like and what keeps the goldens machine-independent. That only holds
/// for a rectangle with room for an edge, which is why `Rect::render_outline` drops empty
/// ones before they get here.
///
/// The horizontal edges run corner to corner and the vertical ones do too, so **the four
/// corner pixels are drawn twice**. With an opaque colour that is idempotent; with a
/// translucent one - a `Button`'s border part way through its hover fade is the only place the
/// toolkit produces one - the corners blend twice and come out slightly darker than the rest
/// of the border. Four pixels for a fraction of a second, and the alternative is edges whose
/// extent depends on the rectangle being at least two pixels each way, so it is left alone.
pub(super) fn outline_edges(rect: gfx::Rect) -> [gfx::Rect; 4] {
    let gfx::Rect { pos, size } = rect;
    [
        gfx::Rect::new(pos, gfx::FloatSize(size.0, 1.0)),
        gfx::Rect::new(gfx::FloatPos(pos.0, pos.1 + size.1 - 1.0), gfx::FloatSize(size.0, 1.0)),
        gfx::Rect::new(pos, gfx::FloatSize(1.0, size.1)),
        gfx::Rect::new(gfx::FloatPos(pos.0 + size.0 - 1.0, pos.1), gfx::FloatSize(1.0, size.1)),
    ]
}

/// Maps texel coordinates onto the `[0,1]` range the sampler wants.
fn texel_scale(texture_size: gfx::FloatSize) -> Transformation {
    let mut result = Transformation::new();
    result.stretch((1.0 / texture_size.0, 1.0 / texture_size.1));
    result
}

/// The blend factors, which apply to the alpha channel as well as to colour.
///
/// That means a translucent draw lowers the framebuffer's alpha. Invisible on screen, because
/// the final blit ignores alpha, but it shows up in a capture and the goldens record it.
const fn blend_state(mode: BlendMode) -> wgpu::BlendState {
    let component = match mode {
        BlendMode::Alpha => wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::SrcAlpha,
            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
            operation: wgpu::BlendOperation::Add,
        },
        BlendMode::Multiply => wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::Dst,
            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
            operation: wgpu::BlendOperation::Add,
        },
    };
    wgpu::BlendState { color: component, alpha: component }
}

fn begin_pass<'encoder>(encoder: &'encoder mut wgpu::CommandEncoder, view: &wgpu::TextureView, clear: bool) -> wgpu::RenderPass<'encoder> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                // The game never clears: it draws an opaque background over the whole window
                // every frame. Only the golden-image harness asks for one.
                load: if clear { wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT) } else { wgpu::LoadOp::Load },
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    })
}

/// The unit quad every rectangle and texture draw is a transform of.
fn build_quad_buffer(gpu: &GpuDevice) -> wgpu::Buffer {
    let mut vertices: Vec<f32> = Vec::new();
    for (x, y) in [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)] {
        vertices.extend_from_slice(&[x, y, 1.0, 1.0, 1.0, 1.0, x, y]);
    }
    let buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("unit quad"),
        size: std::mem::size_of_val(vertices.as_slice()) as u64,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    gpu.queue.write_buffer(&buffer, 0, bytemuck::cast_slice(&vertices));
    buffer
}

/// A 1x1 white texture, bound whenever a draw has no texture of its own.
///
/// WGSL has no way to leave a binding empty, and the shader ignores the sample when
/// `has_texture` is zero, so what this contains never reaches the output. It lives for the
/// whole process, so it is built straight off the device rather than through the registry;
/// dropping the `wgpu::Texture` here is fine, since the view and the bind group hold their own
/// references to it.
fn build_white_bind_group(gpu: &GpuDevice) -> wgpu::BindGroup {
    let size = wgpu::Extent3d {
        width: 1,
        height: 1,
        depth_or_array_layers: 1,
    };
    let texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("white"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    gpu.queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &[255, 255, 255, 255],
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4),
            rows_per_image: Some(1),
        },
        size,
    );
    gpu.create_texture_bind_group(&texture.create_view(&wgpu::TextureViewDescriptor::default()))
}

fn build_uniform_bind_group(gpu: &GpuDevice, layout: &wgpu::BindGroupLayout, buffer: &wgpu::Buffer) -> wgpu::BindGroup {
    gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("uniforms"),
        layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer,
                offset: 0,
                size: wgpu::BufferSize::new(std::mem::size_of::<Uniforms>() as u64),
            }),
        }],
    })
}
