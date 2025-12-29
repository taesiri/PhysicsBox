# Physobx Rendering Improvement Roadmap

This document outlines the planned rendering improvements to achieve more realistic visuals while maintaining performance for large-scale physics simulations.

## Current State

- **Lighting**: Diffuse + specular with fresnel, hemisphere sky IBL
- **Shading**: Per-instance RGB color with fake edge bevels
- **Output**: HDR pipeline with ACES filmic tonemapping
- **Shadows**: 2048x2048 shadow map with PCF soft shadows (cubes, spheres, ground)
- **Ambient Occlusion**: None (planned)
- **Geometry**: Cubes (instanced) + UV spheres (instanced)
- **Ground**: Infinite plane with grid pattern and shadow receiving
- **Sky**: Procedural gradient sky renderer
- **Performance**: ~60+ fps at 1K cubes, ~30 fps at 5K cubes (1080p)

---

## Phase 1: HDR Pipeline + PBR Foundation

**Goal**: Establish physically-based rendering foundation with proper light math.

### 1.1 HDR Render Target ✅ DONE
- Render to `RGBA16Float` texture instead of `BGRA8Unorm`
- Allows light values > 1.0 (realistic light accumulation)
- **Files**: `gpu/render_target.rs`, `gpu/renderer.rs`

### 1.2 ACES Filmic Tonemapping ✅ DONE
- Add post-process pass to convert HDR → SDR
- ACES approximation for filmic look (no blown highlights)
- **Files**: `shaders/tonemap.wgsl`, `gpu/tonemap.rs`

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

### 1.3 PBR BRDF (GGX/Cook-Torrance) ❌ TODO
- Replace Blinn-Phong with physically correct specular
- GGX normal distribution + Smith geometry + Fresnel
- **Files**: `shaders/cube_instance.wgsl`, `shaders/sphere_instance.wgsl`

### 1.4 Per-Instance Material Properties ❌ TODO
- Add `roughness: f32` and `metallic: f32` to instance data
- Roughness controls specular spread (0 = mirror, 1 = diffuse)
- Metallic controls F0 reflectance (0 = dielectric, 1 = metal)
- **Files**: `gpu/instance_renderer.rs`, `scene/builder.rs`, Python bindings

### 1.5 Procedural Micro-Variation ❌ TODO
- Add subtle noise to roughness in shader
- Prevents perfectly uniform surfaces (CG tell)
- **Files**: Shader modifications

**Estimated Performance Impact**: Minimal (<5% slower)

---

## Phase 2: Ambient Occlusion + Fake Bevels

**Goal**: Ground objects in the scene and soften harsh CG edges.

### 2.1 Screen-Space Ambient Occlusion (SSAO) ❌ TODO
- Render depth + normals to G-buffer
- Sample hemisphere around each pixel
- Darken areas where geometry is close
- **Files**: New `shaders/ssao.wgsl`, `gpu/ssao.rs`

### 2.2 SSAO Blur Pass ❌ TODO
- Bilateral blur to remove noise while preserving edges
- **Files**: New `shaders/ssao_blur.wgsl`

### 2.3 Shader-Based Edge Softening ✅ DONE
- Detect edges using local_position in fragment shader
- Darken/lighten edges to simulate bevel without geometry
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

### 2.4 Simple Sky IBL (Diffuse Only) ✅ DONE
- Hemisphere ambient based on normal direction
- Sky color from above, ground bounce from below
- No cubemap needed - analytical approximation
- **Files**: `shaders/cube_instance.wgsl`, `shaders/sphere_instance.wgsl`

**Estimated Performance Impact**: ~15-25% slower (SSAO pass) - currently minimal without SSAO

---

## Phase 3: Shadows

**Goal**: Add directional light shadows for grounding and realism.

### 3.1 Shadow Map Rendering ✅ DONE
- Render scene depth from light's perspective
- Orthographic projection for directional light
- Resolution: 2048x2048
- **Files**: `gpu/shadow.rs`, `shaders/shadow_depth.wgsl`

### 3.2 PCF Soft Shadows ✅ DONE
- Percentage-closer filtering for soft edges
- 3x3 kernel sampling
- **Files**: `shaders/cube_instance.wgsl`, `shaders/sphere_instance.wgsl`, `shaders/ground.wgsl`

### 3.3 Cascaded Shadow Maps (Optional) ❌ TODO
- Multiple shadow maps at different distances
- Better quality near camera, acceptable far
- **Files**: Extended shadow system

### 3.4 Contact Shadows (Screen-Space) ❌ TODO
- Short-range ray march in screen space
- Catches small occlusions shadow maps miss
- **Files**: New `shaders/contact_shadows.wgsl`

**Estimated Performance Impact**: ~30-50% slower (current implementation)

---

## Phase 4: Post-Processing Effects

**Goal**: Screen-space effects for visual polish.

### 4.1 Bloom ❌ TODO
- Extract bright pixels (threshold)
- Gaussian blur bright areas
- Add back to final image
- **Files**: New `shaders/bloom.wgsl`, `gpu/bloom.rs`
- **Complexity**: Low
- **Performance**: ~5% slower

