"""Ball cannon vs cube wall - Multiple balls fired at a massive cube wall."""

import physobx
import os
from datetime import datetime

def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/ball_cannon_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    scene = physobx.Scene()
    scene.add_ground(0.0, 150.0)

    # Build a massive cube wall (10 wide x 8 tall x 3 deep)
    wall_center = [0.0, 4.5, 0.0]
    for y in range(8):
        for x in range(10):
            for z in range(3):
                pos = [
                    wall_center[0] + (x - 4.5) * 1.05,
                    wall_center[1] + y * 1.05 - 3.5,
                    wall_center[2] + (z - 1) * 1.05,
                ]
                scene.add_cube(pos, 0.5, 1.0)

    # Fire balls at the wall over time (we'll add them during simulation)
    # For now, add the initial balls
    ball_positions = [
        ([-20.0, 2.0, 0.0], [45.0, 5.0, 0.0]),   # First ball
        ([-22.0, 3.5, 2.0], [50.0, 3.0, -3.0]),  # Second ball
        ([-21.0, 1.5, -1.5], [48.0, 6.0, 2.0]),  # Third ball
    ]

    for pos, vel in ball_positions:
        scene.add_sphere_with_velocity(pos, vel, 1.0, 8.0)

    cubes, spheres = scene.shape_counts()
    print(f"Created wall with {cubes} cubes and {spheres} balls")

    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([25.0, 12.0, 30.0], [0.0, 4.0, 0.0])

    # 10 seconds at 60fps
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
