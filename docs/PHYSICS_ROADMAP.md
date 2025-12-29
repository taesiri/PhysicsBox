# Physobx Physics & Performance Roadmap

This document outlines planned physics features, additional shapes, and performance optimizations.

## Current State

- **Physics Engine**: Rapier 3D v0.23
- **Shapes**: Cubes (box colliders), Spheres (ball colliders)
- **Ground**: Static plane collider
- **Collision Detection**: CCD enabled for fast bodies (>10 m/s)
- **Solver**: 8 iterations, substeps support via Python API
- **Performance**: ~80 fps physics at 10K bodies

---

## Phase 1: Additional Shapes

**Goal**: More geometry options for varied simulations.

### 1.1 Cylinders ❌ TODO
- Upright cylinders (cans, pillars)
- Rapier `Cylinder` collider
- New instance renderer for cylinder geometry
- **Files**: `scene/builder.rs`, new `gpu/cylinder_renderer.rs`
- **Complexity**: Medium
- **Python API**:
```python
scene.add_cylinder(position, radius, half_height, mass, color)
```

### 1.2 Capsules ❌ TODO
- Pill-shaped colliders (characters, rounded objects)
- Rapier `Capsule` collider
- New instance renderer
- **Files**: `scene/builder.rs`, new `gpu/capsule_renderer.rs`
- **Complexity**: Medium
- **Python API**:
```python
scene.add_capsule(position, radius, half_height, mass, color)
```

### 1.3 Cones ❌ TODO
- Cone-shaped colliders
- Rapier `Cone` collider
- **Files**: `scene/builder.rs`, new `gpu/cone_renderer.rs`
- **Complexity**: Medium

### 1.4 Convex Hulls ❌ TODO
- Arbitrary convex shapes from point clouds
- Rapier `ConvexHull` collider
- Load from mesh files or generate procedurally
- **Files**: New `scene/convex.rs`, mesh loading
- **Complexity**: High
- **Python API**:
```python
scene.add_convex_hull(position, points, mass, color)
```

### 1.5 Compound Shapes ❌ TODO
- Multiple colliders per body
- Build complex shapes from primitives
- **Files**: `scene/builder.rs` modifications
- **Complexity**: Medium
- **Python API**:
```python
scene.add_compound(position, shapes=[
    ("cube", [0, 0, 0], 0.5),
    ("sphere", [0, 1, 0], 0.3),
])
```

---

## Phase 2: Constraints & Joints

**Goal**: Connected bodies for complex structures.

### 2.1 Fixed Joints ❌ TODO
- Rigidly connect two bodies
- Rapier `FixedJoint`
- **Files**: New `physics/joints.rs`
- **Complexity**: Low
- **Python API**:
```python
scene.add_fixed_joint(body_a, body_b)
```

### 2.2 Ball Joints (Spherical) ❌ TODO
- Point-to-point constraint
- Bodies can rotate freely around anchor
- Rapier `SphericalJoint`
- **Complexity**: Low
- **Python API**:
```python
scene.add_ball_joint(body_a, body_b, anchor_a, anchor_b)
```

### 2.3 Hinge Joints (Revolute) ❌ TODO
- Single-axis rotation
- Doors, wheels, flaps
- Rapier `RevoluteJoint`
- Optional motor/limits
- **Complexity**: Medium
- **Python API**:
```python
scene.add_hinge_joint(body_a, body_b, anchor, axis, limits=None)
```

### 2.4 Slider Joints (Prismatic) ❌ TODO
- Single-axis translation
- Pistons, rails
- Rapier `PrismaticJoint`
- **Complexity**: Medium

### 2.5 Spring/Damper ❌ TODO
- Soft constraints with stiffness
- Suspension, bouncy connections
- **Complexity**: Medium
- **Python API**:
```python
scene.add_spring(body_a, body_b, stiffness, damping, rest_length)
```

### 2.6 Rope/Chain ❌ TODO
- Series of bodies connected by joints
- Helper function to create chains
- **Complexity**: Medium
- **Python API**:
```python
scene.add_rope(start, end, segments, mass_per_segment)
```

---

## Phase 3: Advanced Physics

**Goal**: More realistic simulations.

### 3.1 Continuous Collision Detection (CCD) ✅ DONE
- Prevents tunneling for fast objects
- Auto-enabled for bodies with velocity > 10 m/s
- **Files**: `physics/rapier_bridge.rs`

### 3.2 Physics Substeps ✅ DONE
- Multiple physics steps per render frame
- Improves stability for complex scenes
- **Python API**: `sim.step(dt, substeps=2)`

