// Cube instance shader for Physobx
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
    _padding: f32,
    rotation: vec4<f32>,  // quaternion (x, y, z, w)
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
    @location(2) local_position: vec3<f32>,
};

// Rotate a vector by a quaternion
fn quat_rotate(q: vec4<f32>, v: vec3<f32>) -> vec3<f32> {
    let qvec = q.xyz;
    let uv = cross(qvec, v);
    let uuv = cross(qvec, uv);
    return v + ((uv * q.w) + uuv) * 2.0;
}

@vertex
fn vs_main(
    vertex: VertexInput,
    @builtin(instance_index) instance_id: u32,
) -> VertexOutput {
    let inst = instances[instance_id];

    let rotated_pos = quat_rotate(inst.rotation, vertex.position);
    let world_pos = rotated_pos + inst.position;
    let world_normal = quat_rotate(inst.rotation, vertex.normal);

    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(world_pos, 1.0);
    out.world_normal = world_normal;
    out.world_position = world_pos;
    out.local_position = vertex.position;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Normalize vectors
    let N = normalize(in.world_normal);
    let V = normalize(camera.eye_position.xyz - in.world_position);

    // Three-point lighting setup (sun from upper-right-front)
    let key_light_dir = normalize(vec3<f32>(0.5, 0.8, 0.3));
    let fill_light_dir = normalize(vec3<f32>(-0.7, 0.2, -0.4));
    let back_light_dir = normalize(vec3<f32>(0.1, 0.3, -0.9));

    // Material properties - rich orange/terracotta in LINEAR space
    // (sRGB target will convert to gamma, so we work in linear)
    let base_color = vec3<f32>(0.72, 0.12, 0.02);  // Linear orange
    let shininess = 48.0;

    // Key light (warm sunlight)
    let key_diff = max(dot(N, key_light_dir), 0.0);
    let key_H = normalize(key_light_dir + V);
    let key_spec = pow(max(dot(N, key_H), 0.0), shininess) * 0.6;
    let key_intensity = 1.2;
    let key_color = vec3<f32>(1.0, 0.95, 0.85);

    // Fill light (cool sky bounce)
    let fill_diff = max(dot(N, fill_light_dir), 0.0);
    let fill_intensity = 0.35;
    let fill_color = vec3<f32>(0.5, 0.6, 0.85);

    // Back/rim light for edge pop
    let back_diff = max(dot(N, back_light_dir), 0.0);
    let back_intensity = 0.2;

    // Fresnel rim (subtle blue-ish)
    let fresnel = pow(1.0 - max(dot(N, V), 0.0), 4.0) * 0.12;

    // Ambient (low, to preserve contrast)
    let ambient = vec3<f32>(0.08, 0.10, 0.14);

    // Combine lighting
    var color = base_color * ambient;
    color += base_color * key_color * key_diff * key_intensity;
    color += base_color * fill_color * fill_diff * fill_intensity;
    color += base_color * back_diff * back_intensity;
    color += key_color * key_spec;
    color += vec3<f32>(0.6, 0.7, 0.9) * fresnel;

    // Edge darkening (fake AO)
    let local_min = min(min(abs(in.local_position.x), abs(in.local_position.y)), abs(in.local_position.z));
    let edge_ao = smoothstep(0.0, 0.12, local_min);
    color *= mix(0.5, 1.0, edge_ao);

    // Atmospheric fog (subtle)
    let dist = length(camera.eye_position.xyz - in.world_position);
    let fog_color = vec3<f32>(0.55, 0.65, 0.8);  // Linear sky blue
    let fog_factor = smoothstep(30.0, 100.0, dist);
    color = mix(color, fog_color, fog_factor * 0.35);

    // Clamp (sRGB target handles gamma)
    color = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));

    return vec4<f32>(color, 1.0);
}
