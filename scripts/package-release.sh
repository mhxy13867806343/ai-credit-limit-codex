#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$DIR"

echo "==> 1. Building application bundle..."
./scripts/build-app.sh

echo "==> 2. Creating release distribution packages in dist/..."
mkdir -p "$DIR/dist"
rm -f "$DIR/dist/Codex-Monitor-macOS.dmg" "$DIR/dist/Codex-Monitor-macOS.zip"
rm -rf "$DIR/dist/dmg_staging"

STAGING_DIR="$DIR/dist/dmg_staging"
mkdir -p "$STAGING_DIR"
cp -r "$DIR/target/Codex Monitor.app" "$STAGING_DIR/"
ln -s /Applications "$STAGING_DIR/Applications"

if [ -f "$DIR/assets/icon.icns" ]; then
    cp "$DIR/assets/icon.icns" "$STAGING_DIR/.VolumeIcon.icns"
    SetFile -c icnC "$STAGING_DIR/.VolumeIcon.icns" 2>/dev/null || true
    SetFile -a C "$STAGING_DIR" 2>/dev/null || true
fi

echo "==> 3. Generating DMG installer..."
hdiutil detach "/Volumes/Codex Monitor" 2>/dev/null || true
hdiutil create -volname "Codex Monitor" -srcfolder "$STAGING_DIR" -ov -format UDZO "$DIR/dist/Codex-Monitor-macOS.dmg"
rm -rf "$STAGING_DIR"

echo "==> 4. Generating ZIP package..."
(cd "$DIR/target" && zip -r -y "../dist/Codex-Monitor-macOS.zip" "Codex Monitor.app")

echo "==> 5. Setting file icons on DMG and ZIP..."
if [ -f "$DIR/assets/icon.icns" ] && [ -x "$DIR/scripts/generate_icon" ]; then
    "$DIR/scripts/generate_icon" --set-icon "$DIR/assets/icon.icns" "$DIR/dist/Codex-Monitor-macOS.dmg" || true
    "$DIR/scripts/generate_icon" --set-icon "$DIR/assets/icon.icns" "$DIR/dist/Codex-Monitor-macOS.zip" || true
fi

echo "==> Successfully packaged release artifacts:"
ls -lh "$DIR/dist"

