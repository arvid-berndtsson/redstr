#!/bin/bash
# Build script for redstr Go bindings
# This script compiles the Rust library and prepares it for use with Go

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
BUILD_MODE="${1:-release}"

echo "Building redstr library for Go bindings..."
echo "Root dir: $ROOT_DIR"
echo "Build mode: $BUILD_MODE"

cd "$ROOT_DIR"

if [ "$BUILD_MODE" = "debug" ]; then
    echo "Building in debug mode..."
    cargo build --features ffi
    
    # Copy library to bindings directory
    if [ -f "target/debug/libredstr.so" ]; then
        cp target/debug/libredstr.so bindings/go/
    elif [ -f "target/debug/libredstr.dylib" ]; then
        cp target/debug/libredstr.dylib bindings/go/
    elif [ -f "target/debug/libredstr.dll" ]; then
        cp target/debug/libredstr.dll bindings/go/
    fi
else
    echo "Building in release mode..."
    cargo build --release --features ffi
    
    # Copy library to bindings directory
    if [ -f "target/release/libredstr.so" ]; then
        cp target/release/libredstr.so bindings/go/
    elif [ -f "target/release/libredstr.dylib" ]; then
        cp target/release/libredstr.dylib bindings/go/
    elif [ -f "target/release/libredstr.dll" ]; then
        cp target/release/libredstr.dll bindings/go/
    fi
fi

echo "Generating C header file..."
cbindgen --config cbindgen.toml --crate redstr --output bindings/go/libredstr.h

echo "Build complete!"
echo "Library files are in: $ROOT_DIR/bindings/go/"
