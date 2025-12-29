"""Example: Pyramid of cubes tumbling down."""

import physobx
import os
from datetime import datetime


def main():
    # Create timestamped output folder
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/cube_pyramid_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    # Create scene with ground
    scene = physobx.Scene()
    scene.add_ground(0.0, 40.0)

    # Build a pyramid manually (layers decreasing in size)
    cube_size = 0.5
    spacing = 1.1
    base_size = 10

    for layer in range(base_size):
        size = base_size - layer
        y = layer * spacing + 5.0
        offset = layer * spacing / 2

        for x in range(size):
            for z in range(size):
                pos = [
                    (x - size/2 + 0.5) * spacing + offset,
                    y,
                    (z - size/2 + 0.5) * spacing + offset,
                ]
                scene.add_cube(pos, cube_size, 1.0)

    print(f"Created pyramid with {scene.body_count()} cubes")

    # Create simulator
    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([20.0, 15.0, 25.0], [0.0, 5.0, 0.0])

    # Simulate and render
    fps = 30
    duration = 4.0
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
