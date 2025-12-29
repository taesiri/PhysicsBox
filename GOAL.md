# Rust physics sandbox ecosystem for million-body Metal simulation

Building a physics sandbox simulator with **100K-1M+ rigid bodies** on macOS presents a significant architectural challenge: **no off-the-shelf Rust physics library supports GPU/Metal-accelerated simulation**. The optimal approach combines CPU-parallel physics (Rapier) with custom GPU compute shaders (wgpu) for broad-phase collision, wgpu for Metal-based offscreen rendering, and PyO3 for zero-copy Python integration. This report details each component's maturity, performance characteristics, and suitability for your specific requirements.

## Physics engines face a CPU ceiling at scale

**Rapier** emerges as the clear choice for pure-Rust physics, delivering **5-10x better performance than its predecessor nphysics** and approaching CPU-version PhysX speeds. The engine (v0.25.0, actively maintained with 5k GitHub stars) provides rigid body dynamics, force-based joints, BVH-based collision detection, and island-based sleeping—critical for large scenes where inactive bodies have zero simulation cost.

However, Rapier's realistic limit sits at **50,000-100,000 simultaneously active bodies** using CPU parallelism via Rayon. The official documentation explicitly states it's "a great 100% Rust alternative to PhysX as long as you don't need a GPU-based solution." For your million-body target, most cubes must remain sleeping, or you must implement custom GPU physics.

| Physics Option | Max Active Bodies | GPU Support | Maturity |
|----------------|-------------------|-------------|----------|
| Rapier 0.25.0 | ~50-100K | ❌ None | ✅ Production-ready |
| physx-rs 0.19.0 | ~100K | ❌ CPU-only on macOS | ✅ Stable |
| rolt (Jolt bindings) | Unknown | ❌ None | 🔶 Early/incomplete |
| Custom wgpu compute | 1M+ | ✅ Metal | 🔨 Requires implementation |

**For 1M rigid bodies, the path forward requires hybrid architecture**: use Rapier for constraint solving and scene management while implementing GPU-accelerated broad-phase collision detection and position integration via wgpu compute shaders. Research papers demonstrate GPU broad-phase handling millions of bodies using spatial hashing, and your offline/batch context allows more solver iterations per frame than real-time applications.

## wgpu dominates Metal-based rendering options

The rendering ecosystem presents a clearer choice. **wgpu** (v28.0.0, December 2025) offers first-tier Metal backend support with mature compute shader capabilities, making it the definitive choice for macOS GPU work. The library provides safe Rust abstractions over Metal's full feature set, including storage buffers for instance data and compute pipelines for physics/culling operations.

**Offscreen 4K rendering** follows a well-documented pattern: create a texture with `RENDER_ATTACHMENT | COPY_SRC` usage, render to it, copy to a staging buffer via `copy_texture_to_buffer`, then map and read pixels. Each 4K RGBA frame consumes **~33MB**, so a 10-frame ring buffer for double-buffering requires approximately 330MB dedicated to frame extraction.

For rendering **1M identical cubes**, GPU instancing via storage buffers proves essential:

```rust
// Store all transforms in GPU storage buffer
// Vertex shader reads via instance_index
// Single draw call renders 1M instances
render_pass.draw_indexed(0..cube_indices, 0, 0..1_000_000);
```

Alternative renderers fall short: **metal-rs is deprecated** (migrating to objc2), **rend3 entered maintenance mode** in 2024, and **Kajiya requires Vulkan** hardware ray tracing unavailable on macOS. Bevy's rendering system, built atop wgpu, works well for headless rendering via its official `headless_renderer.rs` example but adds framework complexity unnecessary for batch processing.

## Python-Rust integration achieves zero-copy with PyO3

**PyO3 0.27.2** combined with **maturin** and **rust-numpy** delivers the recommended integration stack. The critical pattern for your 28MB-per-frame position/rotation data (1M objects × 7 floats × 4 bytes) involves zero-copy NumPy array access:

