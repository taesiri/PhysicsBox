"""Example: 1000 falling cubes simulation with GPU rendering."""

import physobx
import time
import os
from datetime import datetime

def main():
    print(f"Physobx version: {physobx.version()}")

    # Create output folder
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/falling_cubes_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    # Create scene with ground and 10x10x10 = 1000 cubes
    scene = physobx.Scene()
    scene.add_ground(0.0, 50.0)
    scene.add_cube_grid(
        center=[0.0, 15.0, 0.0],
        spacing=2.0,
        count=[10, 10, 10],
        half_extent=0.5,
        mass=1.0,
    )
    print(f"Created scene with {scene.body_count()} cubes")

    # Create simulator with 1080p rendering
    sim = physobx.Simulator(scene, width=1920, height=1080)
    print(f"Render dimensions: {sim.dimensions()}")

    # Set camera to view the scene
    sim.set_camera(
        eye=[35.0, 30.0, 45.0],
        target=[0.0, 8.0, 0.0],
    )

    # Simulate and save frames
    fps = 60
    duration = 3.0  # seconds
    total_frames = int(fps * duration)

    print(f"Simulating {duration}s at {fps}fps ({total_frames} frames)...")
    start_time = time.perf_counter()

    for frame in range(total_frames):
        # Physics step
        sim.step(1.0 / fps)

        # Save frame
        filename = f"{output_dir}/frame_{frame:04d}.png"
        sim.save_png(filename)

        if frame % 30 == 0:
            print(f"  Frame {frame}/{total_frames}")

    elapsed = time.perf_counter() - start_time
    print(f"\nCompleted in {elapsed:.2f}s ({total_frames / elapsed:.1f} fps)")

    # Final positions
    positions = sim.get_positions()
    print(f"Final Y range: {positions[:, 1].min():.2f} to {positions[:, 1].max():.2f}")
    print(f"Done! Frames saved to {output_dir}/")


if __name__ == "__main__":
    main()
