// Sphere instance shader for Physobx
// Uses GPU instancing with Blinn-Phong lighting

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

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) color: vec3<f32>,
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

    return out;
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

    // Key light diffuse
    let key_diff = max(dot(N, key_dir), 0.0);
    let key_color = vec3<f32>(1.0, 0.98, 0.95);

    // Fill light
    let fill_diff = max(dot(N, fill_dir), 0.0);
    let fill_color = vec3<f32>(0.7, 0.75, 0.9);

    // Strong specular for metallic look (GGX-like)
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

    // Ambient with IBL
    let ambient = vec3<f32>(0.08, 0.09, 0.12) + ibl_diffuse;

    // Combine lighting
    var color = base_color * ambient;
    color += base_color * key_color * key_diff * 0.85;
    color += base_color * fill_color * fill_diff * 0.25;
    color += key_color * spec;
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