```rust
use numpy::{PyReadonlyArray2, PyArrayMethods};

#[pyfunction]
fn simulate_step(py: Python<'_>, positions: PyReadonlyArray2<f64>) -> PyResult<()> {
    py.detach(|| {  // Release GIL for parallel computation
        let arr = positions.as_array();  // Zero-copy view
        // Rayon-parallel physics update
    });
    Ok(())
}
```

**GIL management proves critical**: release the GIL via `py.detach()` during all CPU-intensive Rust computation to enable true parallelism. Polars demonstrates this pattern successfully, achieving 30x+ speedups over pandas by keeping data in Rust and exposing Python API views.

For scene definition, accept NumPy arrays directly from Python dataclasses rather than converting individual objects. Batch operations—transferring entire position/rotation arrays—minimize FFI overhead to microseconds rather than milliseconds. Maturin handles macOS wheel building for both x86_64 and aarch64 via `maturin build --target universal2-apple-darwin`.

## ECS may be overkill for homogeneous rigid bodies

For **1M identical cubes** in batch/offline processing, traditional ECS frameworks introduce unnecessary overhead. Your entities share identical components, require no dynamic component addition/removal, and process through a single dominant physics system—conditions where **simple SOA (Structure of Arrays) storage outperforms ECS** by 6x in benchmarks.

```rust
// Optimal for homogeneous rigid body batches
struct PhysicsWorld {
    positions: Vec<[f32; 3]>,      // Contiguous, SIMD-friendly
    rotations: Vec<[f32; 4]>,      // Cache-efficient iteration
    velocities: Vec<[f32; 3]>,     // Direct rayon parallelism
    angular_velocities: Vec<[f32; 3]>,
    masses: Vec<f32>,
}
```

If you anticipate future heterogeneity (varied entity types, complex relationships), **Bevy ECS** handles 1M+ entities well—recent optimizations reduced spawning 1M child entities from hours to ~111ms. Its archetype-based storage provides cache-friendly iteration, and `par_iter()` enables automatic parallel processing with configurable batch sizes (256-4096 recommended).

Both **specs** and **Legion** are now unmaintained and not recommended. **hecs** offers a lightweight alternative to Bevy ECS if you want archetype storage without framework overhead—Bevy ECS was originally forked from hecs.

For **scene serialization**, use **bincode** for 1M+ entities (~2962 MB/s encoding, ~60-80MB file size for 1M rigid bodies). RON format works for human-readable debugging but is too slow and verbose for production scene files.

## Video encoding works best through FFmpeg bindings

**ffmpeg-next** (v8.0.0) provides the most complete encoding solution, wrapping FFmpeg's full capabilities including **VideoToolbox hardware acceleration** via `h264_videotoolbox` and `hevc_videotoolbox` encoders. This matters significantly for 4K encoding where software x264 achieves only 10-30fps while hardware encoding maintains real-time performance.

The recommended workflow pipes rendered RGBA frames through a bounded channel to a separate encoder thread:

```rust
// 10-frame buffer provides backpressure (~330MB for 4K)
let (tx, rx) = mpsc::sync_channel::<Vec<u8>>(10);

// Encoder thread converts RGBA→YUV420p and encodes
// Simulation thread renders frames and sends to channel
```

For social media optimization, export at **45 Mbps H.264** (YouTube 4K quality) in MP4 container with faststart atom for streaming. Platforms re-encode for their specific requirements, so one high-quality master serves all destinations.

| Platform | Resolution | Bitrate | Notes |
|----------|-----------|---------|-------|
| YouTube | 4K/30fps | 35-45 Mbps | Primary target |
| Twitter | 4K/30fps | 5-15 Mbps | 512MB file limit |
| Instagram | 1080p | 3.5-5 Mbps | Requires downscale |

**video-rs** offers a simpler API for prototyping but provides less control. For maximum reliability, implement a libx264 software fallback—VideoToolbox encoding shows ~50% failure rates on certain Mac configurations according to RustDesk project research. An alternative approach renders to image sequences and invokes external FFmpeg via subprocess, trading disk I/O for simplified integration.

## Reference projects illuminate architectural patterns

