// build_lib_mod.rs

//! Functions to build a library crate.
//!
//! Don't change this code, so it can be updated regularly with
//! cargo auto update_automation_tasks_rs
//! If you want to customize it, copy the code into main.rs and modify it there.

use crate::cl;
use crate::ende;

use cargo_auto_lib::CargoTomlPublicApiMethods;
#[allow(unused_imports)]
use cl::{BLUE, GREEN, RED, RESET, YELLOW};

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
  {YELLOW}Add the dependency to your Rust project and check how it works.{RESET}
{GREEN}{package_name} = "{version}"{RESET}
"#
    );
    Ok(tag_name_version)
}
