// cargo-auto/src/main.rs

// logo for docs.rs in png
#![doc(html_logo_url = "https://github.com/bestia-dev/cargo-auto/raw/main/images/logo/logo_cargo_auto.svg")]
// even favicon ico can be changed
// #![doc(html_favicon_url = "/logo.ico")]
// playground for examples
#![doc(html_playground_url = "https://play.rust-lang.org/")]
// example how to insert a svg file inside the documentation
// #![doc=include_str!("shared-bus.svg")]
#![doc=include_str!("../README.md")]

mod file_hashes_mod;
mod inside_of_rust_project_mod;
mod outside_of_rust_project_mod;
mod template_new_auto_mod;
mod template_new_cli_mod;
mod template_new_pwa_wasm_mod;
mod template_new_wasm_mod;
mod utils_mod;

// region: use statements
use lazy_static::lazy_static;
use std::path::{Path, PathBuf};
// endregion: use statements

// paths
lazy_static! {
    /// constant paths for read/write
    static ref PATH_AUTOMATION_TASKS_RS: PathBuf = PathBuf::from("automation_tasks_rs");
    /// constant paths for read/write
    static ref PATH_CARGO_TOML: PathBuf = PathBuf::from("automation_tasks_rs/Cargo.toml");
    /// constant paths for read/write
    static ref PATH_GITIGNORE: PathBuf = PathBuf::from("automation_tasks_rs/.gitignore");
    /// constant paths for read/write
    static ref PATH_SRC_MAIN_RS: PathBuf = PathBuf::from("automation_tasks_rs/src/main.rs");
    /// constant paths for folder
    static ref PATH_SRC: PathBuf = PathBuf::from("automation_tasks_rs/src");
    /// constant paths for read/write
    static ref PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS: PathBuf =
        PathBuf::from("automation_tasks_rs/target/debug/automation_tasks_rs");
    /// constant paths for read/write
    static ref PATH_FILE_HASHES_JSON: PathBuf =
        PathBuf::from("automation_tasks_rs/.file_hashes.json");
}

// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";

/// file item
pub struct FileItem {
    file_name: &'static str,
    file_content: &'static str,
}

fn main() {
    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();

    if is_not_run_in_rust_project_root_directory() {
        outside_of_rust_project_mod::parse_args(&mut args);
    } else {
        inside_of_rust_project_mod::parse_args(&mut args);
    }
}

/// Check if is not run in Rust project root directory
fn is_not_run_in_rust_project_root_directory() -> bool {
    // return negation of exists
    !Path::new("Cargo.toml").exists()
}
