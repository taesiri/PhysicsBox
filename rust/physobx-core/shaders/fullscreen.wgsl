// Fullscreen quad shader for sky gradient background

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

// Fullscreen triangle trick - no vertex buffer needed
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;

    // Generate fullscreen triangle
    let x = f32(i32(vertex_index & 1u) * 4 - 1);
    let y = f32(i32(vertex_index >> 1u) * 4 - 1);

    out.position = vec4<f32>(x, y, 0.0, 1.0);
    out.uv = vec2<f32>((x + 1.0) * 0.5, (1.0 - y) * 0.5);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sky gradient from top to bottom
    let sky_top = vec3<f32>(0.4, 0.6, 0.9);      // Light blue
    let sky_horizon = vec3<f32>(0.7, 0.8, 0.95); // Pale blue/white
    let ground_color = vec3<f32>(0.35, 0.35, 0.4); // Dark gray

    let y = in.uv.y;

    // Sky gradient (top half)
    if (y < 0.5) {
        let t = y * 2.0; // 0 at top, 1 at horizon
        let sky = mix(sky_top, sky_horizon, t);
        return vec4<f32>(sky, 1.0);
    } else {
        // Ground gradient (bottom half) - subtle fade
        let t = (y - 0.5) * 2.0; // 0 at horizon, 1 at bottom
        let ground = mix(sky_horizon, ground_color, pow(t, 0.5));
        return vec4<f32>(ground, 1.0);
    }
}
