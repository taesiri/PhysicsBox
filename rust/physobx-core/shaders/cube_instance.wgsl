// Cube instance shader for Physobx
// Uses GPU instancing to render many cubes with a single draw call

// Camera uniform buffer
struct Camera {
    view_proj: mat4x4<f32>,
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
    eye_position: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

// Instance data stored in a storage buffer
struct Instance {
    position: vec3<f32>,
    _padding: f32,
    rotation: vec4<f32>,  // quaternion (x, y, z, w)
};

@group(0) @binding(1)
var<storage, read> instances: array<Instance>;

// Vertex input
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

// Vertex output / Fragment input
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) world_position: vec3<f32>,
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

    // Transform vertex position by instance rotation and translation
    let rotated_pos = quat_rotate(inst.rotation, vertex.position);
    let world_pos = rotated_pos + inst.position;

    // Transform normal by rotation only
    let world_normal = quat_rotate(inst.rotation, vertex.normal);

    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(world_pos, 1.0);
    out.world_normal = world_normal;
    out.world_position = world_pos;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Light direction (from upper right)
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 0.5));
    let normal = normalize(in.world_normal);

    // Simple diffuse lighting
    let diffuse = max(dot(normal, light_dir), 0.0);

    // Ambient light
    let ambient = 0.2;

    // Base cube color (orange-red)
    let base_color = vec3<f32>(0.85, 0.35, 0.2);

    // Final color
    let color = base_color * (ambient + diffuse * 0.8);

    return vec4<f32>(color, 1.0);
}