### 3.3 Increased Solver Iterations ✅ DONE
- 8 solver iterations (up from default 4)
- Better collision resolution
- **Files**: `physics/rapier_bridge.rs`

### 3.4 Material Properties ❌ TODO
- Per-body friction and restitution
- Currently hardcoded defaults
- **Python API**:
```python
scene.add_cube(..., friction=0.5, restitution=0.3)
```

### 3.5 Gravity Control ❌ TODO
- Custom gravity vector
- Zero-gravity, low-gravity scenes
- **Python API**:
```python
sim.set_gravity([0, -9.81, 0])
```

### 3.6 Body Sleeping ❌ TODO
- Disable simulation for stationary bodies
- Performance optimization
- Rapier handles this, may need tuning

---

## Phase 4: Performance & Scale

**Goal**: Handle 100K+ bodies efficiently.

### 4.1 Frustum Culling ❌ TODO
- Skip rendering off-screen bodies
- Camera frustum intersection test
- **Files**: `gpu/renderer.rs` modifications
- **Complexity**: Low
- **Performance**: 10-50% faster depending on view

### 4.2 Level of Detail (LOD) ❌ TODO
- Simpler geometry for distant objects
- Reduce cube vertices, sphere tessellation
- **Files**: Renderer modifications
- **Complexity**: Medium
- **Performance**: 20-40% faster for large scenes

### 4.3 GPU Spatial Hashing ❌ TODO
- Accelerate broad-phase collision
- Compute shader for spatial partitioning
- **Files**: New `gpu/spatial_hash.rs`, compute shaders
- **Complexity**: High
- **Performance**: Required for 100K+ bodies

### 4.4 Instance Buffer Streaming ❌ TODO
- Double-buffered GPU uploads
- Reduce CPU-GPU sync stalls
- **Files**: `gpu/instance_renderer.rs`
- **Complexity**: Medium
- **Performance**: ~10-20% faster

### 4.5 Parallel Physics Readback ❌ TODO
- Async copy physics state to GPU
- Overlap physics and rendering
- **Complexity**: High

---

## Phase 5: Soft Bodies & Particles (Future)

**Goal**: Deformable objects and effects.

### 5.1 Soft Bodies ❌ TODO
- Deformable meshes
- Rapier doesn't support natively
- Would need custom implementation or different library
- **Complexity**: Very High

### 5.2 Particle Systems ❌ TODO
- Dust, sparks, debris on collision
- GPU-accelerated particles
- **Files**: New `gpu/particles.rs`
- **Complexity**: High

### 5.3 Destruction/Fracture ❌ TODO
- Break objects into pieces on impact
- Pre-fractured meshes or runtime Voronoi
- **Complexity**: Very High

---

## Implementation Status

| Phase | Feature | Status | Notes |
|-------|---------|--------|-------|
| **1** | Cubes | ✅ Done | Box colliders + instanced rendering |
| **1** | Spheres | ✅ Done | Ball colliders + instanced rendering |
| **1** | Cylinders | ❌ Todo | Next priority |
| **1** | Capsules | ❌ Todo | After cylinders |
| **2** | Joints | ❌ Todo | Rapier supports, needs API |
| **3** | CCD | ✅ Done | Auto-enabled for fast bodies |
| **3** | Substeps | ✅ Done | Python API: step(dt, substeps) |
| **4** | Frustum Culling | ❌ Todo | Quick win for performance |
| **4** | GPU Spatial Hash | ❌ Todo | Required for 100K+ scale |

---

## Next Priority Items

| Priority | Feature | Complexity | Impact |
|----------|---------|------------|--------|
| 1 | **Cylinders** | Medium | New shape variety |
| 2 | **Capsules** | Medium | Rounded shapes |
| 3 | **Frustum Culling** | Low | Performance boost |
| 4 | **Basic Joints** | Low | Connected structures |

---

## Performance Targets

| Body Count | Current | With Culling | With GPU Hash |
|------------|---------|--------------|---------------|
| 1K | 400 fps | 400 fps | 400 fps |
| 10K | 80 fps | 100 fps | 150 fps |
| 50K | 15 fps | 25 fps | 80 fps |
| 100K | 5 fps | 10 fps | 50 fps |
| 500K | <1 fps | 2 fps | 20 fps |

---

## References

- [Rapier Physics Documentation](https://rapier.rs/docs/)
- [Rapier Joint Types](https://rapier.rs/docs/user_guides/rust/joints)
- [GPU Spatial Hashing](https://developer.nvidia.com/gpugems/gpugems3/part-v-physics-simulation/chapter-32-broad-phase-collision-detection-cuda)
