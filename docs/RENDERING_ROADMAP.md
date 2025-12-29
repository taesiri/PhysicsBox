# Physobx Rendering Improvement Roadmap

This document outlines the planned rendering improvements to achieve more realistic visuals while maintaining performance for large-scale physics simulations.

## Current State (Baseline)

- **Lighting**: Basic Blinn-Phong with key + fill lights
- **Shading**: Per-instance RGB color only
- **Output**: Direct sRGB, no HDR pipeline
- **Shadows**: None
- **Ambient Occlusion**: None
- **Geometry**: Simple cubes (24 vertices, hard 90° edges)
- **Performance**: ~0.3 fps at 200K cubes, ~60+ fps at 1K cubes

---

## Phase 1: HDR Pipeline + PBR Foundation

**Goal**: Establish physically-based rendering foundation with proper light math.

### 1.1 HDR Render Target
- Render to `RGBA16Float` texture instead of `BGRA8Unorm`
- Allows light values > 1.0 (realistic light accumulation)
- **Files**: `gpu/render_target.rs`, `gpu/renderer.rs`

### 1.2 ACES Filmic Tonemapping
- Add post-process pass to convert HDR → SDR
- ACES approximation for filmic look (no blown highlights)
- **Files**: New `shaders/tonemap.wgsl`, `gpu/postprocess.rs`

```wgsl
// ACES approximation
fn aces_tonemap(x: vec3<f32>) -> vec3<f32> {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    return saturate((x * (a * x + b)) / (x * (c * x + d) + e));
}
```

### 1.3 PBR BRDF (GGX/Cook-Torrance)
- Replace Blinn-Phong with physically correct specular
- GGX normal distribution + Smith geometry + Fresnel
- **Files**: `shaders/cube_instance.wgsl`, `shaders/sphere_instance.wgsl`

### 1.4 Per-Instance Material Properties
- Add `roughness: f32` and `metallic: f32` to instance data
- Roughness controls specular spread (0 = mirror, 1 = diffuse)
- Metallic controls F0 reflectance (0 = dielectric, 1 = metal)
- **Files**: `gpu/instance_renderer.rs`, `scene/builder.rs`, Python bindings

### 1.5 Procedural Micro-Variation
- Add subtle noise to roughness in shader
- Prevents perfectly uniform surfaces (CG tell)
- **Files**: Shader modifications

**Estimated Performance Impact**: Minimal (<5% slower)

---

## Phase 2: Ambient Occlusion + Fake Bevels

**Goal**: Ground objects in the scene and soften harsh CG edges.

### 2.1 Screen-Space Ambient Occlusion (SSAO)
- Render depth + normals to G-buffer
- Sample hemisphere around each pixel
- Darken areas where geometry is close
- **Files**: New `shaders/ssao.wgsl`, `gpu/ssao.rs`

### 2.2 SSAO Blur Pass
- Bilateral blur to remove noise while preserving edges
- **Files**: New `shaders/ssao_blur.wgsl`

### 2.3 Shader-Based Edge Softening
- Detect edges using local_position in fragment shader
- Darken/lighten edges to simulate bevel without geometry
- Alternative: edge-based normal perturbation
- **Files**: `shaders/cube_instance.wgsl`

```wgsl
// Fake bevel darkening at edges
let edge_dist = min(min(
    abs(in.local_position.x) - 0.48,
    abs(in.local_position.y) - 0.48),
    abs(in.local_position.z) - 0.48
);
let edge_factor = smoothstep(0.0, 0.02, edge_dist);
color *= mix(0.7, 1.0, edge_factor);
```

### 2.4 Simple Sky IBL (Diffuse Only)
- Hemisphere ambient based on normal direction
- Sky color from above, ground bounce from below
- No cubemap needed - analytical approximation
- **Files**: Shader modifications

**Estimated Performance Impact**: ~15-25% slower (SSAO pass)

---

