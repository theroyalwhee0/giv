//! Build script for giv.
//!
//! This build script handles reproducible build support by managing
//! SOURCE_DATE_EPOCH, EXPECT_PROFILE, and TZ environment variables.

#![forbid(unsafe_code)]

use chrono::{DateTime, Utc};
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// REF: https://doc.rust-lang.org/cargo/reference/build-scripts.html
// REF: https://reproducible-builds.org/specs/source-date-epoch/

/// The build script main.
///
/// # Returns
///
/// - Ok if the build script runs successfully. Else it returns an error.
fn main() -> io::Result<()> {
    // Inform Cargo to rerun this build script when state changes.
    println!("cargo:rerun-if-changed=build.rs");

    // Get the output directory from Cargo.
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Expected OUT_DIR to be set"));

    // Find the build directory by locating the 'build' component.
    // OUT_DIR is something like 'target/<profile>/build/<package-hash>/out'.
    // We want 'target/<profile>/'.
    let build_dir = out_dir
        .ancestors()
        .find(|p| p.file_name().and_then(|n| n.to_str()) == Some("build"))
        .and_then(|p| p.parent())
        .expect("Could not find build directory");

    // Create the env file name.
    let env_file_name = "build.env";

    // Create or truncate the env file beside the binary.
    let env_file_path = build_dir.join(env_file_name);
    let mut env_file = File::create(&env_file_path)?;

    // Add SOURCE_DATE_EPOCH to the build environment variables and the env file.
    add_source_date_epoch(&mut env_file)?;

    // Add TZ to the build environment variables and the env file.
    add_tz(&mut env_file)?;

    // Add build mode (release/debug) to the env file.
    add_profile(&mut env_file)?;

    // Success.
    Ok(())
}

/// Add SOURCE_DATE_EPOCH to the build environment variables.
///
/// Uses the SOURCE_DATE_EPOCH environment variable if defined.
/// SOURCE_DATE_EPOCH should be seconds since the UNIX epoch.
///
/// If not defined, it uses the current time.
///
/// # Parameters
///
/// - `file` The build data file to write the environment variable to.
///
/// # Returns
///
/// - Ok if the build script runs successfully. Else it returns an error.
fn add_source_date_epoch(file: &mut File) -> io::Result<()> {
    // SOURCE_DATE_EPOCH env var name.
    const SOURCE_DATE_EPOCH: &str = "SOURCE_DATE_EPOCH";

    // Use SOURCE_DATE_EPOCH if defined, otherwise use the current time.
    let timestamp = env::var(SOURCE_DATE_EPOCH)
        .ok()
        .and_then(|value| {
            if value.is_empty() {
                None
            } else {
                value
                    .parse::<u64>()
                    .map_err(|err| panic!("Invalid {SOURCE_DATE_EPOCH} '{value}': {err}"))
                    .ok()
                    .map(Duration::from_secs)
            }
        })
        .unwrap_or_else(|| {
            // If not set, get the current time in seconds since the UNIX epoch.
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
        });

    // Convert the timestamp to seconds since the UNIX epoch.
    let source_date_epoch = timestamp.as_secs();

    // Format the timestamp in various formats using chrono
    let dt = DateTime::<Utc>::from_timestamp(source_date_epoch as i64, 0)
        .expect("Invalid timestamp for DateTime conversion");
    let date_iso = dt.format("%Y-%m-%d").to_string();
    let datetime_iso = dt.format("%Y-%m-%dT%H:%M:%SZ").to_string();

    // Write to the 'build.env' file.
    writeln!(file, "{SOURCE_DATE_EPOCH}={source_date_epoch}")?;
    // Inform Cargo to rerun this build script when the source is changed.
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");
    // Inform Cargo to rerun this build script when state changes.
    println!("cargo:rerun-if-env-changed={SOURCE_DATE_EPOCH}");

    // Set the SOURCE_DATE_EPOCH environment variable for the build.
    println!("cargo:rustc-env={SOURCE_DATE_EPOCH}={source_date_epoch}");

    // Set various timestamp formats for the build.
    println!("cargo:rustc-env=BUILD_TIMESTAMP={source_date_epoch}");
    println!("cargo:rustc-env=BUILD_DATE_ISO={date_iso}");
    println!("cargo:rustc-env=BUILD_DATETIME_ISO={datetime_iso}");

    // Success.
    Ok(())
}

/// Add a fixed TZ of 'UTC' to the build environment variables.
///
/// # Parameters
///
/// - `file` The build data file to write the environment variable to.
///
/// # Returns
///
/// - Ok if the build script runs successfully. Else it returns an error.
fn add_tz(_file: &mut File) -> io::Result<()> {
    // TZ env var name.
    const TZ: &str = "TZ";
    const UTC: &str = "UTC";

    // Set the TZ environment variable for the build.
    println!("cargo:rustc-env={TZ}={UTC}");

    // Success.
    Ok(())
}

/// Add build mode to the build environment variables.
///
/// Determines if the build is in release or debug mode and adds it to the env file.
/// If EXPECT_PROFILE is defined, it verifies that it matches the current profile.
///
/// This variable has no effect on the build process, it is just to record the build mode.
///
/// # Parameters
///
/// - `file` The build data file to write the environment variable to.
///
/// # Returns
///
/// - Ok if the build script runs successfully. Else it returns an error.
fn add_profile(file: &mut File) -> io::Result<()> {
    const PROFILE: &str = "PROFILE";
    const EXPECT_PROFILE: &str = "EXPECT_PROFILE";
    const RELEASE: &str = "release";
    const DEBUG: &str = "debug";

    // Determine if we're building in release mode
    let profile = env::var(PROFILE).expect("Expected PROFILE to be set");
    let is_release = profile == RELEASE;
    let mode = if is_release { RELEASE } else { DEBUG };

    // If EXPECT_PROFILE is defined, verify it matches our current profile.
    if let Ok(expected) = env::var(EXPECT_PROFILE) {
        if expected != mode {
            panic!("Expected profile '{expected}' but building with profile '{mode}'");
        }
    }

    // Write to the env file
    writeln!(file, "{EXPECT_PROFILE}={mode}")?;

    // Inform Cargo to rerun this build script when profile changes
    println!("cargo:rerun-if-env-changed={PROFILE}");
    println!("cargo:rerun-if-env-changed={EXPECT_PROFILE}");

    // Set the EXPECT_PROFILE environment variable for the build
    println!("cargo:rustc-env={EXPECT_PROFILE}={mode}");

    // Success
    Ok(())
}
