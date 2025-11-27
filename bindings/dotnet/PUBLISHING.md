# Publishing redstr .NET Bindings to NuGet

This guide explains how to publish the redstr .NET bindings to NuGet.org.

## Prerequisites

1. **NuGet Account**: Create an account at [nuget.org](https://www.nuget.org/)
2. **API Key**: Generate an API key from your NuGet account settings
   - Go to https://www.nuget.org/account/apikeys
   - Create a new API key with "Push" permissions
   - Select the package ID pattern: `Redstr`
3. **GitHub Secret**: Add the API key as a GitHub secret
   - Go to repository Settings → Secrets and variables → Actions
   - Create a new secret named `NUGET_API_KEY`
   - Paste your NuGet API key

## Automated Publishing (Recommended)

The GitHub Actions workflow `.github/workflows/dotnet-bindings.yml` automatically:

1. Builds native libraries for all platforms (Windows, Linux, macOS x64/ARM64)
2. Creates the NuGet package with all native libraries included
3. Publishes to NuGet.org on releases

### Steps for Automated Publishing

1. **Update Version**: Update the version in `bindings/dotnet/Redstr/Redstr.csproj`
   ```xml
   <Version>0.2.4</Version>
   ```

2. **Update Rust Version**: Ensure `Cargo.toml` has the same version
   ```toml
   version = "0.2.4"
   ```

3. **Commit and Push**: Commit changes to main branch
   ```bash
   git add .
   git commit -m "chore: bump version to 0.2.4"
   git push origin main
   ```

4. **Create Release**: Create a new release on GitHub
   - Go to https://github.com/arvid-berndtsson/redstr/releases/new
   - Tag version: `v0.2.4`
   - Release title: `v0.2.4`
   - Add release notes
   - Click "Publish release"

5. **Monitor Workflow**: The GitHub Actions workflow will automatically:
   - Build native libraries for all platforms
   - Create the NuGet package
   - Publish to NuGet.org

The package will be available at: https://www.nuget.org/packages/Redstr/

## Manual Publishing

For testing or emergency releases, you can publish manually.

### Prerequisites

- Rust toolchain installed
- .NET 8.0 SDK installed
- NuGet API key

### Build Native Libraries

You need to build native libraries for all platforms. This typically requires:

**On Linux:**
```bash
cargo build --release --features ffi
cp target/release/libredstr.so bindings/dotnet/Redstr/runtimes/linux-x64/native/redstr.so
```

**On Windows:**
```cmd
cargo build --release --features ffi
copy target\release\redstr.dll bindings\dotnet\Redstr\runtimes\win-x64\native\redstr.dll
```

**On macOS (Intel):**
```bash
cargo build --release --features ffi
cp target/release/libredstr.dylib bindings/dotnet/Redstr/runtimes/osx-x64/native/redstr.dylib
```

**On macOS (Apple Silicon):**
```bash
cargo build --release --features ffi
cp target/release/libredstr.dylib bindings/dotnet/Redstr/runtimes/osx-arm64/native/redstr.dylib
```

### Create and Publish Package

```bash
# Navigate to the .NET project
cd bindings/dotnet/Redstr

# Build the project
dotnet build -c Release

# Create the NuGet package
dotnet pack -c Release -o ./artifacts

# Publish to NuGet
dotnet nuget push ./artifacts/Redstr.*.nupkg \
  --api-key YOUR_API_KEY \
  --source https://api.nuget.org/v3/index.json
```

## Cross-Compilation (Advanced)

For building all platform libraries from a single machine, you can use cross-compilation.

### Linux to Windows

```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Install MinGW
sudo apt-get install mingw-w64

# Build
cargo build --release --features ffi --target x86_64-pc-windows-gnu
```

### Linux to macOS

This requires osxcross, which is more complex to set up. Using GitHub Actions is recommended.

## Verification

After publishing, verify the package:

1. **Check NuGet.org**: Visit https://www.nuget.org/packages/Redstr/
2. **Test Installation**: Create a test project and install the package
   ```bash
   mkdir test-redstr
   cd test-redstr
   dotnet new console
   dotnet add package Redstr
   dotnet run
   ```

3. **Verify Platforms**: Check that native libraries are included
   ```bash
   # Examine the .nupkg file
   unzip -l Redstr.*.nupkg
   # Should contain:
   # - runtimes/win-x64/native/redstr.dll
   # - runtimes/linux-x64/native/redstr.so
   # - runtimes/osx-x64/native/redstr.dylib
   # - runtimes/osx-arm64/native/redstr.dylib
   ```

## Troubleshooting

### Package Upload Fails

- **Check version**: Ensure the version hasn't been published before
- **Check API key**: Verify the API key has correct permissions
- **Check package size**: NuGet has a 250 MB limit per package

### Native Library Not Found

- Ensure all platform native libraries are in the correct `runtimes/{rid}/native/` directories
- Verify the library naming is correct:
  - Windows: `redstr.dll`
  - Linux: `redstr.so` (or `libredstr.so`)
  - macOS: `redstr.dylib` (or `libredstr.dylib`)

### Missing Platforms

If a platform is missing, you need to:
1. Build the native library on that platform OR use cross-compilation
2. Copy it to the appropriate `runtimes/{rid}/native/` directory
3. Rebuild the NuGet package

## Package Metadata

The package metadata is configured in `bindings/dotnet/Redstr/Redstr.csproj`:

- **PackageId**: `Redstr`
- **Version**: Should match Cargo.toml version
- **Authors**: Arvid Berndtsson
- **Description**: See `.csproj` file
- **License**: MIT
- **ProjectUrl**: https://github.com/arvid-berndtsson/redstr
- **Tags**: security, pentesting, red-team, obfuscation, waf-bypass, xss, sql-injection, phishing

## Release Checklist

Before publishing a new version:

- [ ] Update version in `bindings/dotnet/Redstr/Redstr.csproj`
- [ ] Update version in `Cargo.toml` (should match)
- [ ] Update CHANGELOG.md with new features/fixes
- [ ] Test all platforms locally or via CI
- [ ] Verify example application works
- [ ] Create GitHub release with tag
- [ ] Monitor GitHub Actions workflow
- [ ] Verify package on NuGet.org
- [ ] Test installation: `dotnet add package Redstr --version X.Y.Z`
- [ ] Update documentation if needed

## Support

For issues with publishing:
- Check GitHub Actions logs for errors
- Review NuGet documentation: https://docs.microsoft.com/en-us/nuget/
- Open an issue: https://github.com/arvid-berndtsson/redstr/issues
