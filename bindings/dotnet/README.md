# Redstr .NET Bindings

This directory contains the C# / .NET bindings for the redstr library.

## Project Structure

```
bindings/dotnet/
├── Redstr/                    # Main .NET library project
│   ├── Redstr.cs             # High-level C# API
│   ├── NativeMethods.cs      # P/Invoke declarations
│   ├── Redstr.csproj         # Project configuration
│   ├── Redstr.targets        # MSBuild targets for native library
│   ├── README.md             # Package README for NuGet
│   └── runtimes/             # Native libraries per platform
│       ├── linux-x64/
│       ├── win-x64/
│       ├── osx-x64/
│       └── osx-arm64/
├── RedstrExample/            # Example console application
│   ├── Program.cs
│   └── RedstrExample.csproj
├── build-native.sh           # Script to build Rust library
└── README.md                 # This file
```

## Building

### Prerequisites

- .NET 8.0 SDK or later
- Rust toolchain (for building native library)
- Platform-specific build tools

### Build Steps

1. **Build the Rust library with FFI support:**
   ```bash
   cd ../..  # Go to project root
   cargo build --release --features ffi
   ```

2. **Copy native library to the .NET project:**
   ```bash
   cd bindings/dotnet
   ./build-native.sh
   ```

3. **Build the .NET library:**
   ```bash
   cd Redstr
   dotnet build
   ```

4. **Run the example:**
   ```bash
   cd ../RedstrExample
   dotnet run
   ```

### Creating a NuGet Package

To create a NuGet package with all platform-specific native libraries:

1. Build native libraries for all target platforms (requires cross-compilation setup or CI/CD)
2. Run the packaging command:
   ```bash
   cd Redstr
   dotnet pack -c Release
   ```

The package will be created in `bin/Release/Redstr.{version}.nupkg`.

## Cross-Platform Support

The .NET bindings support the following platforms:

- **Windows**: x64
- **Linux**: x64
- **macOS**: x64 (Intel), arm64 (Apple Silicon)

Each platform requires a native library built specifically for that platform.

### Building for Multiple Platforms

For production releases, use cross-compilation or CI/CD:

#### Using GitHub Actions (Recommended)

The repository includes GitHub Actions workflows that automatically build native libraries for all platforms.

#### Manual Cross-Compilation

**For Windows from Linux:**
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --features ffi --target x86_64-pc-windows-gnu
```

**For macOS from Linux (requires osxcross):**
```bash
rustup target add x86_64-apple-darwin
cargo build --release --features ffi --target x86_64-apple-darwin
```

## Publishing to NuGet

1. Ensure all native libraries are built and placed in the appropriate `runtimes/{rid}/native/` directories
2. Update the version in `Redstr.csproj`
3. Create the package:
   ```bash
   dotnet pack -c Release
   ```
4. Publish to NuGet:
   ```bash
   dotnet nuget push bin/Release/Redstr.{version}.nupkg --api-key YOUR_API_KEY --source https://api.nuget.org/v3/index.json
   ```

## Usage

### Installation

Install via NuGet:
```bash
dotnet add package Redstr
```

### Example Code

```csharp
using Redstr;

// Random capitalization
var result = RedstrTransform.RandomizeCapitalization("Hello World");
Console.WriteLine(result);  // "HeLlO wOrLd"

// Leetspeak
var obfuscated = RedstrTransform.Leetspeak("password");
Console.WriteLine(obfuscated);  // "p@55w0rd"

// Homoglyph substitution
var spoofed = RedstrTransform.HomoglyphSubstitution("admin@example.com");
Console.WriteLine(spoofed);  // "аdmіn@еxаmple.com"

// Random user agent
var ua = RedstrTransform.RandomUserAgent();
Console.WriteLine(ua);
```

See `RedstrExample/Program.cs` for more examples.

## Development

### Adding New Functions

When adding new functions to the Rust library:

1. Add the function to `src/ffi.rs` with appropriate C FFI bindings
2. Add the P/Invoke declaration to `NativeMethods.cs`
3. Add the high-level C# wrapper to `Redstr.cs`
4. Update the documentation
5. Add examples to `RedstrExample/Program.cs`

### Testing

To test the bindings locally:

1. Build the native library: `./build-native.sh`
2. Build the .NET library: `cd Redstr && dotnet build`
3. Run the example: `cd ../RedstrExample && dotnet run`

## Troubleshooting

### "Unable to load shared library 'redstr'"

This error occurs when the native library cannot be found. Ensure:

1. The native library is built and placed in the correct `runtimes/{rid}/native/` directory
2. The library has the correct name:
   - Windows: `redstr.dll`
   - Linux: `libredstr.so`
   - macOS: `libredstr.dylib`
3. The library is copied to the output directory during build

### Platform-Specific Issues

**Linux:** Ensure `glibc` version compatibility
**macOS:** May require code signing for distribution
**Windows:** May require Visual C++ Redistributable

## Architecture

The .NET bindings use P/Invoke to call native Rust functions through a C FFI interface:

```
C# Application
    ↓
RedstrTransform (High-level API)
    ↓
NativeMethods (P/Invoke declarations)
    ↓
C FFI (src/ffi.rs)
    ↓
Rust Library (src/lib.rs)
```

## License

MIT License - See LICENSE file in the repository root.
