"""Wrecking balls - Heavy spheres demolish cube structures."""

import physobx
import os
import math
from datetime import datetime

def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/wrecking_balls_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    scene = physobx.Scene()
    scene.add_ground(0.0, 150.0)

    # Build three towers
    tower_positions = [[-8, 0, 0], [0, 0, 0], [8, 0, 0]]

    for tx, ty, tz in tower_positions:
        # Each tower is 3x3 base, 10 tall
        for y in range(10):
            for x in range(3):
                for z in range(3):
                    scene.add_cube([
                        tx + (x - 1) * 1.05,
                        y * 1.05 + 0.525,
                        tz + (z - 1) * 1.05
                    ], 0.5, 1.0)

    # Add connecting walls between towers
    for y in range(4):
        for x in range(4):
            # Wall between tower 1 and 2
            scene.add_cube([-4.0 + x * 1.05, y * 1.05 + 0.525, 0.0], 0.5, 1.0)
            # Wall between tower 2 and 3
            scene.add_cube([4.0 + x * 1.05, y * 1.05 + 0.525, 0.0], 0.5, 1.0)

    # Wrecking balls from different angles
    # Ball 1 - from front left, high
    scene.add_sphere_with_velocity(
        [-25.0, 12.0, 20.0],
        [35.0, -5.0, -25.0],
        1.5, 20.0
    )

    # Ball 2 - from front right
    scene.add_sphere_with_velocity(
        [25.0, 8.0, 18.0],
        [-35.0, 0.0, -22.0],
        1.5, 20.0
    )

    # Ball 3 - from above center
    scene.add_sphere_with_velocity(
        [0.0, 30.0, 0.0],
        [0.0, -15.0, 0.0],
        2.0, 30.0
    )

    cubes, spheres = scene.shape_counts()
    print(f"Created {cubes} cubes and {spheres} wrecking balls")

    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([30.0, 18.0, 35.0], [0.0, 5.0, 0.0])

    fps = 60
    duration = 10.0
    total_frames = int(fps * duration)

    print(f"Rendering {total_frames} frames ({duration}s at {fps}fps)...")

    for frame in range(total_frames):
        sim.step(1.0 / fps)

        filename = f"{output_dir}/frame_{frame:04d}.png"
        sim.save_png(filename)

        if frame % 60 == 0:
            print(f"  Frame {frame}/{total_frames}")

    print(f"Done! Frames saved to {output_dir}/")


if __name__ == "__main__":
    main()
