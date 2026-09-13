#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$DIR"

echo "==> 1. Building application bundle..."
./scripts/build-app.sh

echo "==> 2. Creating release distribution packages in dist/..."
mkdir -p "$DIR/dist"
rm -rf "$DIR/dist/*"

STAGING_DIR="$DIR/dist/dmg_staging"
mkdir -p "$STAGING_DIR"
cp -r "$DIR/target/Codex Monitor.app" "$STAGING_DIR/"
ln -s /Applications "$STAGING_DIR/Applications"

echo "==> 3. Generating DMG installer..."
hdiutil create -volname "Codex Monitor" -srcfolder "$STAGING_DIR" -ov -format UDZO "$DIR/dist/Codex-Monitor-macOS.dmg"
rm -rf "$STAGING_DIR"

echo "==> 4. Generating ZIP package..."
(cd "$DIR/target" && zip -r -y "../dist/Codex-Monitor-macOS.zip" "Codex Monitor.app")

echo "==> Successfully packaged release artifacts:"
ls -lh "$DIR/dist"
