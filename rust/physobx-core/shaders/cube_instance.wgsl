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
    color: vec3<f32>,
    _padding2: f32,
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
    @location(3) color: vec3<f32>,
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
    out.color = inst.color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let N = normalize(in.world_normal);
    let V = normalize(camera.eye_position.xyz - in.world_position);

    // Key light - from upper-left-front for good face contrast
    let key_dir = normalize(vec3<f32>(-0.5, 0.9, 0.6));
    // Fill light - softer from opposite side
    let fill_dir = normalize(vec3<f32>(0.7, 0.3, -0.4));

    // Per-instance color
    let base_color = in.color;

    // Key light (warm)
    let key_diff = max(dot(N, key_dir), 0.0);
    let key_color = vec3<f32>(1.0, 0.95, 0.9);

    // Fill light (cool, much softer)
    let fill_diff = max(dot(N, fill_dir), 0.0);
    let fill_color = vec3<f32>(0.6, 0.7, 0.9);

    // Specular
    let H = normalize(key_dir + V);
    let spec = pow(max(dot(N, H), 0.0), 24.0) * 0.35;

    // Low ambient to preserve contrast
    let ambient = vec3<f32>(0.08, 0.10, 0.14);

    // Combine lighting
    var color = base_color * ambient;
    color += base_color * key_color * key_diff * 0.9;
    color += base_color * fill_color * fill_diff * 0.2;
    color += key_color * spec;

    // Sky reflection on top faces
    let sky_reflect = max(N.y, 0.0) * 0.08;
    color += vec3<f32>(0.5, 0.6, 0.8) * sky_reflect;

    // Distance fog
    let dist = length(camera.eye_position.xyz - in.world_position);
    let fog_color = vec3<f32>(0.65, 0.72, 0.85);
    let fog_factor = smoothstep(50.0, 200.0, dist);
    color = mix(color, fog_color, fog_factor * 0.35);

    return vec4<f32>(clamp(color, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}
