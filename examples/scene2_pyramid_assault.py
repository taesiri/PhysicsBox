"""Scene 2: Pyramid Assault - 500 cubes in a pyramid attacked by 8 balls."""

import physobx
import os
import math
from datetime import datetime

def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/scene2_pyramid_assault_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    scene = physobx.Scene()
    scene.add_ground(0.0, 150.0)

    # Build a large pyramid with gradient colors
    cube_count = 0
    base_size = 12

    for level in range(base_size):
        size = base_size - level
        # Color gradient from warm (bottom) to cool (top)
        t = level / (base_size - 1)
        color = [
            0.9 - t * 0.5,    # Red decreases
            0.3 + t * 0.3,    # Green increases slightly
            0.2 + t * 0.6,    # Blue increases
        ]

        offset = level * 0.525  # Center each level
        for x in range(size):
            for z in range(size):
                scene.add_cube_colored(
                    [x * 1.05 - size * 0.525 + 0.525, level * 1.05 + 0.525, z * 1.05 - size * 0.525 + 0.525],
                    0.5, 1.0, color
                )
                cube_count += 1

    print(f"Built pyramid with {cube_count} cubes")

    # 8 balls from different angles in a ring pattern
    num_balls = 8
    for i in range(num_balls):
        angle = i * 2 * math.pi / num_balls
        distance = 30.0
        height = 3.0 + (i % 3) * 3.0  # Vary heights

        start_x = math.cos(angle) * distance
        start_z = math.sin(angle) * distance

        # Velocity toward center
        speed = 35.0
        vel_x = -math.cos(angle) * speed
        vel_z = -math.sin(angle) * speed

        # Ball color - metallic gradient
        hue = i / num_balls
        color = [
            0.4 + 0.5 * math.sin(hue * 2 * math.pi),
            0.4 + 0.5 * math.sin(hue * 2 * math.pi + 2.09),
            0.4 + 0.5 * math.sin(hue * 2 * math.pi + 4.19),
        ]

        scene.add_sphere_with_velocity_colored(
            [start_x, height, start_z],
            [vel_x, 2.0, vel_z],
            1.3, 18.0,
            color
        )

    cubes, spheres = scene.shape_counts()
    print(f"Scene 2: {cubes} cubes, {spheres} balls")

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
