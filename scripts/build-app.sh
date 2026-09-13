#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$DIR"

if [ ! -f "$DIR/target/release/codex-monitor" ]; then
    cargo build --release
fi

APP_NAME="Codex Monitor.app"
APP_DIR="$DIR/target/$APP_NAME"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"

echo "==> Creating macOS Application Bundle at $APP_DIR..."
rm -rf "$APP_DIR"
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

# Copy binary
cp "$DIR/target/release/codex-monitor" "$MACOS_DIR/codex-monitor"
chmod +x "$MACOS_DIR/codex-monitor"

# Copy Info.plist
cp "$DIR/scripts/Info.plist" "$CONTENTS_DIR/Info.plist"

# Copy icon if exists
if [ -f "$DIR/assets/icon.icns" ]; then
    cp "$DIR/assets/icon.icns" "$RESOURCES_DIR/AppIcon.icns"
fi

echo "==> Application bundle successfully created!"
echo "    Path: $APP_DIR"
echo "    Size: $(du -sh "$APP_DIR" | cut -f1)"
echo ""
echo "To run:"
echo "    open \"$APP_DIR\""
