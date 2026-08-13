// Shaders for the whole toolkit. There are only two: everything draws textured, tinted
// triangles, and the blur is the one effect with a kernel.

struct Uniforms {
    transform: mat3x3<f32>,
    texture_transform: mat3x3<f32>,
    color: vec4<f32>,
    // Blur only: sampling is clamped inside (limit.zw, limit.xy) so a pass cannot pull in
    // pixels from outside the region being blurred.
    limit: vec4<f32>,
    // Blur only: the spacing between the thirteen taps.
    blur_offset: vec2<f32>,
    has_texture: u32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(1) @binding(0) var source_texture: texture_2d<f32>;
@group(1) @binding(1) var source_sampler: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) texture_coord: vec2<f32>,
};

@vertex
fn vertex_main(
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) texture_coord: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>((uniforms.transform * vec3<f32>(position, 1.0)).xy, 0.0, 1.0);
    out.color = uniforms.color * color;
    out.texture_coord = (uniforms.texture_transform * vec3<f32>(texture_coord, 1.0)).xy;
    return out;
}

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sampled = textureSample(source_texture, source_sampler, in.texture_coord);
    let base = select(vec4<f32>(1.0, 1.0, 1.0, 1.0), sampled, uniforms.has_texture != 0u);
    return base * in.color;
}

// A thirteen tap gaussian along whichever axis `blur_offset` points down. Two passes make a
// separable 2D blur; the backend runs four or six of them.
const GAUSS = array<f32, 13>(
    0.01854, 0.034196, 0.056341, 0.083121, 0.109695, 0.129574, 0.13699,
    0.129574, 0.109695, 0.083121, 0.056341, 0.034196, 0.01854
);

@fragment
fn fragment_blur(in: VertexOutput) -> @location(0) vec4<f32> {
    // The alpha starts at 255 rather than at 0. That is not a unit value, so it saturates the
    // channel and the blurred region comes out opaque. A quirk, kept because the goldens
    // record what it produces.
    var color = vec4<f32>(0.0, 0.0, 0.0, 255.0);
    let low = vec2<f32>(uniforms.limit.z, uniforms.limit.w);
    let high = vec2<f32>(uniforms.limit.x, uniforms.limit.y);

    for (var i = 0; i < 13; i++) {
        let coord = max(min(in.texture_coord + (f32(i) - 6.0) * uniforms.blur_offset, high), low);
        color += textureSample(source_texture, source_sampler, coord) * GAUSS[i];
    }
    return color;
}
