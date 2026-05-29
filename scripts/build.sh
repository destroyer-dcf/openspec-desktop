#!/bin/zsh
set -e

export PATH="$HOME/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH"

echo "Cleaning previous build..."
cd src-tauri && cargo clean && cd ..

echo "Building for x86_64-apple-darwin..."
npm run tauri build -- --target x86_64-apple-darwin

echo "Building for aarch64-apple-darwin..."
npm run tauri build -- --target aarch64-apple-darwin

echo "Done. Bundles:"
echo "  src-tauri/target/x86_64-apple-darwin/release/bundle/"
echo "  src-tauri/target/aarch64-apple-darwin/release/bundle/"
