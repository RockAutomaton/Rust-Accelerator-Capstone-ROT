//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {

    let _ = dotenvy::dotenv();

    // WiFi Network - required
    let wifi_network = env::var("WIFI_NETWORK").unwrap_or_else(|_| {
        println!("cargo:warning=WIFI_NETWORK not set, using default");
        "YOUR_WIFI_SSID".to_string()
    });

    // WiFi Password - required
    let wifi_password = env::var("WIFI_PASSWORD").unwrap_or_else(|_| {
        println!("cargo:warning=WIFI_PASSWORD not set, using default");
        "YOUR_WIFI_PASSWORD".to_string()
    });

    // Telemetry Host - required
    let telemetry_host = env::var("TELEMETRY_HOST").unwrap_or_else(|_| {
        println!("cargo:warning=TELEMETRY_HOST not set, using default");
        "YOUR_TELEMETRY_HOST".to_string()
    });

    let config_host = env::var("CONFIG_HOST").unwrap_or_else(|_| {
        println!("cargo:warning=CONFIG_HOST not set, using default");
        "YOUR_CONFIG_HOST".to_string()
    });

    let device_id = env::var("DEVICE_ID").unwrap_or_else(|_| {
        println!("cargo:warning=DEVICE_ID not set, using default");
        "YOUR_DEVICE_ID".to_string()
    });

    // Debug Server - optional
    let debug_server = env::var("DEBUG_SERVER").unwrap_or_else(|_| {
        println!("cargo:warning=DEBUG_SERVER not set, using default");
        "localhost".to_string()
    });

    // Pass to compiler as constants
    println!("cargo:rustc-env=WIFI_NETWORK={}", wifi_network);
    println!("cargo:rustc-env=WIFI_PASSWORD={}", wifi_password);
    println!("cargo:rustc-env=TELEMETRY_HOST={}", telemetry_host);
    println!("cargo:rustc-env=CONFIG_HOST={}", config_host);
    println!("cargo:rustc-env=DEBUG_SERVER={}", debug_server);
    println!("cargo:rustc-env=DEVICE_ID={}", device_id);

    // Rebuild if .env file changes
    println!("cargo:rerun-if-changed=.env");
    println!("cargo:rerun-if-env-changed=WIFI_NETWORK");
    println!("cargo:rerun-if-env-changed=WIFI_PASSWORD");
    println!("cargo:rerun-if-env-changed=DEBUG_SERVER");
    println!("cargo:rerun-if-env-changed=CONFIG_HOST");
    println!("cargo:rerun-if-env-changed=DEVICE_ID");
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");
}
