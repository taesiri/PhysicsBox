# Installation

This guide covers setting up Physobx on macOS with Metal GPU support.

## Requirements

- **macOS** 11.0+ (Big Sur or later)
- **Python** 3.10+
- **Rust** 1.70+ (for building from source)
- **uv** (recommended Python package manager)
- **ffmpeg** (for video encoding)

## Quick Install

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/physobx.git
cd physobx
```

### 2. Install uv (if not already installed)

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

### 3. Create Virtual Environment and Install

```bash
uv sync
```

### 4. Build the Rust Extension

```bash
uv run maturin develop --release
```

This compiles the Rust code and installs the Python module in development mode.

### 5. Verify Installation

```python
python -c "import physobx; print(physobx.version())"
```

You should see the version number (e.g., `0.1.0`).

## Dependencies

### Rust Dependencies (Cargo.toml)

The Rust core uses these main dependencies:

| Crate | Version | Purpose |
|-------|---------|---------|
| `rapier3d` | 0.23 | Physics simulation |
| `wgpu` | 23.0 | GPU rendering (Metal backend) |
| `nalgebra` | 0.33 | Linear algebra |
| `bytemuck` | 1.21 | Safe transmutes for GPU buffers |
| `image` | 0.25 | PNG encoding |
| `pyo3` | 0.23 | Python bindings |

### Python Dependencies

- `numpy` - Array handling
- `maturin` - Build system for PyO3

## Building for Release

For optimized performance:

```bash
uv run maturin develop --release
```

The `--release` flag enables compiler optimizations, which significantly improves both physics and rendering performance.

## Troubleshooting

### GPU Not Found

If you get GPU initialization errors:

1. Ensure you're on macOS with Metal support
2. Check that no other apps are using excessive GPU memory
3. Try reducing the render resolution

### Build Failures

If Rust compilation fails:

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
uv run maturin develop --release
```

### Import Errors

If Python can't find the module:

```bash
# Ensure you're in the virtual environment
source .venv/bin/activate

# Reinstall
uv run maturin develop --release
```

## Optional: Install ffmpeg

For converting rendered frames to video:

```bash
# macOS with Homebrew
brew install ffmpeg
```

Verify:

```bash
ffmpeg -version
```

## Development Setup

For active development on the Rust code:

```bash
# Watch for changes and rebuild
cargo watch -x build

# Run tests
cargo test

# Check formatting
cargo fmt --check
cargo clippy
```

## Next Steps

- [Scene Setup](scene-setup.md) - Learn how to create physics scenes
- [Sample Rendering](sample-rendering.md) - Render your first simulation