**Particular** (github.com/Canleskis/particular) demonstrates the gold standard for large-scale Rust simulation: N-body physics with Barnes-Hut algorithm for O(N log N) scaling, rayon parallelism, and wgpu GPU acceleration via trait-based design. Its iterator-based acceleration computation pattern translates well to rigid body simulation.

The **200M particle simulation** achieved 180M particles at ~9fps on Apple M1 using explicit SIMD (`std::simd` with f32x4) and rayon multi-threading—proving Rust's performance ceiling exceeds your requirements given sufficient optimization. The author found Rust 4-5x faster than JavaScript at 20M particles while using 1/4 the memory.

**Sandspiel** (Rust WASM falling sand) established a crucial pattern: keep "everything low-level or CPU intensive" in Rust while handling UI in the host language. The developer later regretted spending performance headroom on resolution increases, leaving slower devices unable to maintain 60fps—a cautionary tale for your offline context where batch processing removes real-time constraints.

For Python-Rust scientific computing, **Polars** exemplifies the architecture: pure Rust core with zero-copy Arrow data layout, PyO3 wrapper exposing operations, and custom allocators (JeMalloc/Mimalloc) providing ~25% runtime improvement.

## Recommended architecture integrates all components

```
┌─────────────────────────────────────────────────┐
│              Python Frontend (PyO3)              │
│  • Scene definition via NumPy arrays            │
│  • Simulation orchestration and configuration   │
│  • Result analysis with pandas/polars           │
└──────────────────────┬──────────────────────────┘
                       │ Zero-copy array transfer
┌──────────────────────▼──────────────────────────┐
│                   Rust Core                      │
│  ┌────────────────────────────────────────────┐ │
│  │  SOA Physics Storage (positions, rotations)│ │
│  │  • rayon parallel iteration                 │ │
│  │  • SIMD-friendly contiguous arrays          │ │
│  └────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────┐ │
│  │  wgpu Compute (Metal backend)              │ │
│  │  • GPU broad-phase collision               │ │
│  │  • Spatial hashing for 1M bodies           │ │
│  └────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────┐ │
│  │  Rapier (CPU constraint solving)           │ │
│  │  • Narrow-phase collision                  │ │
│  │  • Joint constraints and sleeping          │ │
│  └────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────┐ │
│  │  wgpu Renderer (Metal, headless)           │ │
│  │  • GPU instancing for 1M cubes             │ │
│  │  • Offscreen 4K texture rendering          │ │
│  └────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────┐ │
│  │  ffmpeg-next Encoder                       │ │
│  │  • VideoToolbox H.264 acceleration         │ │
│  │  • Bounded frame queue with backpressure   │ │
│  └────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘
```

**Critical implementation decisions:**

- **Physics scaling**: Accept that active body count likely caps at ~100K without custom GPU physics. Leverage island-based sleeping aggressively—your cube-drop scenario likely has most bodies inactive after settling.

- **GPU compute**: Implement spatial hashing in WGSL compute shaders for broad-phase collision on 1M bodies. This provides the path to true million-body simulation while Rapier handles complex constraint solving.

- **Memory budget**: ~56MB for rigid body data (1M × 56 bytes), ~330MB for 4K frame buffers (10-frame ring), ~100MB ECS overhead if used. Total working memory under 500MB is achievable.

- **Offline advantage**: Without real-time constraints, use higher solver iteration counts for stability, implement adaptive timesteps based on velocity magnitudes, and render frames asynchronously from simulation.

## Conclusion

The Rust ecosystem provides mature components for most requirements but lacks turnkey million-body GPU physics. Success requires architectural pragmatism: **wgpu** for all GPU work (compute and rendering), **Rapier** for physics with realistic scaling expectations, **PyO3+rust-numpy** for efficient Python interop, and **ffmpeg-next** with VideoToolbox for 4K encoding. The hybrid approach—CPU constraint solving combined with GPU broad-phase and rendering—represents the most viable path to your million-body target on macOS Metal.

Key repositories to study: `dimforge/rapier` for physics patterns, `Canleskis/particular` for GPU-accelerated simulation architecture, `pola-rs/polars` for Python-Rust data pipeline design, and Bevy's `headless_renderer.rs` for offscreen rendering workflows.