## Phase 3: Shadows (Optional, Performance Tradeoff)

**Goal**: Add directional light shadows for grounding and realism.

**Warning**: Significant performance impact. Recommend only for scenes <10K cubes.

### 3.1 Shadow Map Rendering
- Render scene depth from light's perspective
- Orthographic projection for directional light
- Resolution: 2048x2048 or 4096x4096
- **Files**: New `gpu/shadow.rs`, `shaders/shadow.wgsl`

### 3.2 PCF Soft Shadows
- Percentage-closer filtering for soft edges
- 3x3 or 5x5 kernel sampling
- **Files**: Shader modifications

### 3.3 Cascaded Shadow Maps (Optional)
- Multiple shadow maps at different distances
- Better quality near camera, acceptable far
- **Files**: Extended shadow system

### 3.4 Contact Shadows (Screen-Space)
- Short-range ray march in screen space
- Catches small occlusions shadow maps miss
- **Files**: New `shaders/contact_shadows.wgsl`

**Estimated Performance Impact**: 40-100% slower (doubles geometry passes)

---

## Phase 4: Advanced Materials (Future)

**Goal**: More material variety and realism.

### 4.1 Material System
- Define material types (metal, plastic, concrete, wood)
- Pre-defined roughness/metallic/color presets
- Material ID per instance

### 4.2 Normal Map Support
- Texture atlas for surface detail
- Micro surface variation without geometry
- Requires UV coordinates

### 4.3 Emission Support
- Emissive materials for glowing objects
- Bloom post-process for glow effect

---

## Phase 5: Environment (Future)

**Goal**: Better scene context and atmosphere.

### 5.1 Skybox/Skydome
- Procedural sky gradient or HDRI cubemap
- Visible background instead of solid color

### 5.2 HDRI-Based IBL
- Load HDR environment maps
- Prefiltered specular + irradiance cubemaps
- Accurate reflections

### 5.3 Volumetric Fog (Optional)
- Ray-marched atmospheric scattering
- God rays from directional light

---

## Implementation Priority

| Phase | Features | Impact | Risk |
|-------|----------|--------|------|
| **1** | HDR + Tonemap + PBR + Materials | High | Low |
| **2** | SSAO + Fake Bevels + Sky IBL | Medium | Low |
| **3** | Shadows | Medium | High (perf) |
| **4** | Material System + Normal Maps | Low | Medium |
| **5** | Environment + HDRI | Low | Medium |

---

## Performance Targets

| Scene Size | Phase 1 | Phase 2 | Phase 3 |
|------------|---------|---------|---------|
| 1K cubes | 60 fps | 50 fps | 30 fps |
| 10K cubes | 30 fps | 25 fps | 15 fps |
| 100K cubes | 3 fps | 2.5 fps | Not recommended |
| 200K cubes | 0.3 fps | 0.25 fps | Not recommended |

---

## API Changes (Phase 1)

### Python API Additions

```python
# New material properties
scene.add_cube_pbr(
    position=[0, 1, 0],
    half_extent=0.5,
    mass=1.0,
    color=[0.8, 0.2, 0.2],
    roughness=0.5,      # NEW: 0.0 (mirror) to 1.0 (diffuse)
    metallic=0.0        # NEW: 0.0 (plastic) to 1.0 (metal)
)

# Simulator settings
sim = physobx.Simulator(scene, width=1920, height=1080)
sim.set_tonemapping("aces")  # NEW: "none", "aces", "reinhard"
sim.set_exposure(1.0)        # NEW: exposure multiplier
```

---

## References

- [Learn OpenGL - PBR Theory](https://learnopengl.com/PBR/Theory)
- [Filmic Tonemapping](http://filmicworlds.com/blog/filmic-tonemapping-operators/)
- [SSAO Tutorial](https://learnopengl.com/Advanced-Lighting/SSAO)
- [GGX Specular BRDF](https://google.github.io/filament/Filament.html)
