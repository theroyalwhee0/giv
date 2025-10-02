# Reproducible Builds

Reproducible builds are a way to ensure that the same source code will produce the same binary output, regardless of the environment in which it is built.

This package has initial support for reproducible builds through build-time environment variables.

## Building

To build with locked dependencies (recommended for reproducibility):

```bash
cargo build --locked
```

To build in release mode:

```bash
cargo build --locked --release
```

## Build Environment Data

To support reproducible builds, the build script records build-time values in a `build.env` file located in the build output directory (`target/debug/` or `target/release/`). This file contains environment variables that affect or verify the build output.

## Environment Variables

### `SOURCE_DATE_EPOCH`

This variable sets the timestamp of the build. It is the number of seconds since UNIX epoch (January 1, 1970 00:00:00 UTC).

- **If not provided**: The build will use the current time
- **If provided**: Must be a valid positive integer representing seconds since UNIX epoch
- **If invalid**: The build will fail with a clear error message

**Example:**

```bash
# Build with a specific timestamp (January 1, 2024 00:00:00 UTC)
SOURCE_DATE_EPOCH=1704067200 cargo build --locked
```

### `EXPECT_PROFILE`

This variable is used to verify the build profile matches expectations. Valid values are `debug` or `release`.

- **If not provided**: The current build profile is accepted without verification
- **If provided**: The build will fail if the actual profile doesn't match the expected value

**Example:**

```bash
# Verify that we're building in release mode
EXPECT_PROFILE=release cargo build --release
```

### `TZ`

The timezone is automatically fixed to `UTC` in the build script. This ensures that build times are consistent across different time zones. This cannot be overridden.

## Combined Usage

You can combine multiple environment variables:

```bash
# Reproducible release build with specific timestamp
SOURCE_DATE_EPOCH=1704067200 EXPECT_PROFILE=release cargo build --locked --release
```

## Build Outputs

### `build.env` File

After building, you'll find a `build.env` file in the build output directory with the captured build variables:

```env
SOURCE_DATE_EPOCH=1704067200
EXPECT_PROFILE=release
```

### Version Information

The build date and profile are included in the `--version` output:

```bash
giv --version
```

This will show the build date time and build profile along with version and repository information.

## Checking Paths in the Binary

Path mapping is not yet implemented. To check for embedded paths in the binary:

```bash
cd target/debug
readelf -p .debug_str ./giv | grep $HOME
```

## Current Limitations

This is an **initial implementation** of reproducible builds:

- ✅ Deterministic timestamps via `SOURCE_DATE_EPOCH`
- ✅ Build profile verification via `EXPECT_PROFILE`
- ✅ Fixed timezone (`UTC`)
- ✅ Build metadata recording
- ❌ Path mapping (not yet implemented)
- ❌ Complete reproducibility verification tooling (future work)

## References

- [Reproducible Builds Specification](https://reproducible-builds.org/)
- [SOURCE_DATE_EPOCH Specification](https://reproducible-builds.org/specs/source-date-epoch/)
- [Cargo Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
