// build_cli_bin_mod.rs

//! Functions to build a CLI binary executable.

use crate::cl;
use crate::ende;

use cargo_auto_lib::CargoTomlPublicApiMethods;
use cargo_auto_lib::ShellCommandLimitedDoubleQuotesSanitizerTrait;
#[allow(unused_imports)]
use cl::{BLUE, GREEN, RED, RESET, YELLOW};

/// cargo build
pub fn task_build() -> cl::CargoToml {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::run_shell_command_static("cargo fmt").unwrap_or_else(|e| panic!("{e}"));
    cl::run_shell_command_static("cargo clippy --no-deps").unwrap_or_else(|e| panic!("{e}"));
    cl::run_shell_command_static("cargo build").unwrap_or_else(|e| panic!("{e}"));
    cargo_toml
}

/// cargo build --release
pub fn task_release() -> cl::CargoToml {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");

    cl::run_shell_command_static("cargo fmt").unwrap_or_else(|e| panic!("{e}"));
    cl::run_shell_command_static("cargo clippy --no-deps").unwrap_or_else(|e| panic!("{e}"));
    cl::run_shell_command_static("cargo build --release").unwrap_or_else(|e| panic!("{e}"));

    // strip only for binary executables
    #[cfg(target_family = "unix")]
    if std::fs::exists("target/release/{package_name}").unwrap() {
        cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"strip "target/release/{package_name}" "#)
            .unwrap_or_else(|e| panic!("{e}"))
            .arg("{package_name}", &cargo_toml.package_name())
            .unwrap_or_else(|e| panic!("{e}"))
            .run()
            .unwrap_or_else(|e| panic!("{e}"));
    }
    cargo_toml
}

#[allow(dead_code)]
/// publish to crates.io and git tag
pub fn task_publish_to_crates_io() -> String {
    let cargo_toml = cl::CargoToml::read();
    let package_name = cargo_toml.package_name();
    let version = cargo_toml.package_version();
    // take care of tags
    let tag_name_version = cl::git_tag_sync_check_create_push(&version);

    // cargo publish with encrypted secret secret_token
    ende::crates_io_api_token_mod::publish_to_crates_io().unwrap();

    println!(
        r#"
  {YELLOW}After `cargo auto publish_to_crates_io`, check in browser{RESET}
{GREEN}https://crates.io/crates/{package_name}{RESET}
  {YELLOW}Install the crate with{RESET}
{GREEN}cargo install {package_name}{RESET}
  {YELLOW}and check how it works.{RESET}
"#
    );
    tag_name_version
}
