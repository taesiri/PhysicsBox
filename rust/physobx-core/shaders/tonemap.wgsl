// Tonemapping post-process shader
// Converts HDR render to LDR output with ACES filmic curve

@group(0) @binding(0)
var hdr_texture: texture_2d<f32>;

@group(0) @binding(1)
var hdr_sampler: sampler;

struct TonemapParams {
    exposure: f32,
    _padding1: f32,
    _padding2: f32,
    _padding3: f32,
};

@group(0) @binding(2)
var<uniform> params: TonemapParams;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

// Fullscreen triangle vertex shader
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    // Generate fullscreen triangle from vertex index
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );

    var uvs = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, -1.0),
    );

    var out: VertexOutput;
    out.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    out.uv = uvs[vertex_index];

    return out;
}

// ACES filmic tonemapping approximation
// From Krzysztof Narkowicz: https://knarkowicz.wordpress.com/2016/01/06/aces-filmic-tone-mapping-curve/
fn aces_tonemap(x: vec3<f32>) -> vec3<f32> {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), vec3<f32>(0.0), vec3<f32>(1.0));
}

// Linear to sRGB gamma correction
fn linear_to_srgb(linear: vec3<f32>) -> vec3<f32> {
    let cutoff = linear < vec3<f32>(0.0031308);
    let higher = vec3<f32>(1.055) * pow(linear, vec3<f32>(1.0 / 2.4)) - vec3<f32>(0.055);
    let lower = linear * vec3<f32>(12.92);
    return select(higher, lower, cutoff);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample HDR color
    let hdr_color = textureSample(hdr_texture, hdr_sampler, in.uv).rgb;

    // Apply exposure
    let exposed = hdr_color * params.exposure;

    // Apply ACES tonemapping
    let tonemapped = aces_tonemap(exposed);

    // Note: Output format is Rgba8UnormSrgb, which does sRGB conversion automatically
    // So we output linear values and let the hardware handle gamma

    return vec4<f32>(tonemapped, 1.0);
}
