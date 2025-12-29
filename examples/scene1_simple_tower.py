"""Scene 1: Simple Tower - 100 cubes in a colorful tower attacked by 3 balls."""

import physobx
import os
import math
from datetime import datetime

def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/scene1_simple_tower_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    scene = physobx.Scene()
    scene.add_ground(0.0, 100.0)

    # Color palette - rainbow gradient
    colors = [
        [0.9, 0.2, 0.2],   # Red
        [0.9, 0.5, 0.1],   # Orange
        [0.9, 0.9, 0.2],   # Yellow
        [0.2, 0.8, 0.2],   # Green
        [0.2, 0.5, 0.9],   # Blue
        [0.6, 0.2, 0.9],   # Purple
    ]

    # Build a 5x5 base tower, 4 levels high (100 cubes)
    for y in range(4):
        color = colors[y % len(colors)]
        for x in range(5):
            for z in range(5):
                scene.add_cube_colored(
                    [x * 1.1 - 2.2, y * 1.1 + 0.55, z * 1.1 - 2.2],
                    0.5, 1.0, color
                )

    # 3 balls with different colors
    ball_colors = [
        [0.9, 0.3, 0.3],   # Red ball
        [0.3, 0.9, 0.3],   # Green ball
        [0.3, 0.3, 0.9],   # Blue ball
    ]

    # Ball 1 - from front
    scene.add_sphere_with_velocity_colored(
        [-15.0, 3.0, 0.0],
        [30.0, 0.0, 0.0],
        1.2, 15.0,
        ball_colors[0]
    )

    # Ball 2 - from side
    scene.add_sphere_with_velocity_colored(
        [0.0, 5.0, -15.0],
        [0.0, 0.0, 30.0],
        1.2, 15.0,
        ball_colors[1]
    )

    # Ball 3 - from above
    scene.add_sphere_with_velocity_colored(
        [0.0, 20.0, 0.0],
        [0.0, -15.0, 0.0],
        1.5, 20.0,
        ball_colors[2]
    )

    cubes, spheres = scene.shape_counts()
    print(f"Scene 1: {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([20.0, 12.0, 20.0], [0.0, 2.0, 0.0])

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
