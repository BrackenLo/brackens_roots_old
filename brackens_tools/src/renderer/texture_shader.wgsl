struct Projection {
    transform: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> projection: Projection;

struct VertexInput {
    // Vertex Data
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
    // Instance Data
    @location(2) transform_0: vec4<f32>,
    @location(3) transform_1: vec4<f32>,
    @location(4) transform_2: vec4<f32>,
    @location(5) transform_3: vec4<f32>,
    @location(6) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let transform = mat4x4<f32>(
        in.transform_0,
        in.transform_1,
        in.transform_2,
        in.transform_3
    );

    out.clip_pos =
        projection.transform *
        transform *
        vec4<f32>(in.vertex, 1.);

    out.tex_coord = in.tex_coord;
    out.color = in.color;

    return out;
}

@group(1) @binding(0) var texture: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    var color = textureSample(texture, texture_sampler, in.tex_coord);

    return mix(in.color, color);
    
}
