#!/bin/bash
# Build script for compiling the native Rust library for different platforms

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building redstr native library for .NET bindings${NC}"

# Navigate to project root
cd "$(dirname "$0")/../.."

# Build for the current platform
echo -e "${YELLOW}Building for current platform...${NC}"
cargo build --release --features ffi

# Determine the library extension based on the OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    LIB_EXT="so"
    LIB_PREFIX="lib"
    PLATFORM="linux-x64"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    LIB_EXT="dylib"
    LIB_PREFIX="lib"
    if [[ $(uname -m) == "arm64" ]]; then
        PLATFORM="osx-arm64"
    else
        PLATFORM="osx-x64"
    fi
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    LIB_EXT="dll"
    LIB_PREFIX=""
    PLATFORM="win-x64"
else
    echo -e "${RED}Unknown platform: $OSTYPE${NC}"
    exit 1
fi

# Create output directory
NATIVE_DIR="bindings/dotnet/Redstr/runtimes/$PLATFORM/native"
mkdir -p "$NATIVE_DIR"

# Copy the built library
SRC_LIB="target/release/${LIB_PREFIX}redstr.${LIB_EXT}"
DST_LIB="$NATIVE_DIR/redstr.${LIB_EXT}"

if [ -f "$SRC_LIB" ]; then
    cp "$SRC_LIB" "$DST_LIB"
    echo -e "${GREEN}✓ Copied native library to $DST_LIB${NC}"
else
    echo -e "${RED}✗ Native library not found at $SRC_LIB${NC}"
    exit 1
fi

echo -e "${GREEN}Native library build complete!${NC}"
echo -e "${YELLOW}Note: For cross-platform builds, use GitHub Actions or cross-compilation tools${NC}"
