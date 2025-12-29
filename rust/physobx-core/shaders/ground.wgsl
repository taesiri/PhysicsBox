// Ground plane shader with grid pattern and shadow mapping

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

// Shadow map bindings (group 1)
struct ShadowUniforms {
    light_view_proj: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> shadow_uniforms: ShadowUniforms;

@group(1) @binding(1)
var shadow_map: texture_depth_2d;

@group(1) @binding(2)
var shadow_sampler: sampler_comparison;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) shadow_pos: vec4<f32>,
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

    // Transform world position to shadow map space
    out.shadow_pos = shadow_uniforms.light_view_proj * vec4<f32>(world_pos, 1.0);

    return out;
}

// PCF shadow sampling (3x3 kernel)
fn sample_shadow_pcf(shadow_pos: vec4<f32>) -> f32 {
    // Perspective divide to get NDC
    let proj_coords = shadow_pos.xyz / shadow_pos.w;

    // Transform from [-1,1] to [0,1] for UV coordinates
    let shadow_uv = proj_coords.xy * vec2<f32>(0.5, -0.5) + vec2<f32>(0.5, 0.5);

    // Check if outside shadow map bounds
    if (shadow_uv.x < 0.0 || shadow_uv.x > 1.0 || shadow_uv.y < 0.0 || shadow_uv.y > 1.0) {
        return 1.0; // Outside shadow map - fully lit
    }

    // Check if behind light
    if (proj_coords.z < 0.0 || proj_coords.z > 1.0) {
        return 1.0;
    }

    // Shadow map texel size (2048x2048)
    let texel_size = 1.0 / 2048.0;

    // PCF 3x3 sampling
    var shadow = 0.0;
    for (var x = -1; x <= 1; x++) {
        for (var y = -1; y <= 1; y++) {
            let offset = vec2<f32>(f32(x), f32(y)) * texel_size;
            shadow += textureSampleCompare(
                shadow_map,
                shadow_sampler,
                shadow_uv + offset,
                proj_coords.z - 0.001 // Small bias for ground
            );
        }
    }

    return shadow / 9.0;
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
    var color = mix(ground_base, grid_color, grid * fade * 0.6);

    // Sample shadow map
    let shadow = sample_shadow_pcf(in.shadow_pos);

    // Apply shadow to ground (darken shadowed areas)
    // Mix between shadowed and lit based on shadow value
    let shadow_darkness = 0.4; // How dark shadows are (0 = black, 1 = no shadow)
    let shadow_factor = mix(shadow_darkness, 1.0, shadow);
    color *= shadow_factor;

    // Subtle gradient based on distance (atmospheric perspective)
    let fog_color = vec3<f32>(0.5, 0.55, 0.65);  // Muted blue-gray
    let fog_factor = smoothstep(400.0, 1000.0, dist);  // Very far start
    let final_color = mix(color, fog_color, fog_factor * 0.05);  // 5% max

    return vec4<f32>(final_color, 1.0);
}
