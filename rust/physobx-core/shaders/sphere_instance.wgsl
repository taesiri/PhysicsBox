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

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let N = normalize(in.world_normal);
    let V = normalize(camera.eye_position.xyz - in.world_position);

    // Same key light as cubes for consistency
    let key_dir = normalize(vec3<f32>(-0.5, 0.9, 0.6));
    let fill_dir = normalize(vec3<f32>(0.7, 0.3, -0.4));

    // Material - bright metallic blue
    let base_color = vec3<f32>(0.35, 0.5, 0.75);

    // Key light diffuse
    let key_diff = max(dot(N, key_dir), 0.0);
    let key_color = vec3<f32>(1.0, 0.98, 0.95);

    // Fill light
    let fill_diff = max(dot(N, fill_dir), 0.0);
    let fill_color = vec3<f32>(0.7, 0.75, 0.9);

    // Strong specular for metallic look
    let H = normalize(key_dir + V);
    let spec = pow(max(dot(N, H), 0.0), 48.0) * 0.9;

    // Fresnel rim lighting
    let fresnel = pow(1.0 - max(dot(N, V), 0.0), 3.0) * 0.25;

    // Ambient
    let ambient = vec3<f32>(0.1, 0.12, 0.18);

    // Combine lighting
    var color = base_color * ambient;
    color += base_color * key_color * key_diff * 0.85;
    color += base_color * fill_color * fill_diff * 0.25;
    color += key_color * spec;
    color += vec3<f32>(0.6, 0.7, 0.9) * fresnel;

    // Sky reflection on top
    let sky_reflect = max(N.y, 0.0) * 0.12;
    color += vec3<f32>(0.5, 0.6, 0.85) * sky_reflect;

    // Distance fog
    let dist = length(camera.eye_position.xyz - in.world_position);
    let fog_color = vec3<f32>(0.65, 0.72, 0.85);
    let fog_factor = smoothstep(50.0, 200.0, dist);
    color = mix(color, fog_color, fog_factor * 0.35);

    return vec4<f32>(clamp(color, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}
