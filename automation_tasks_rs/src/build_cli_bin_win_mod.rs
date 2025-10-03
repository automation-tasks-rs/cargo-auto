// build_cli_bin_win_mod.rs

//! Functions to cross-build a CLI binary executable from Linux to Windows.
//!
//! Don't change this code, so it can be updated regularly with
//! cargo auto update_automation_tasks_rs
//! If you want to customize it, copy the code into main.rs and modify it there.

use crate::cl;

use cargo_auto_lib::CargoTomlPublicApiMethods;
#[allow(unused_imports)]
use cl::{BLUE, GREEN, RED, RESET, YELLOW};

#[allow(dead_code)]
/// cargo build --target x86_64-pc-windows-gnu
pub fn task_build() -> anyhow::Result<cl::CargoToml> {
    let cargo_toml = cl::CargoToml::read()?;
    cl::auto_version_increment_semver_or_date()?;
    cl::run_shell_command_static("cargo fmt")?;
    cl::run_shell_command_static("cargo clippy --no-deps --target x86_64-pc-windows-gnu")?;
    cl::run_shell_command_static("cargo build --target x86_64-pc-windows-gnu")?;
    Ok(cargo_toml)
}

#[allow(dead_code)]
/// cargo build --release --target x86_64-pc-windows-gnu
pub fn task_release() -> anyhow::Result<cl::CargoToml> {
    let cargo_toml = cl::CargoToml::read()?;
    cl::auto_version_increment_semver_or_date()?;
    cl::auto_cargo_toml_to_md()?;
    cl::auto_lines_of_code("")?;

    cl::run_shell_command_static("cargo fmt")?;
    cl::run_shell_command_static("cargo clippy --no-deps --target x86_64-pc-windows-gnu")?;
    cl::run_shell_command_static("cargo build --release --target x86_64-pc-windows-gnu")?;

    Ok(cargo_toml)
}
