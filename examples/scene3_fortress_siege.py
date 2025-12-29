"""Scene 3: Fortress Siege - 1000+ cubes in a fortress attacked by 15 balls."""

import physobx
import os
import math
from datetime import datetime

def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/scene3_fortress_siege_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    scene = physobx.Scene()
    scene.add_ground(0.0, 200.0)

    cube_count = 0

    # Main fortress walls (4 walls)
    wall_height = 8
    wall_length = 15

    # Wall colors - stone gray with variation
    for i in range(wall_length):
        for y in range(wall_height):
            # Vary the gray slightly for texture
            gray = 0.5 + 0.1 * ((i + y) % 3 - 1)
            color = [gray * 0.95, gray * 0.9, gray * 0.85]

            # Front wall
            scene.add_cube_colored([i * 1.05 - 7.5, y * 1.05 + 0.525, -8.0], 0.5, 1.2, color)
            # Back wall
            scene.add_cube_colored([i * 1.05 - 7.5, y * 1.05 + 0.525, 8.0], 0.5, 1.2, color)
            # Left wall
            scene.add_cube_colored([-8.0, y * 1.05 + 0.525, i * 1.05 - 7.5], 0.5, 1.2, color)
            # Right wall
            scene.add_cube_colored([8.0, y * 1.05 + 0.525, i * 1.05 - 7.5], 0.5, 1.2, color)
            cube_count += 4

    # Corner towers (4 towers, taller)
    tower_height = 12
    tower_positions = [[-8, -8], [-8, 8], [8, -8], [8, 8]]
    tower_colors = [
        [0.7, 0.3, 0.3],  # Red tower
        [0.3, 0.7, 0.3],  # Green tower
        [0.3, 0.3, 0.7],  # Blue tower
        [0.7, 0.7, 0.3],  # Yellow tower
    ]

    for (tx, tz), base_color in zip(tower_positions, tower_colors):
        for y in range(tower_height):
            for dx in range(2):
                for dz in range(2):
                    # Darken towards top
                    t = y / tower_height
                    color = [c * (1.0 - t * 0.3) for c in base_color]
                    scene.add_cube_colored(
                        [tx + dx * 1.05 - 0.525, y * 1.05 + 0.525, tz + dz * 1.05 - 0.525],
                        0.5, 1.5, color
                    )
                    cube_count += 1

    # Central keep (3x3 base, 10 tall)
    keep_height = 10
    for y in range(keep_height):
        for x in range(3):
            for z in range(3):
                # Golden keep
                t = y / keep_height
                color = [0.85 - t * 0.2, 0.65 - t * 0.15, 0.2]
                scene.add_cube_colored(
                    [x * 1.05 - 1.05, y * 1.05 + 0.525, z * 1.05 - 1.05],
                    0.5, 1.8, color
                )
                cube_count += 1

    print(f"Built fortress with {cube_count} cubes")

    # 15 siege balls from multiple directions and heights
    ball_configs = [
        # From front (5 balls)
        ([-25, 4, 0], [40, 0, 0], 1.4, 20.0, [0.9, 0.2, 0.2]),
        ([-25, 8, 3], [38, -2, -3], 1.2, 15.0, [0.9, 0.4, 0.2]),
        ([-25, 6, -3], [38, 0, 5], 1.3, 18.0, [0.9, 0.3, 0.1]),
        ([-25, 10, 0], [35, -3, 0], 1.5, 25.0, [0.8, 0.2, 0.2]),
        ([-25, 2, 0], [42, 5, 0], 1.1, 12.0, [0.7, 0.3, 0.3]),
        # From sides (4 balls)
        ([0, 5, -25], [0, 0, 40], 1.4, 20.0, [0.2, 0.9, 0.2]),
        ([0, 8, 25], [0, 0, -40], 1.4, 20.0, [0.2, 0.2, 0.9]),
        ([25, 6, 5], [-40, 0, -5], 1.3, 18.0, [0.9, 0.9, 0.2]),
        ([25, 4, -5], [-40, 2, 5], 1.3, 18.0, [0.9, 0.5, 0.2]),
        # From above (3 balls)
        ([0, 35, 0], [0, -20, 0], 2.0, 35.0, [0.5, 0.5, 0.9]),
        ([5, 30, 5], [-5, -18, -5], 1.5, 22.0, [0.6, 0.4, 0.8]),
        ([-5, 30, -5], [5, -18, 5], 1.5, 22.0, [0.4, 0.6, 0.8]),
        # Diagonal attacks (3 balls)
        ([-20, 8, -20], [30, -2, 30], 1.4, 20.0, [0.8, 0.4, 0.8]),
        ([20, 6, -20], [-30, 0, 30], 1.4, 20.0, [0.4, 0.8, 0.8]),
        ([-20, 10, 20], [30, -5, -30], 1.6, 25.0, [0.8, 0.8, 0.4]),
    ]

    for pos, vel, radius, mass, color in ball_configs:
        scene.add_sphere_with_velocity_colored(pos, vel, radius, mass, color)

    cubes, spheres = scene.shape_counts()
    print(f"Scene 3: {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([45.0, 25.0, 45.0], [0.0, 6.0, 0.0])

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
