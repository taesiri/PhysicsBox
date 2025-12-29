#!/bin/bash
# Convert PNG frames to MP4 video using ffmpeg
# Usage: ./frames_to_video.sh <frame_folder> [fps]
#
# Output video is saved next to the frame folder as <folder_name>.mp4

set -e

FRAME_DIR="${1:-.}"
FPS="${2:-60}"

# Get absolute path and folder name
FRAME_DIR=$(cd "$FRAME_DIR" && pwd)
FOLDER_NAME=$(basename "$FRAME_DIR")
PARENT_DIR=$(dirname "$FRAME_DIR")
OUTPUT_VIDEO="${PARENT_DIR}/${FOLDER_NAME}.mp4"

# Find frame pattern (frame_0000.png or frame_000.png)
if ls "$FRAME_DIR"/frame_0000.png >/dev/null 2>&1; then
    PATTERN="frame_%04d.png"
elif ls "$FRAME_DIR"/frame_000.png >/dev/null 2>&1; then
    PATTERN="frame_%03d.png"
else
    echo "Error: No frame_*.png files found in $FRAME_DIR"
    exit 1
fi

echo "Converting frames to video..."
echo "  Input:  $FRAME_DIR/$PATTERN"
echo "  Output: $OUTPUT_VIDEO"
echo "  FPS:    $FPS"

ffmpeg -y -framerate "$FPS" -i "$FRAME_DIR/$PATTERN" \
    -c:v libx264 -preset medium -crf 18 \
    -pix_fmt yuv420p \
    -movflags +faststart \
    "$OUTPUT_VIDEO"

echo "Done! Video saved to: $OUTPUT_VIDEO"
