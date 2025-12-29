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

// Compute fake bevel factor based on distance to cube edges
fn compute_bevel(local_pos: vec3<f32>, half_extent: f32) -> f32 {
    // Distance from each axis to the edge (normalized 0-1)
    let edge_x = abs(local_pos.x) / half_extent;
    let edge_y = abs(local_pos.y) / half_extent;
    let edge_z = abs(local_pos.z) / half_extent;

    // Find the two closest edges (we're on a face, so one axis is ~1.0)
    // Sort to find the two largest (closest to edge)
    let sorted = vec3<f32>(
        max(max(edge_x, edge_y), edge_z),
        max(min(edge_x, edge_y), min(max(edge_x, edge_y), edge_z)),
        min(min(edge_x, edge_y), edge_z)
    );

    // Bevel width (in normalized coordinates)
    let bevel_width = 0.15;

    // Distance to corner (both edges close)
    let corner_dist = min(1.0 - sorted.y, 1.0 - sorted.z);
    let corner_factor = smoothstep(0.0, bevel_width, corner_dist);

    // Distance to edge (one edge close)
    let edge_dist = 1.0 - sorted.y;
    let edge_factor = smoothstep(0.0, bevel_width * 0.7, edge_dist);

    // Combine: corners are darker than edges
    return min(corner_factor, edge_factor);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let N = normalize(in.world_normal);
    let V = normalize(camera.eye_position.xyz - in.world_position);

    // Key light - from upper-left-front for good face contrast
    let key_dir = normalize(vec3<f32>(-0.5, 0.9, 0.6));
    // Fill light - softer from opposite side
    let fill_dir = normalize(vec3<f32>(0.7, 0.3, -0.4));
    // Rim light from behind
    let rim_dir = normalize(vec3<f32>(0.3, 0.2, -0.8));

    // Per-instance color
    let base_color = in.color;

    // Key light (warm)
    let key_diff = max(dot(N, key_dir), 0.0);
    let key_color = vec3<f32>(1.0, 0.95, 0.9);

    // Fill light (cool, much softer)
    let fill_diff = max(dot(N, fill_dir), 0.0);
    let fill_color = vec3<f32>(0.6, 0.7, 0.9);

    // Specular (GGX-like)
    let H = normalize(key_dir + V);
    let NdotH = max(dot(N, H), 0.0);
    let spec = pow(NdotH, 32.0) * 0.4;

    // === Sky IBL (hemisphere lighting) ===
    // Sky color from above, ground bounce from below
    let sky_color = vec3<f32>(0.4, 0.5, 0.7);    // Blue sky
    let ground_color = vec3<f32>(0.15, 0.12, 0.1); // Brown ground bounce
    let sky_amount = N.y * 0.5 + 0.5;  // Remap -1..1 to 0..1
    let ibl_diffuse = mix(ground_color, sky_color, sky_amount) * 0.15;

    // Ambient with IBL
    let ambient = vec3<f32>(0.06, 0.07, 0.09) + ibl_diffuse;

    // Combine lighting
    var color = base_color * ambient;
    color += base_color * key_color * key_diff * 0.85;
    color += base_color * fill_color * fill_diff * 0.25;
    color += key_color * spec;

    // Fresnel rim highlight
    let fresnel = pow(1.0 - max(dot(N, V), 0.0), 4.0) * 0.12;
    color += sky_color * fresnel;

    // === Fake Bevel ===
    // Assume half_extent of 0.5 (standard cube)
    let bevel = compute_bevel(in.local_position, 0.5);
    // Darken edges and corners to simulate chamfer
    let bevel_darken = mix(0.6, 1.0, bevel);
    // Add slight highlight at bevel edge
    let bevel_highlight = (1.0 - bevel) * 0.08 * max(dot(N, key_dir), 0.0);
    color = color * bevel_darken + vec3<f32>(bevel_highlight);

    // === Ambient Occlusion approximation ===
    // Darken bottom faces (ground contact)
    let ao_bottom = smoothstep(-0.3, 0.3, N.y) * 0.3 + 0.7;
    color *= ao_bottom;

    // Distance fog - minimal, only far horizon
    let dist = length(camera.eye_position.xyz - in.world_position);
    let fog_color = vec3<f32>(0.5, 0.55, 0.65);
    let fog_factor = smoothstep(400.0, 1000.0, dist);
    color = mix(color, fog_color, fog_factor * 0.05);

    return vec4<f32>(clamp(color, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}
