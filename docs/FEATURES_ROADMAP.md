# Physobx Features & Workflow Roadmap

This document outlines planned workflow improvements, camera features, and API enhancements.

## Current State

- **Rendering**: Headless GPU rendering (no window)
- **Output**: PNG frames, external ffmpeg for video
- **Camera**: Static position/target, manual setup
- **Resolution**: Any resolution supported (tested up to 4K)
- **Python API**: Scene building, simulation, frame export

---

## Phase 1: Camera System

**Goal**: Flexible camera control for cinematic renders.

### 1.1 Camera Presets ❌ TODO
- Orbit camera (auto-rotate around target)
- Top-down view
- Side view
- Isometric view
- **Files**: `gpu/camera.rs`, Python bindings
- **Complexity**: Low
- **Python API**:
```python
sim.set_camera_preset("orbit", center=[0, 10, 0], distance=50)
sim.set_camera_preset("top_down", height=100)
sim.set_camera_preset("isometric")
```

### 1.2 Camera Animation ❌ TODO
- Smooth interpolation between keyframes
- Orbit animation (continuous rotation)
- Dolly/zoom animation
- **Files**: New `gpu/camera_animation.rs`
- **Complexity**: Medium
- **Python API**:
```python
sim.add_camera_keyframe(time=0.0, eye=[50, 30, 50], target=[0, 10, 0])
sim.add_camera_keyframe(time=5.0, eye=[50, 30, -50], target=[0, 10, 0])
sim.set_camera_interpolation("smooth")  # or "linear"
```

### 1.3 Camera Paths ❌ TODO
- Bezier/spline camera paths
- Import from external tools
- **Files**: New `gpu/camera_path.rs`
- **Complexity**: Medium
- **Python API**:
```python
sim.set_camera_path([
    ([50, 30, 50], [0, 10, 0]),
    ([0, 50, 50], [0, 10, 0]),
    ([-50, 30, 50], [0, 10, 0]),
])
```

### 1.4 Camera Shake ❌ TODO
- Procedural shake on impacts
- Adds cinematic feel
- **Complexity**: Low
- **Python API**:
```python
sim.enable_camera_shake(intensity=0.5, decay=0.95)
```

### 1.5 Focus Following ❌ TODO
- Camera tracks specific body or center of mass
- **Complexity**: Medium
- **Python API**:
```python
sim.set_camera_follow(body_index=0, offset=[10, 5, 10])
sim.set_camera_follow_center_of_mass()
```

---

## Phase 2: Video & Export

**Goal**: Streamlined video output without external tools.

### 2.1 Native Video Encoding ❌ TODO
- Integrate ffmpeg-next crate
- Direct MP4/H.264 output
- No intermediate PNG files needed
- **Files**: New `video/encoder.rs`
- **Complexity**: Medium
- **Dependencies**: `ffmpeg-next` crate
- **Python API**:
```python
sim.start_recording("output.mp4", fps=60, quality="high")
for _ in range(600):
    sim.step(1/60)
    sim.record_frame()
sim.finish_recording()
```

### 2.2 Hardware Video Encoding ❌ TODO
- VideoToolbox on macOS (Apple Silicon)
- Much faster than software encoding
- **Complexity**: High
- **Platform**: macOS only

### 2.3 GIF Export ❌ TODO
- Direct GIF output for previews
- Lower quality but universal
- **Files**: New `video/gif.rs`
- **Complexity**: Low
- **Dependencies**: `gif` crate

### 2.4 Image Sequence Options ❌ TODO
- JPEG output (smaller files)
- EXR output (HDR, for compositing)
- **Complexity**: Low
- **Python API**:
```python
sim.save_frame("frame.jpg", quality=90)
sim.save_frame("frame.exr")  # HDR
```

---

## Phase 3: Scene Building

**Goal**: Easier scene construction.

### 3.1 Scene Presets ❌ TODO
- Pre-built destruction scenes
- Tower, wall, pyramid, city templates
- **Files**: New `scene/presets.rs`, Python presets
- **Complexity**: Low
- **Python API**:
```python
scene = physobx.Scene.from_preset("tower", height=50, width=5)
scene = physobx.Scene.from_preset("city", buildings=10)
scene = physobx.Scene.from_preset("domino_spiral", count=200)
```

### 3.2 Procedural Generation ❌ TODO
- Noise-based terrain
- Random city generation
- Fractal structures
- **Complexity**: Medium
- **Python API**:
```python
scene.add_procedural_terrain(size=100, height_scale=10, seed=42)
scene.add_random_city(bounds=[-50, 50], buildings=20, seed=42)
```

