"""Domino destruction - A ball triggers a chain reaction through cube structures."""

import physobx
import os
import math
from datetime import datetime

def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/domino_destruction_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    scene = physobx.Scene()
    scene.add_ground(0.0, 150.0)

    # Create domino-like tall cube columns in a curved path
    num_dominoes = 25
    for i in range(num_dominoes):
        angle = i * 0.15
        radius = 8.0 + i * 0.3
        x = math.cos(angle) * radius
        z = math.sin(angle) * radius

        # Stack 4 cubes high for each domino
        for y in range(4):
            scene.add_cube([x, y * 1.05 + 0.525, z], 0.5, 0.8)

    # Create a cube pyramid at the end of the path
    pyramid_center = [
        math.cos(num_dominoes * 0.15) * (8.0 + num_dominoes * 0.3) + 5,
        0.0,
        math.sin(num_dominoes * 0.15) * (8.0 + num_dominoes * 0.3),
    ]

    # Pyramid layers
    for layer in range(6):
        size = 6 - layer
        y_pos = layer * 1.05 + 0.525
        offset = (size - 1) / 2.0
        for x in range(size):
            for z in range(size):
                pos = [
                    pyramid_center[0] + (x - offset) * 1.05,
                    y_pos,
                    pyramid_center[2] + (z - offset) * 1.05,
                ]
                scene.add_cube(pos, 0.5, 1.0)

    # Create a second structure - tall tower
    tower_pos = [-10.0, 0.0, 5.0]
    for y in range(12):
        for dx in range(2):
            for dz in range(2):
                scene.add_cube([
                    tower_pos[0] + dx * 1.05 - 0.525,
                    y * 1.05 + 0.525,
                    tower_pos[2] + dz * 1.05 - 0.525,
                ], 0.5, 1.0)

    # Launch ball to start the chain reaction
    scene.add_sphere_with_velocity(
        [-5.0, 2.0, -15.0],  # Start position
        [20.0, 5.0, 25.0],   # Velocity toward first domino
        1.2, 12.0
    )

    # Second ball aimed at the tower
    scene.add_sphere_with_velocity(
        [-25.0, 8.0, 5.0],
        [30.0, 0.0, 0.0],
        1.0, 10.0
    )

    cubes, spheres = scene.shape_counts()
    print(f"Created scene with {cubes} cubes and {spheres} balls")

    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([0.0, 25.0, 45.0], [5.0, 5.0, 0.0])

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
