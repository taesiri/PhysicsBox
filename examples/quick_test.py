"""Quick test: Small simulation for fast verification."""

import physobx
import os
from datetime import datetime


def main():
    # Create timestamped output folder
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/quick_test_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    # Small scene for quick testing
    scene = physobx.Scene()
    scene.add_ground(0.0, 20.0)
    scene.add_cube_grid(
        center=[0.0, 8.0, 0.0],
        spacing=1.5,
        count=[5, 5, 5],  # 125 cubes
        half_extent=0.5,
        mass=1.0,
    )
    print(f"Created scene with {scene.body_count()} cubes")

    # Create simulator
    sim = physobx.Simulator(scene, width=1280, height=720)
    sim.set_camera([15.0, 12.0, 18.0], [0.0, 3.0, 0.0])

    # Short simulation
    fps = 30
    duration = 2.0
    total_frames = int(fps * duration)

    print(f"Rendering {total_frames} frames...")
    for frame in range(total_frames):
        for _ in range(2):
            sim.step(1.0 / 60.0)

        filename = f"{output_dir}/frame_{frame:04d}.png"
        sim.save_png(filename)

    print(f"Done! {total_frames} frames saved to {output_dir}/")


if __name__ == "__main__":
    main()
