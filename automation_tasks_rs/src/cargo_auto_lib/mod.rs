// lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # cargo_auto_lib
//!
//! **Automation tasks coded in Rust language for the workflow of Rust projects**  
//! ***version: 4.0.6 date: 2025-10-02 author: [Bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/automation-tasks-rs/cargo_auto_lib)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![ready-for-use](https://img.shields.io/badge/ready_for_use-green)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!  ![automation](https://img.shields.io/badge/automation-orange)
//!  ![workflow](https://img.shields.io/badge/workflow-orange)
//!
//!  ![logo](https://raw.githubusercontent.com/automation-tasks-rs/cargo-auto/main/images/logo/logo_cargo_auto.svg)
//!  cargo_auto_lib is part of the [automation_tasks_rs](https://github.com/automation-tasks-rs) project
//!
//!  [![crates.io](https://img.shields.io/crates/v/cargo_auto_lib.svg)](https://crates.io/crates/cargo_auto_lib)
//!  [![Documentation](https://docs.rs/cargo_auto_lib/badge.svg)](https://docs.rs/cargo_auto_lib/)
//!  [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/cargo_auto_lib.svg)](https://web.crev.dev/rust-reviews/crate/cargo_auto_lib/)
//!  [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/cargo_auto_lib/)  
//!  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/automation-tasks-rs/cargo_auto_lib/blob/master/LICENSE)
//!  [![Rust](https://github.com/automation-tasks-rs/cargo_auto_lib/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/automation-tasks-rs/cargo_auto_lib/)
//!  [![Newest docs](https://img.shields.io/badge/newest_docs-blue.svg)](https://automation-tasks-rs.github.io/cargo_auto_lib/cargo_auto_lib/index.html)
//!  ![cargo_auto_lib](https://bestia.dev/webpage_hit_counter/get_svg_image/276360626.svg)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-2031-green.svg)](https://github.com/automation-tasks-rs/cargo_auto_lib/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-993-blue.svg)](https://github.com/automation-tasks-rs/cargo_auto_lib/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-292-purple.svg)](https://github.com/automation-tasks-rs/cargo_auto_lib/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-28-yellow.svg)](https://github.com/automation-tasks-rs/cargo_auto_lib/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-80-orange.svg)](https://github.com/automation-tasks-rs/cargo_auto_lib/)
//!
//! Hashtags: #maintained #ready-for-use #rustlang #automation #workflow  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  
//! I recommend using the [CRUSTDE - Containerized Rust Development Environment](https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod) to write Rust projects on Linux, isolated from your system.  
//!
//! ## Try it
//!
//! First, install the tool for task automation in Rust projects:
//!
//! ```bash
//! cargo install cargo-auto
//! # Generate a new Rust CLI project:
//! cargo auto new_cli hello_world
//! # Open the `hello_world` project in VSCode:
//! code hello_world
//! # Open the generated directory `automation_tasks_rs` as an independent rust project in VSCode.
//! code hello_world/automation_tasks_rs
//! ```
//!
//! Now we can analyze the automation code. There is already this dependency inside `Cargo.toml` for our library:  
//!
//! ```toml
//! cargo_auto_lib="2.0.2"
//! ```
//!
//! Review the code in `automation_tasks_rs/main.rs`. The `cl::` namespace is the alias for `cargo_auto_lib`.  
//! Example:  
//!
//! ```rust ignore
//! /// cargo build --release
//! fn task_release() {
//!     let cargo_toml = CargoToml::read();
//!     cl::auto_version_increment_semver_or_date();
//!     cl::auto_cargo_toml_to_md();
//!     cl::auto_lines_of_code("");
//!
//!     cl::run_shell_command("cargo fmt");
//!     cl::run_shell_command("cargo build --release");
//!     println!(
//!         r#"
//!   {YELLOW}After `cargo auto release`, run the compiled binary, examples and/or tests{RESET}
//! {GREEN}./target/release/{package_name} arg_1{RESET}
//!   {YELLOW}if ok then{RESET}
//! {GREEN}cargo auto doc{RESET}
//! "#,
//! package_name = cargo_toml.package_name(),
//!     );
//! }
//! ```
//!
//! You can see this function will increment the version in `Cargo.toml`.  
//! Then it will copy some data from `Cargo.toml` to README.md (title, description, version, author,...).  
//! It will count the lines of code and create badges in README.md.  
//! Then comes the traditional Rust part: cargo fmt and cargo build --release.  
//! Finally, it will show on the screen the instructions on how to continue developing.  
//!
//! Run (in your main rust project):
//!
//! ```bash
//! cargo auto release
//! ```
//!
//! Now open the README.md and you will see the data that this automation task copied from other places. Therefore you change this data only in one place, the automation task copies them wherever needed.
//!
//! ## Development details
//!
//! Read the development details in a separate md file:  
//! [DEVELOPMENT.md](https://github.com/automation-tasks-rs/cargo_auto_lib/blob/main/DEVELOPMENT.md)
//!
//! ## Releases changelog
//!
//! Read the release changelog in a separate md file:  
//! [RELEASES.md](https://github.com/automation-tasks-rs/cargo_auto_lib/blob/main/RELEASES.md)
//!
//! ## TODO
//!
//! - remove the existing support for workspaces. Workspaces are a mess
//! - change it so that every member must have its own automation
//! - the workspace automation then calls the member's automation
//! - It is better to have a non-workspace group of projects that are
//! - developed together using single workflow automation
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

// region: mod, extern and use statements
mod auto_cargo_toml_mod;
mod auto_cargo_toml_to_md_mod;
mod auto_check_micro_xml_mod;
mod auto_copy_files_to_strings_mod;
mod auto_delete_old_js_snippets_mod;
mod auto_doc_tidy_html_mod;
mod auto_git_mod;
mod auto_github_mod;
mod auto_helper_functions_mod;
mod auto_lines_of_code_mod;
mod auto_md_to_doc_comments_mod;
mod auto_plantuml_mod;
mod auto_playground_mod;
mod auto_semver_mod;
mod auto_semver_or_date_mod;
mod auto_shell_mod;
mod auto_version_from_date_mod;
mod error_mod;
mod public_api_mod;
mod utils_mod;
// endregion

// only the Public API is exported (functions, structs, methods, enums, traits)
pub use public_api_mod::*;
