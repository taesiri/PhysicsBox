// Sphere instance shader for Physobx
// Uses GPU instancing with Blinn-Phong lighting and shadow mapping

struct Camera {
    view_proj: mat4x4<f32>,
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
    eye_position: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

struct Instance {
    position: vec3<f32>,
    radius: f32,
    rotation: vec4<f32>,  // quaternion (x, y, z, w) - unused for spheres but kept for consistency
    color: vec3<f32>,
    _padding: f32,
};

@group(0) @binding(1)
var<storage, read> instances: array<Instance>;

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

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) color: vec3<f32>,
    @location(3) shadow_pos: vec4<f32>,
};

@vertex
fn vs_main(
    vertex: VertexInput,
    @builtin(instance_index) instance_id: u32,
) -> VertexOutput {
    let inst = instances[instance_id];

    // Scale unit sphere by radius and translate
    let world_pos = vertex.position * inst.radius + inst.position;
    let world_normal = vertex.normal;  // Unit sphere normals don't need rotation

    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(world_pos, 1.0);
    out.world_normal = world_normal;
    out.world_position = world_pos;
    out.color = inst.color;

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
                proj_coords.z - 0.002 // Bias to reduce shadow acne
            );
        }
    }

    return shadow / 9.0;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let N = normalize(in.world_normal);
    let V = normalize(camera.eye_position.xyz - in.world_position);

    // Same key light as cubes for consistency
    let key_dir = normalize(vec3<f32>(-0.5, 0.9, 0.6));
    let fill_dir = normalize(vec3<f32>(0.7, 0.3, -0.4));

    // Per-instance color
    let base_color = in.color;

    // Sample shadow map
    let shadow = sample_shadow_pcf(in.shadow_pos);

    // Key light diffuse - affected by shadow
    let key_diff = max(dot(N, key_dir), 0.0);
    let key_color = vec3<f32>(1.0, 0.98, 0.95);

    // Fill light - not shadowed
    let fill_diff = max(dot(N, fill_dir), 0.0);
    let fill_color = vec3<f32>(0.7, 0.75, 0.9);

    // Strong specular for metallic look (GGX-like) - affected by shadow
    let H = normalize(key_dir + V);
    let NdotH = max(dot(N, H), 0.0);
    let spec = pow(NdotH, 64.0) * 1.0;

    // Fresnel rim lighting (stronger for spheres)
    let NdotV = max(dot(N, V), 0.0);
    let fresnel = pow(1.0 - NdotV, 4.0) * 0.3;

    // === Sky IBL (hemisphere lighting) ===
    let sky_color = vec3<f32>(0.4, 0.5, 0.7);
    let ground_color = vec3<f32>(0.15, 0.12, 0.1);
    let sky_amount = N.y * 0.5 + 0.5;
    let ibl_diffuse = mix(ground_color, sky_color, sky_amount) * 0.18;

    // Ambient with IBL (not shadowed)
    let ambient = vec3<f32>(0.08, 0.09, 0.12) + ibl_diffuse;

    // Combine lighting with shadows
    var color = base_color * ambient;
    color += base_color * key_color * key_diff * 0.85 * shadow;  // Key light shadowed
    color += base_color * fill_color * fill_diff * 0.25;         // Fill light not shadowed
    color += key_color * spec * shadow;                          // Specular shadowed
    color += sky_color * fresnel;

    // Environment reflection approximation
    let reflect_dir = reflect(-V, N);
    let env_reflect = mix(ground_color, sky_color * 1.2, reflect_dir.y * 0.5 + 0.5);
    color += env_reflect * fresnel * 0.5;

    // Distance fog - minimal, only far horizon
    let dist = length(camera.eye_position.xyz - in.world_position);
    let fog_color = vec3<f32>(0.5, 0.55, 0.65);
    let fog_factor = smoothstep(400.0, 1000.0, dist);
    color = mix(color, fog_color, fog_factor * 0.05);

    return vec4<f32>(clamp(color, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}