### 3.3 Import from Files ❌ TODO
- Load scenes from JSON/TOML
- Import OBJ meshes as convex hulls
- **Files**: New `scene/loader.rs`
- **Complexity**: Medium
- **Python API**:
```python
scene = physobx.Scene.load("scene.json")
scene.add_mesh("building.obj", position=[0, 0, 0], mass=100)
```

### 3.4 Scene Serialization ❌ TODO
- Save/load simulation state
- Resume from checkpoint
- **Complexity**: Medium
- **Python API**:
```python
sim.save_state("checkpoint.bin")
sim.load_state("checkpoint.bin")
```

---

## Phase 4: Live Preview

**Goal**: Real-time interactive window.

### 4.1 Preview Window ❌ TODO
- Open window for live viewing
- Not headless, uses winit
- **Files**: New `window/` module
- **Complexity**: High
- **Dependencies**: `winit` crate
- **Python API**:
```python
sim = physobx.Simulator(scene, preview=True)
sim.run_interactive()  # Blocking, opens window
```

### 4.2 Interactive Camera ❌ TODO
- Mouse orbit/pan/zoom
- Keyboard shortcuts
- **Complexity**: Medium (requires window)

### 4.3 Real-time Parameter Adjustment ❌ TODO
- Adjust gravity, timestep live
- ImGui or egui integration
- **Complexity**: High

### 4.4 Pause/Step/Rewind ❌ TODO
- Interactive simulation control
- Step frame-by-frame
- Rewind (requires state history)
- **Complexity**: High

---

## Phase 5: Python API Improvements

**Goal**: More Pythonic, feature-rich API.

### 5.1 Zero-Copy Arrays ❌ TODO
- Direct memory sharing with NumPy
- Avoid copying position/rotation data
- **Files**: `physobx-python/src/lib.rs`
- **Complexity**: Medium
- **Performance**: Significant for large scenes

### 5.2 Context Managers ❌ TODO
- Pythonic resource management
- **Python API**:
```python
with physobx.Simulator(scene) as sim:
    for _ in range(100):
        sim.step(1/60)
```

### 5.3 Async Support ❌ TODO
- Async simulation stepping
- Background rendering
- **Complexity**: High
- **Python API**:
```python
async def render():
    async for frame in sim.render_async(frames=600):
        # Process frame
        pass
```

### 5.4 Progress Callbacks ❌ TODO
- Callback during long renders
- Progress bars, ETA
- **Complexity**: Low
- **Python API**:
```python
def on_progress(frame, total):
    print(f"Frame {frame}/{total}")

sim.render_frames(600, callback=on_progress)
```

### 5.5 Type Stubs ❌ TODO
- `.pyi` files for IDE autocomplete
- Better developer experience
- **Files**: `python/physobx/*.pyi`
- **Complexity**: Low

---

## Implementation Status

| Phase | Feature | Status | Notes |
|-------|---------|--------|-------|
| **1** | Static Camera | ✅ Done | set_camera(eye, target) |
| **1** | Camera Presets | ❌ Todo | Quick win |
| **1** | Camera Animation | ❌ Todo | High impact for videos |
| **2** | PNG Export | ✅ Done | save_png() |
| **2** | Native Video | ❌ Todo | Eliminate ffmpeg dependency |
| **3** | Manual Scene Building | ✅ Done | Full API |
| **3** | Scene Presets | ❌ Todo | Quick win |
| **4** | Live Preview | ❌ Todo | Major feature |
| **5** | Basic NumPy | ✅ Done | get_positions(), etc. |
| **5** | Zero-Copy | ❌ Todo | Performance improvement |

---

## Next Priority Items

| Priority | Feature | Complexity | Impact |
|----------|---------|------------|--------|
| 1 | **Camera Presets** | Low | Quick win |
| 2 | **Camera Animation** | Medium | Better videos |
| 3 | **Scene Presets** | Low | Easier demos |
| 4 | **Native Video Encoding** | Medium | No ffmpeg needed |
| 5 | **Progress Callbacks** | Low | Better UX |

---

## Dependencies for Future Features

| Feature | New Crates |
|---------|------------|
| Native Video | `ffmpeg-next` |
| GIF Export | `gif` |
| Live Preview | `winit`, `raw-window-handle` |
| GUI | `egui` |
| Mesh Import | `tobj` or `gltf` |
