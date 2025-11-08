// build_lib_mod.rs

//! Functions to build a library crate.
//!
//! Don't change this code, so it can be updated regularly with
//! cargo auto update_automation_tasks_rs
//! If you want to customize it, copy the code into main.rs and modify it there.

use crate::cargo_auto_lib as cl;
use crate::encrypt_decrypt_with_ssh_key_mod as ende;

use crate::cargo_auto_lib::CargoTomlPublicApiMethods;
use crate::utils_mod::pos;
use crate::utils_mod::ResultLogError;
#[allow(unused_imports)]
use cl::{BLUE, GREEN, RED, RESET, YELLOW};

#[allow(dead_code)]
/// publish to crates.io and git tag
pub fn task_publish_to_crates_io() -> anyhow::Result<(String, String, String)> {
    let cargo_toml = cl::CargoToml::read().log(pos!())?;
    let package_name = cargo_toml.package_name();
    let version = cargo_toml.package_version();
    // take care of tags
    let tag_name_version = cl::git_tag_sync_check_create_push(&version).log(pos!())?;

    // cargo publish with encrypted secret secret_token
    ende::crates_io_api_token_mod::publish_to_crates_io().log(pos!())?;

    Ok((tag_name_version, package_name, version))
}
