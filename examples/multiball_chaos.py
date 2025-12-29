"""Multi-ball chaos - Many balls bombard a cube fortress from all directions."""

import physobx
import os
import math
from datetime import datetime

def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/multiball_chaos_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    scene = physobx.Scene()
    scene.add_ground(0.0, 150.0)

    # Build a cube fortress in the center
    # Outer walls
    wall_height = 6
    wall_length = 8

    # Four walls of the fortress
    for i in range(wall_length):
        for y in range(wall_height):
            # Front wall
            scene.add_cube([i * 1.05 - 3.5, y * 1.05 + 0.525, -4.0], 0.5, 1.0)
            # Back wall
            scene.add_cube([i * 1.05 - 3.5, y * 1.05 + 0.525, 4.0], 0.5, 1.0)
            # Left wall
            scene.add_cube([-4.0, y * 1.05 + 0.525, i * 1.05 - 3.5], 0.5, 1.0)
            # Right wall
            scene.add_cube([4.0, y * 1.05 + 0.525, i * 1.05 - 3.5], 0.5, 1.0)

    # Inner tower
    for y in range(10):
        for dx in range(2):
            for dz in range(2):
                scene.add_cube([dx * 1.05 - 0.525, y * 1.05 + 0.525, dz * 1.05 - 0.525], 0.5, 1.2)

    # Balls from multiple directions
    ball_speed = 35.0
    ball_radius = 0.9
    ball_mass = 8.0

    # Balls from 8 directions around the fortress
    num_balls_per_direction = 3
    for angle_idx in range(8):
        angle = angle_idx * math.pi / 4
        distance = 25.0

        for i in range(num_balls_per_direction):
            height = 2.0 + i * 2.5
            start_x = math.cos(angle) * distance
            start_z = math.sin(angle) * distance

            # Velocity pointing toward center with some variation
            vel_x = -math.cos(angle) * ball_speed
            vel_z = -math.sin(angle) * ball_speed
            vel_y = 3.0 - i  # Slightly different vertical components

            scene.add_sphere_with_velocity(
                [start_x, height, start_z],
                [vel_x, vel_y, vel_z],
                ball_radius,
                ball_mass
            )

    # Extra balls from above
    for i in range(5):
        angle = i * math.pi * 2 / 5
        x = math.cos(angle) * 3.0
        z = math.sin(angle) * 3.0
        scene.add_sphere_with_velocity(
            [x, 30.0, z],
            [0.0, -20.0, 0.0],
            1.2, 15.0
        )

    cubes, spheres = scene.shape_counts()
    print(f"Created fortress with {cubes} cubes and {spheres} attacking balls")

    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([35.0, 20.0, 35.0], [0.0, 5.0, 0.0])

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
