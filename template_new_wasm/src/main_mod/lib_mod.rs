// src/lib_mod.rs
// This module is like a lib.rs module for a binary CLI executable.
// The `lib_mod.rs` must not contains any input/output interface stuff.
// So the program logic can be separate from the interface.

// The `main_mod.rs` contains all input/output interface stuff.
// This `lib_mod.rs` can then be used as dependency crate for other projects.

// The `lib_mod.rs` does not have any real code. All the code is in modules in separate files.
// The `lib_mod.rs` has just the list of modules, it publishes module's functions or class for the caller
// and it has some global stuff like the Error enum.

// The `main_mod.rs` uses the `anyhow` error library.
// The `lib_mod.rs` uses the `thiserror` library.

use thiserror::Error;

// Instead of a hello_mod local module, we could use a UI agnostic crate library dependency.
// So the same library could be used for CLI and for WASM, that have vastly different UI.
mod hello_mod;

pub mod web_sys_mod;
pub use web_sys_mod as wsm;

pub use hello_mod::format_hello_phrase;
pub use hello_mod::format_upper_hello_phrase;

/// all possible library errors for `thiserror`
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("Name `{0}` is already uppercase.")]
    Uppercase(String),
    #[error("Unknown error.")]
    Unknown,
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
