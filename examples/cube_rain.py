"""Example: Cubes raining down from above."""

import physobx
import os
from datetime import datetime


def main():
    # Create timestamped output folder
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/cube_rain_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    # Create scene: spread out cubes high up
    scene = physobx.Scene()
    scene.add_ground(0.0, 50.0)

    # Wide spread of cubes at different heights
    scene.add_cube_grid(
        center=[0.0, 30.0, 0.0],
        spacing=3.0,
        count=[12, 8, 12],  # 1152 cubes
        half_extent=0.4,
        mass=1.0,
    )
    print(f"Created rain with {scene.body_count()} cubes")

    # Create simulator
    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([45.0, 25.0, 50.0], [0.0, 8.0, 0.0])

    # Simulate and render
    fps = 30
    duration = 5.0
    total_frames = int(fps * duration)

    print(f"Rendering {total_frames} frames...")
    for frame in range(total_frames):
        for _ in range(2):
            sim.step(1.0 / 60.0)

        filename = f"{output_dir}/frame_{frame:04d}.png"
        sim.save_png(filename)

        if frame % 30 == 0:
            print(f"  Frame {frame}/{total_frames}")

    print(f"Done! Frames saved to {output_dir}/")


if __name__ == "__main__":
    main()
