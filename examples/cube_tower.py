"""Example: Tall tower of cubes collapsing."""

import physobx
import os
from datetime import datetime


def main():
    # Create timestamped output folder
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/cube_tower_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    # Create scene: tall tower of cubes
    scene = physobx.Scene()
    scene.add_ground(0.0, 30.0)

    # Stack cubes in a tall tower (5x5 base, 20 high)
    scene.add_cube_grid(
        center=[0.0, 20.0, 0.0],
        spacing=1.2,
        count=[5, 20, 5],  # 500 cubes
        half_extent=0.5,
        mass=1.0,
    )
    print(f"Created tower with {scene.body_count()} cubes")

    # Create simulator
    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([25.0, 20.0, 35.0], [0.0, 10.0, 0.0])

    # Simulate and render
    fps = 30
    duration = 4.0
    total_frames = int(fps * duration)

    print(f"Rendering {total_frames} frames...")
    for frame in range(total_frames):
        # Physics at 60Hz
        for _ in range(2):
            sim.step(1.0 / 60.0)

        # Save frame
        filename = f"{output_dir}/frame_{frame:04d}.png"
        sim.save_png(filename)

        if frame % 30 == 0:
            print(f"  Frame {frame}/{total_frames}")

    print(f"Done! Frames saved to {output_dir}/")


if __name__ == "__main__":
    main()
