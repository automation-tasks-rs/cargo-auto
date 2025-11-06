// build_cli_bin_mod.rs

//! Functions to build a CLI binary executable.
//!
//! Don't change this code, so it can be updated regularly with
//! cargo auto update_automation_tasks_rs
//! If you want to customize it, copy the code into main.rs and modify it there.

use crate::cargo_auto_lib as cl;

use crate::cargo_auto_lib::CargoTomlPublicApiMethods;
use crate::cargo_auto_lib::ShellCommandLimitedDoubleQuotesSanitizerTrait;
use crate::generic_functions_mod::ResultLogError;
#[allow(unused_imports)]
use cl::{BLUE, GREEN, RED, RESET, YELLOW};

#[allow(dead_code)]
/// cargo build
pub fn task_build() -> anyhow::Result<cl::CargoToml> {
    let cargo_toml = cl::CargoToml::read().log()?;
    cl::auto_version_increment_semver_or_date().log()?;
    cl::run_shell_command_static("cargo fmt").log()?;
    cl::run_shell_command_static("cargo clippy --no-deps").log()?;
    cl::run_shell_command_static("cargo build").log()?;
    Ok(cargo_toml)
}

#[allow(dead_code)]
/// cargo build --release
pub fn task_release() -> anyhow::Result<cl::CargoToml> {
    let cargo_toml = cl::CargoToml::read().log()?;
    cl::auto_version_increment_semver_or_date().log()?;
    cl::auto_cargo_toml_to_md().log()?;
    cl::auto_lines_of_code("").log()?;

    cl::run_shell_command_static("cargo fmt").log()?;
    cl::run_shell_command_static("cargo clippy --no-deps").log()?;
    cl::run_shell_command_static("cargo build --release").log()?;

    // strip only for binary executables
    #[cfg(target_family = "unix")]
    if std::fs::exists("target/release/{package_name}").log()? {
        cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"strip "target/release/{package_name}" "#).log()?
            .arg("{package_name}", &cargo_toml.package_name()).log()?
            .run().log()?;
    }
    Ok(cargo_toml)
}
