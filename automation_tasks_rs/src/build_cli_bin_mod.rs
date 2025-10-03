// build_cli_bin_mod.rs

//! Functions to build a CLI binary executable.
//!
//! Don't change this code, so it can be updated regularly with
//! cargo auto update_automation_tasks_rs
//! If you want to customize it, copy the code into main.rs and modify it there.

use crate::cl;
use crate::ende;

use cargo_auto_lib::CargoTomlPublicApiMethods;
use cargo_auto_lib::ShellCommandLimitedDoubleQuotesSanitizerTrait;
#[allow(unused_imports)]
use cl::{BLUE, GREEN, RED, RESET, YELLOW};

#[allow(dead_code)]
/// cargo build
pub fn task_build() -> anyhow::Result<cl::CargoToml> {
    let cargo_toml = cl::CargoToml::read()?;
    cl::auto_version_increment_semver_or_date()?;
    cl::run_shell_command_static("cargo fmt")?;
    cl::run_shell_command_static("cargo clippy --no-deps")?;
    cl::run_shell_command_static("cargo build")?;
    Ok(cargo_toml)
}

#[allow(dead_code)]
/// cargo build --release
pub fn task_release() -> anyhow::Result<cl::CargoToml> {
    let cargo_toml = cl::CargoToml::read()?;
    cl::auto_version_increment_semver_or_date()?;
    cl::auto_cargo_toml_to_md()?;
    cl::auto_lines_of_code("")?;

    cl::run_shell_command_static("cargo fmt")?;
    cl::run_shell_command_static("cargo clippy --no-deps")?;
    cl::run_shell_command_static("cargo build --release")?;

    // strip only for binary executables
    #[cfg(target_family = "unix")]
    if std::fs::exists("target/release/{package_name}")? {
        cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"strip "target/release/{package_name}" "#)?
            .arg("{package_name}", &cargo_toml.package_name())?
            .run()?;
    }
    Ok(cargo_toml)
}

#[allow(dead_code)]
/// publish to crates.io and git tag
pub fn task_publish_to_crates_io() -> anyhow::Result<String> {
    let cargo_toml = cl::CargoToml::read()?;
    let package_name = cargo_toml.package_name();
    let version = cargo_toml.package_version();
    // take care of tags
    let tag_name_version = cl::git_tag_sync_check_create_push(&version)?;

    // cargo publish with encrypted secret secret_token
    ende::crates_io_api_token_mod::publish_to_crates_io()?;

    println!(
        r#"
  {YELLOW}After `cargo auto publish_to_crates_io`, check in browser{RESET}
{GREEN}https://crates.io/crates/{package_name}{RESET}
  {YELLOW}Install the crate with{RESET}
{GREEN}cargo install {package_name}{RESET}
  {YELLOW}and check how it works.{RESET}
"#
    );
    Ok(tag_name_version)
}
