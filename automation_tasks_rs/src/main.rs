// automation_tasks_rs for cargo-auto

// region: library and modules with basic automation tasks

mod build_cli_bin_mod;
mod cargo_auto_github_api_mod;
mod encrypt_decrypt_with_ssh_key_mod;
mod generic_functions_mod;

pub use cargo_auto_lib as cl;

use crate::cargo_auto_github_api_mod as cgl;
use crate::encrypt_decrypt_with_ssh_key_mod as ende;
use crate::generic_functions_mod as gn;

pub use cl::{BLUE, GREEN, RED, RESET, YELLOW};

// traits must be in scope (Rust strangeness)
use cl::CargoTomlPublicApiMethods;

// region: library with basic automation tasks

fn main() {
    std::panic::set_hook(Box::new(gn::panic_set_hook));
    gn::tracing_init();
    cl::exit_if_not_run_in_rust_project_root_directory();
    ende::github_api_token_with_oauth2_mod::github_api_config_initialize();
    ende::crates_io_api_token_mod::crates_io_config_initialize();
    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("  {YELLOW}Running automation task: {task}{RESET}");
                if &task == "build" {
                    task_build();
                } else if &task == "release" {
                    task_release();
                } else if &task == "doc" {
                    task_doc();
                } else if &task == "test" {
                    task_test();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else if &task == "publish_to_crates_io" {
                    task_publish_to_crates_io();
                } else if &task == "github_new_release" {
                    task_github_new_release();
                } else {
                    eprintln!("{RED}Error: Task {task} is unknown.{RESET}");
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
  {YELLOW}Welcome to cargo-auto !{RESET}
  {YELLOW}This program automates your custom tasks when developing a Rust project.{RESET}

  {YELLOW}User defined tasks in automation_tasks_rs:{RESET}
{GREEN}cargo auto build{RESET} - {YELLOW}builds the crate in debug mode, fmt, increment version{RESET}
{GREEN}cargo auto release{RESET} - {YELLOW}builds the crate in release mode, fmt, increment version{RESET}
{GREEN}cargo auto doc{RESET} - {YELLOW}builds the docs, copy to docs directory{RESET}
{GREEN}cargo auto test{RESET} - {YELLOW}runs all the tests{RESET}
{GREEN}cargo auto commit_and_push "message"{RESET} - {YELLOW}commits with message and push with mandatory message{RESET}
  {YELLOW}It is preferred to use SSH for git push to GitHub.{RESET}
  {YELLOW}<https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod/blob/main/ssh_easy.md>{YELLOW}
  {YELLOW}On the very first commit, this task will initialize a new local git repository and create a remote GitHub repo.{RESET}
  {YELLOW}For the GitHub API the task needs the Access secret token from OAuth2 device workflow.{RESET}
  {YELLOW}The secret token will be stored in a file encrypted with your SSH private key.{RESET}
  {YELLOW}You can type the passphrase of the private key for every usee. This is pretty secure.{RESET}
  {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}
{GREEN}cargo auto publish_to_crates_io{RESET} - {YELLOW}publish to crates.io, git tag{RESET}
  {YELLOW}You need the API secret_token for publishing. Get the secret_token on <https://crates.io/settings/tokens>.{RESET}
  {YELLOW}The secret token will be stored in a file encrypted with your SSH private key.{RESET}
  {YELLOW}You can type the passphrase of the private key for every usee. This is pretty secure.{RESET}
  {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}
{GREEN}cargo auto github_new_release{RESET} - {YELLOW}creates new release on GitHub{RESET}
  {YELLOW}For the GitHub API the task needs the Access secret token from OAuth2 device workflow.{RESET}
  {YELLOW}The secret token will be stored in a file encrypted with your SSH private key.{RESET}
  {YELLOW}You can type the passphrase of the private key for every usee. This is pretty secure.{RESET}
  {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}
{GREEN}cargo auto update_automation_tasks_rs{RESET} - {YELLOW}updates the files in automation_tasks_rs{RESET}
  {YELLOW}Some files are fixed and the update is straight forward, other files need manual diff.{RESET}

  {YELLOW}© 2025 bestia.dev  MIT License github.com/automation-tasks-rs/cargo-auto{RESET}
"#
    );
    print_examples_cmd();
}

/// all example commands in one place
fn print_examples_cmd() {
    /*
            println!(
                r#"
      {YELLOW}run examples:{RESET}
    {GREEN}cargo run --example plantuml1{RESET}
        "#
            );
        */
}

/// Sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`.
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec![
            "build",
            "release",
            "doc",
            "test",
            "commit_and_push",
            "publish_to_crates_io",
            "github_new_release",
            "update_automation_tasks_rs"
        ];
        cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
       cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// cargo build
fn task_build() {
    let cargo_toml = crate::build_cli_bin_mod::task_build();
    println!(
        r#"
  {YELLOW}After `cargo auto build`, run the compiled binary, examples and/or tests{RESET}
{GREEN}./target/debug/{package_name} arg_1{RESET}
  {YELLOW}if ok then{RESET}
{GREEN}cargo auto release{RESET}
"#,
        package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
}

/// cargo build --release
fn task_release() {
    let cargo_toml = crate::build_cli_bin_mod::task_release();

    println!(
        r#"
  {YELLOW}After `cargo auto release`, run the compiled binary, examples and/or tests{RESET}

  {YELLOW}1. Check if the template `new_cli` is working. Open a second VSCode:{RESET}
{GREEN}code ~/rustprojects{RESET}
  {YELLOW}and run in its terminal:{RESET}
{GREEN}./cargo-auto/target/release/{package_name} new_cli hello_world bestia-dev;{RESET}
{GREEN}code hello_world{RESET}
  {YELLOW} In the new VSCode try the workflow: {RESET}
{GREEN}cargo auto build, cargo auto release, cargo auto doc,... {RESET}
  {YELLOW}Follow the instructions, try all tasks.{RESET}
  {YELLOW}Leave this VSCode open for the next check.{RESET}

  {YELLOW}2. Check if the template `new_auto_for_cli` is working. Continue in the third VSCode:{RESET}
{GREEN}../cargo-auto/target/release/{package_name} new_auto_for_cli;{RESET}
  {YELLOW}An Error must be produced because automation_tasks_rs already exists.{RESET}
  {YELLOW}Remove the directory and try again:{RESET}
{GREEN}rm -r automation_tasks_rs{RESET}
{GREEN}../cargo-auto/target/release/{package_name} new_auto_for_cli;{RESET}
  {YELLOW}Try the workflow: {RESET}
{GREEN}cargo auto build, cargo auto release, cargo auto doc,... {RESET}
  {YELLOW}Follow the instructions, try all tasks.{RESET}
  {YELLOW}If ok, close the third VSCode window. In the second VSCode remove the temporary project:{RESET}
{GREEN}rm -rf hello_world{RESET}
  {YELLOW}Close the second VSCode.{RESET}


  {YELLOW}3. Check if the template `new_wasm` is working. Open a second VSCode:{RESET}
{GREEN}code ~/rustprojects{RESET}
  {YELLOW}and run in its terminal:{RESET}
{GREEN}./cargo-auto/target/release/{package_name} new_wasm hello_world bestia-dev bestia.dev luciano_bestia;{RESET}
{GREEN}code hello_world{RESET}
  {YELLOW} In the new VSCode try the workflow: {RESET}
{GREEN}cargo auto build, cargo auto release, cargo auto doc,... {RESET}
  {YELLOW}Follow the instructions, try all tasks.{RESET}
  {YELLOW}If ok, close the third VSCode window. In the second VSCode remove the temporary project:{RESET}
{GREEN}rm -rf hello_world{RESET}
  {YELLOW}Close the second VSCode.{RESET}


  {YELLOW}4. Check if the template `new_pwa_wasm` is working. Open a second VSCode:{RESET}
{GREEN}code ~/rustprojects{RESET}
  {YELLOW}and run in its terminal:{RESET}
  {YELLOW}Copy your 'icon512x512.png' into this folder or download and use this default icon.{RESET}
{GREEN}curl -L https://github.com/automation-tasks-rs/cargo_auto_template_new_pwa_wasm/raw/main/icon512x512.png --output icon512x512.png{RESET}
{GREEN}./cargo-auto/target/release/{package_name} new_pwa_wasm hello_world bestia-dev bestia.dev luciano_bestia;{RESET}
{GREEN}code hello_world{RESET}
  {YELLOW} In the new VSCode try the workflow: {RESET}
{GREEN}cargo auto build, cargo auto release, cargo auto doc,... {RESET}
  {YELLOW}Follow the instructions, try all tasks.{RESET}
{GREEN}rm icon512x512.png{RESET}
{GREEN}rm -rf hello_world{RESET}
  {YELLOW}Close the second VSCode.{RESET}

  {YELLOW}if ok then{RESET}
{GREEN}cargo auto doc{RESET}
"#,
        package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
}

/// cargo doc, then copies to /docs/ folder, because this is a GitHub standard folder
fn task_doc() {
    gn::task_doc();
    // message to help user with next move
    println!(
        r#"
  {YELLOW}If ok then run the tests in code and the documentation code examples.{RESET}
{GREEN}cargo auto test{RESET}
"#
    );
}

/// cargo test
fn task_test() {
    cl::run_shell_command_static("cargo test").unwrap_or_else(|e| panic!("{e}"));
    println!(
        r#"
  {YELLOW}After `cargo auto test`. If ok then {RESET}
  {YELLOW}(commit message is mandatory){RESET}
{GREEN}cargo auto commit_and_push "message"{RESET}
"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    gn::task_commit_and_push(arg_2);
    println!(
        r#"
  {YELLOW}After `cargo auto commit_and_push "message"`{RESET}
{GREEN}cargo auto publish_to_crates_io{RESET}
"#
    );
}

/// publish to crates.io and git tag
fn task_publish_to_crates_io() {
    let tag_name_version = crate::build_cli_bin_mod::task_publish_to_crates_io();

    println!(
        r#"
  {YELLOW}Now, write the content of the release in the RELEASES.md in the `## Unreleased` section, then{RESET}
  {YELLOW}Next, create the GitHub Release {tag_name_version}.{RESET}
{GREEN}cargo auto github_new_release{RESET}
"#
    );
}

/// create a new release on github
fn task_github_new_release() {
    gn::task_github_new_release();
    println!(r#"  {YELLOW}No more automation tasks. {RESET}"#);
}
// endregion: tasks
