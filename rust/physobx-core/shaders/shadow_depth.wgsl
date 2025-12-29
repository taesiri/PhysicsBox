// Shadow depth shader for Physobx
// Renders scene depth from light's perspective for shadow mapping

struct LightCamera {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> light_camera: LightCamera;

// Cube instance data (must match main shader)
struct CubeInstance {
    position: vec3<f32>,
    _padding: f32,
    rotation: vec4<f32>,  // quaternion (x, y, z, w)
    color: vec3<f32>,
    _padding2: f32,
};

// Sphere instance data
struct SphereInstance {
    position: vec3<f32>,
    radius: f32,
    rotation: vec4<f32>,
    color: vec3<f32>,
    _padding: f32,
};

@group(0) @binding(1)
var<storage, read> cube_instances: array<CubeInstance>;

@group(0) @binding(2)
var<storage, read> sphere_instances: array<SphereInstance>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

// Rotate a vector by a quaternion
fn quat_rotate(q: vec4<f32>, v: vec3<f32>) -> vec3<f32> {
    let qvec = q.xyz;
    let uv = cross(qvec, v);
    let uuv = cross(qvec, uv);
    return v + ((uv * q.w) + uuv) * 2.0;
}

// Vertex shader for cube shadow pass
@vertex
fn vs_cube(
    vertex: VertexInput,
    @builtin(instance_index) instance_id: u32,
) -> VertexOutput {
    let inst = cube_instances[instance_id];

    let rotated_pos = quat_rotate(inst.rotation, vertex.position);
    let world_pos = rotated_pos + inst.position;

    var out: VertexOutput;
    out.clip_position = light_camera.view_proj * vec4<f32>(world_pos, 1.0);
    return out;
}

// Vertex shader for sphere shadow pass
@vertex
fn vs_sphere(
    vertex: VertexInput,
    @builtin(instance_index) instance_id: u32,
) -> VertexOutput {
    let inst = sphere_instances[instance_id];

    // Scale unit sphere by radius and translate
    let world_pos = vertex.position * inst.radius + inst.position;

    var out: VertexOutput;
    out.clip_position = light_camera.view_proj * vec4<f32>(world_pos, 1.0);
    return out;
}

// No fragment shader needed - depth-only pass
// wgpu writes depth automatically without a fragment shader