### 4.2 Anti-Aliasing (FXAA) ❌ TODO
- Fast approximate anti-aliasing
- Single post-process pass
- Smooths jagged edges without MSAA cost
- **Files**: New `shaders/fxaa.wgsl`, `gpu/fxaa.rs`
- **Complexity**: Low
- **Performance**: ~3% slower

### 4.3 Anti-Aliasing (MSAA) ❌ TODO
- Hardware multi-sample anti-aliasing
- 4x or 8x samples per pixel
- Higher quality than FXAA but more expensive
- **Files**: `gpu/render_target.rs` modifications
- **Complexity**: Low
- **Performance**: ~15-30% slower

### 4.4 Depth of Field (Optional) ❌ TODO
- Blur based on distance from focus point
- Cinematic effect
- **Files**: New `shaders/dof.wgsl`
- **Complexity**: Medium
- **Performance**: ~10% slower

---

## Phase 5: Advanced Materials

**Goal**: More material variety and realism.

### 5.1 Material System ❌ TODO
- Define material types (metal, plastic, concrete, wood)
- Pre-defined roughness/metallic/color presets
- Material ID per instance
- **Files**: `scene/materials.rs`, Python bindings
- **Complexity**: Medium

### 5.2 Normal Map Support ❌ TODO
- Texture atlas for surface detail
- Micro surface variation without geometry
- Requires UV coordinates
- **Files**: Shader modifications, texture loading
- **Complexity**: Medium

### 5.3 Emission + Bloom ❌ TODO
- Emissive materials for glowing objects
- Requires bloom post-process (Phase 4.1)
- **Files**: Instance data changes, shader modifications
- **Complexity**: Low (after bloom)

---

## Phase 6: Environment

**Goal**: Better scene context and atmosphere.

### 6.1 Skybox/Skydome ⚠️ PARTIAL
- Procedural sky gradient ✅ Done
- HDRI cubemap ❌ TODO
- **Files**: `gpu/sky_renderer.rs`, `shaders/sky.wgsl`
- **Complexity**: Medium (for HDRI)

### 6.2 HDRI-Based IBL ❌ TODO
- Load HDR environment maps
- Prefiltered specular + irradiance cubemaps
- Accurate reflections
- **Files**: New texture loading, IBL precomputation
- **Complexity**: High

### 6.3 Volumetric Fog (Optional) ❌ TODO
- Ray-marched atmospheric scattering
- God rays from directional light
- **Files**: New `shaders/volumetric.wgsl`
- **Complexity**: High
- **Performance**: ~20-40% slower

---

## Implementation Status

| Phase | Features | Status | Notes |
|-------|----------|--------|-------|
| **1** | HDR + Tonemap | ✅ Done | ACES tonemapping implemented |
| **1** | PBR Materials | ❌ Todo | Roughness/metallic not yet added |
| **2** | Fake Bevels + Sky IBL | ✅ Done | Edge darkening + hemisphere ambient |
| **2** | SSAO | ❌ Todo | Screen-space AO not implemented |
| **3** | Shadow Mapping | ✅ Done | 2048x2048 with PCF 3x3 |
| **3** | Cascaded/Contact Shadows | ❌ Todo | Optional improvements |
| **4** | Bloom | ❌ Todo | Post-process glow effect |
| **4** | Anti-Aliasing | ❌ Todo | FXAA or MSAA |
| **5** | Material System | ❌ Todo | Roughness/metallic presets |
| **6** | Environment/HDRI | ⚠️ Partial | Procedural sky done, HDRI todo |

---

## Next Priority Items (Recommended Order)

| Priority | Feature | Complexity | Impact |
|----------|---------|------------|--------|
| 1 | **Bloom** | Low | Medium - makes scenes pop |
| 2 | **FXAA** | Low | Medium - removes jaggies |
| 3 | **SSAO** | Medium | High - improves grounding |
| 4 | **PBR Materials** | Medium | High - material variety |

---

## Current Performance

| Scene Size | Render FPS (1080p) | Render FPS (4K) |
|------------|-------------------|-----------------|
| 1K cubes | ~60 fps | ~30 fps |
| 3K cubes | ~40 fps | ~15 fps |
| 5K cubes | ~30 fps | ~10 fps |
| 7K cubes | ~20 fps | ~7 fps |

---

## Future API Additions

### PBR Materials (When Implemented)

```python
# New material properties
scene.add_cube_pbr(
    position=[0, 1, 0],
    half_extent=0.5,
    mass=1.0,
    color=[0.8, 0.2, 0.2],
    roughness=0.5,      # 0.0 (mirror) to 1.0 (diffuse)
    metallic=0.0        # 0.0 (plastic) to 1.0 (metal)
)
```

### Renderer Settings (When Implemented)

```python
sim = physobx.Simulator(scene, width=1920, height=1080)
sim.set_exposure(1.0)        # Exposure multiplier
sim.set_ssao_enabled(True)   # Toggle SSAO
sim.set_bloom_enabled(True)  # Toggle bloom
```

---

## References

- [Learn OpenGL - PBR Theory](https://learnopengl.com/PBR/Theory)
- [Filmic Tonemapping](http://filmicworlds.com/blog/filmic-tonemapping-operators/)
- [SSAO Tutorial](https://learnopengl.com/Advanced-Lighting/SSAO)
- [GGX Specular BRDF](https://google.github.io/filament/Filament.html)
