// Ground plane shader with grid pattern

struct Camera {
    view_proj: mat4x4<f32>,
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
    eye_position: vec4<f32>,
};

struct GroundUniforms {
    ground_y: f32,
    ground_size: f32,
    grid_scale: f32,
    _padding: f32,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(0) @binding(1)
var<uniform> ground: GroundUniforms;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    // Generate quad from vertex index (0-5 for two triangles)
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
        vec2<f32>(-1.0,  1.0),
    );

    let pos = positions[vertex_index];
    let world_pos = vec3<f32>(
        pos.x * ground.ground_size,
        ground.ground_y,
        pos.y * ground.ground_size
    );

    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(world_pos, 1.0);
    out.world_position = world_pos;
    out.uv = pos * ground.ground_size;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Grid pattern
    let grid_size = ground.grid_scale;
    let grid_x = abs(fract(in.uv.x / grid_size + 0.5) - 0.5);
    let grid_z = abs(fract(in.uv.y / grid_size + 0.5) - 0.5);

    // Anti-aliased grid lines
    let line_width = 0.02;
    let aa = 0.01;

    let line_x = 1.0 - smoothstep(line_width - aa, line_width + aa, grid_x);
    let line_z = 1.0 - smoothstep(line_width - aa, line_width + aa, grid_z);
    let grid = max(line_x, line_z);

    // Base ground colors
    let ground_base = vec3<f32>(0.45, 0.48, 0.5);    // Gray concrete
    let grid_color = vec3<f32>(0.35, 0.38, 0.42);     // Darker grid lines

    // Distance fade for grid (fade out far away)
    let dist = length(in.world_position.xz - camera.eye_position.xz);
    let fade = 1.0 - smoothstep(20.0, 80.0, dist);

    // Mix base and grid
    let color = mix(ground_base, grid_color, grid * fade * 0.6);

    // Subtle gradient based on distance (atmospheric perspective)
    let fog_color = vec3<f32>(0.7, 0.8, 0.9);
    let fog_factor = smoothstep(30.0, 150.0, dist);
    let final_color = mix(color, fog_color, fog_factor * 0.5);

    return vec4<f32>(final_color, 1.0);
}
