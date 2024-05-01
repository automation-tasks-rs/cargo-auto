// template_new_auto_mod.rs

//! template for new_auto (automation_tasks_rs)
//!
//! An automation task copy the content of the template_new_wasm folder into this strings.
//! When installing a crate from crates.io, only the code is transferred. No additional files.

use crate::file_hashes_mod;
#[allow(unused)]
use crate::{GREEN, RED, RESET, YELLOW};

/// copies the template to the `automation_tasks_rs` directory  
/// in development use: `cargo run -- new_auto`  
/// in runtime use: `cargo auto new_auto`  
pub fn new_auto() {
    crate::template_new_auto_mod::copy_to_files("automation_tasks_rs");
    // panic! if cannot compile automation_tasks_rs
    compile_automation_tasks_rs_if_needed();

    println!(
        r#"
    {YELLOW}`crate auto new_auto` generated the directory `automation_tasks_rs` in your main Rust project.
    You can open this new helper Rust project in a new Rust editor.
    View and edit the Rust code in `automation_tasks_rs`. It is independent from the main project.
    It will be automatically compiled on the next use of `crate auto task_name` command.
    The new directory will be added to your git commit.
    There is a local .gitignore file to avoid commit of the `target/` directory.
{RESET}"#
    );
    // call `cargo auto` to show the help of the new automation_tasks_rs
    let _success = crate::utils_mod::run_shell_command_success("cargo auto");
}

/// build if the files are different then the hashes in automation_tasks_rs/file_hashes.json
/// panic! if cannot compile automation_tasks_rs
pub fn compile_automation_tasks_rs_if_needed() {
    if !crate::PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS.exists() || file_hashes_mod::is_project_changed() {
        compile_project_automation_tasks_rs();
        let vec_of_metadata = file_hashes_mod::read_file_metadata();
        file_hashes_mod::save_json_file_for_file_meta_data(vec_of_metadata);
    }
}

/// build automation_tasks_rs
/// panic! if cannot compile automation_tasks_rs
pub fn compile_project_automation_tasks_rs() {
    // build in other directory (not in working current directory)
    // cargo build --manifest-path=dir/Cargo.toml
    if !crate::utils_mod::run_shell_command_success("cargo build --manifest-path=automation_tasks_rs/Cargo.toml") {
        panic!("{RED}Cannot compile automation_tasks_rs. Exiting...{RESET}");
    }
}

pub fn copy_to_files(rust_project_name: &str) {
    let folder_path = std::path::Path::new(rust_project_name);
    std::fs::create_dir_all(folder_path).unwrap();
    for file_item in get_vec_file() {
        // create directory if needed
        std::fs::create_dir_all(folder_path.join(&file_item.file_name).parent().unwrap()).unwrap();
        std::fs::write(folder_path.join(&file_item.file_name), file_item.file_content.as_bytes()).unwrap();
    }
}

pub fn get_vec_file() -> Vec<crate::FileItem> {
    let mut vec_file = vec![];

    // region: files copied into strings by automation tasks
    vec_file.push(crate::FileItem {
        file_name: "rustfmt.toml",
        file_content: r###"max_width = 200
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: ".gitignore",
        file_content: r###"/target
/logs

# so the GitHub action gets the fresh libraries
Cargo.lock

# not needed in commits, but also not a problem if they are committed
/.file_hashes.json
/.old_metadata.json
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/main.rs",
        file_content: r###"// automation_tasks_rs for cargo_auto_template_new_cli

// region: library and modules with basic automation tasks

// for projects that don't use GitHub, delete all the mentions of GitHub
mod secrets_always_local_mod;
use crate::secrets_always_local_mod::crates_io_mod;
use crate::secrets_always_local_mod::github_mod;

use cargo_auto_github_lib as cgl;
use cargo_auto_lib as cl;

use cl::GREEN;
use cl::RED;
use cl::RESET;
use cl::YELLOW;

// traits must be in scope (Rust strangeness)
use cgl::SendToGitHubApi;
use cl::CargoTomlPublicApiMethods;
use cl::ShellCommandLimitedDoubleQuotesSanitizerTrait;

// region: library with basic automation tasks

fn main() {
    std::panic::set_hook(Box::new(|panic_info| panic_set_hook(panic_info)));
    tracing_init();
    cl::exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: general functions

/// Initialize tracing to file logs/automation_tasks_rs.log
///
/// The folder logs/ is in .gitignore and will not be committed.
pub fn tracing_init() {
    // uncomment this line to enable tracing to file
    // let file_appender = tracing_appender::rolling::daily("logs", "automation_tasks_rs.log");

    let offset = time::UtcOffset::current_local_offset().expect("should get local offset!");
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(offset, time::macros::format_description!("[hour]:[minute]:[second].[subsecond digits:6]"));

    // Filter out logs from: hyper_util, reqwest
    // A filter consists of one or more comma-separated directives
    // target[span{field=value}]=level
    // examples: tokio::net=info
    // directives can be added with the RUST_LOG environment variable:
    // export RUST_LOG=automation_tasks_rs=trace
    // Unset the environment variable RUST_LOG
    // unset RUST_LOG
    let filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive("hyper_util=error".parse().unwrap_or_else(|e| panic!("{e}")))
        .add_directive("reqwest=error".parse().unwrap_or_else(|e| panic!("{e}")));

    tracing_subscriber::fmt()
        .with_file(true)
        .with_max_level(tracing::Level::DEBUG)
        .with_timer(timer)
        .with_line_number(true)
        .with_ansi(false)
        //.with_writer(file_appender)
        .with_env_filter(filter)
        .init();
}

/// The original Rust report of the panic is ugly for the end user
///
/// I use panics extensively to stop the execution. I am lazy to implement a super complicated error handling.
/// I just need to stop the execution on every little bit of error. This utility is for developers. They will understand me.
/// For errors I print the location. If the message contains "Exiting..." than it is a "not-error exit" and  the location is not important.
fn panic_set_hook(panic_info: &std::panic::PanicInfo) {
    let mut string_message = "".to_string();
    if let Some(message) = panic_info.payload().downcast_ref::<String>() {
        string_message = message.to_owned();
    }
    if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
        string_message.push_str(message);
    }

    tracing::debug!("{string_message}");
    eprintln!("{string_message}");

    if !string_message.contains("Exiting...") {
        let file = panic_info.location().unwrap().file();
        let line = panic_info.location().unwrap().line();
        let column = panic_info.location().unwrap().column();
        tracing::debug!("Location: {file}:{line}:{column}");
        eprintln!("Location: {file}:{line}:{column}");
    }
}

// endregion: general functions

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
                println!("{YELLOW}Running automation task: {task}{RESET}");
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
    {YELLOW}For the GitHub API the task needs the Personal Access secret_token Classic from <https://github.com/settings/tokens>{RESET}
    {YELLOW}You can choose to type the secret_token every time or to store it in a file encrypted with an SSH key.{RESET}
    {YELLOW}Then you can type the passphrase of the private key every time. This is pretty secure.{RESET}
    {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}
{GREEN}cargo auto publish_to_crates_io{RESET} - {YELLOW}publish to crates.io, git tag{RESET}
    {YELLOW}You need the API secret_token for publishing. Get the secret_token on <https://crates.io/settings/tokens>.{RESET}
    {YELLOW}You can choose to type the secret_token every time or to store it in a file encrypted with an SSH key.{RESET}
    {YELLOW}Then you can type the passphrase of the private key every time. This is pretty secure.{RESET}
    {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}
{GREEN}cargo auto github_new_release{RESET} - {YELLOW}creates new release on GitHub{RESET}
    {YELLOW}For the GitHub API the task needs the Personal Access secret_token Classic from <https://github.com/settings/tokens>{RESET}
    {YELLOW}You can choose to type the secret_token every time or to store it in a file encrypted with an SSH key.{RESET}
    {YELLOW}Then you can type the passphrase of the private key every time. This is pretty secure.{RESET}
    {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}

    {YELLOW}© 2024 bestia.dev  MIT License github.com/automation-tasks-rs/cargo-auto{RESET}
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

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "release", "doc", "test", "commit_and_push", "publish_to_crates_io", "github_new_release"];
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
    let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::run_shell_command_static("cargo fmt").unwrap_or_else(|e| panic!("{e}"));
    cl::run_shell_command_static("cargo build").unwrap_or_else(|e| panic!("{e}"));
    println!(
        r#"
    {YELLOW}After `cargo auto build`, run the compiled binary, examples and/or tests{RESET}
{GREEN}./target/debug/{package_name} print world{RESET}
    {YELLOW}If ok then{RESET}
{GREEN}./target/debug/{package_name} upper world{RESET}
    {YELLOW}If ok then{RESET}
{GREEN}./target/debug/{package_name} upper WORLD{RESET}
    {YELLOW}if ok then{RESET}
{GREEN}cargo auto release{RESET}
"#,
        package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
}

/// cargo build --release
fn task_release() {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");

    cl::run_shell_command_static("cargo fmt").unwrap_or_else(|e| panic!("{e}"));
    cl::run_shell_command_static("cargo build --release").unwrap_or_else(|e| panic!("{e}"));

    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"strip "target/release/{package_name}" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{package_name}", &cargo_toml.package_name()).unwrap_or_else(|e| panic!("{e}"))
    .run().unwrap_or_else(|e| panic!("{e}"));

    println!(
        r#"
    {YELLOW}After `cargo auto release`, run the compiled binary, examples and/or tests{RESET}
{GREEN}./target/release/{package_name} print world{RESET}
    {YELLOW}If ok then{RESET}
{GREEN}./target/release/{package_name} upper world{RESET}
    {YELLOW}If ok then{RESET}
{GREEN}./target/release/{package_name} upper WORLD{RESET}
    {YELLOW}if ok then{RESET}
{GREEN}cargo auto doc{RESET}
"#,
        package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
}

/// cargo doc, then copies to /docs/ folder, because this is a GitHub standard folder
fn task_doc() {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");
    cl::auto_plantuml(&cargo_toml.package_repository().unwrap());
    cl::auto_playground_run_code();
    cl::auto_md_to_doc_comments();

    cl::run_shell_command_static("cargo doc --no-deps --document-private-items").unwrap_or_else(|e| panic!("{e}"));
    // copy target/doc into docs/ because it is GitHub standard
    cl::run_shell_command_static("rsync -a --info=progress2 --delete-after target/doc/ docs/").unwrap_or_else(|e| panic!("{e}"));

    // Create simple index.html file in docs directory
    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"printf "<meta http-equiv=\"refresh\" content=\"0; url={url_sanitized_for_double_quote}/index.html\" />\n" > docs/index.html"#)
        .unwrap_or_else(|e| panic!("{e}"))
        .arg("{url_sanitized_for_double_quote}", &cargo_toml.package_name().replace("-", "_"))
        .unwrap_or_else(|e| panic!("{e}"))
        .run()
        .unwrap_or_else(|e| panic!("{e}"));

    // pretty html
    cl::auto_doc_tidy_html().unwrap_or_else(|e| panic!("{e}"));
    cl::run_shell_command_static("cargo fmt").unwrap_or_else(|e| panic!("{e}"));
    // message to help user with next move
    println!(
        r#"
    {YELLOW}After `cargo auto doc`, ctrl-click on `docs/index.html`. 
    It will show the index.html in VSCode Explorer, then right-click and choose "Show Preview".
    This works inside the CRUSTDE container, because of the extension "Live Preview" 
    <https://marketplace.visualstudio.com/items?itemName=ms-vscode.live-server>
    If ok then run the tests in code and the documentation code examples.{RESET}
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
    let Some(message) = arg_2 else {
        eprintln!("{RED}Error: Message for commit is mandatory.{RESET}");
        // early exit
        return;
    };

    // If needed, ask to create new local git repository
    if !cl::git_is_local_repository() {
        cl::new_local_repository(&message).unwrap();
    }

    // If needed, ask to create a GitHub remote repository
    if !cgl::git_has_remote() || !cgl::git_has_upstream() {
        let github_client = github_mod::GitHubClient::new_with_stored_secret_token();
        cgl::new_remote_github_repository(&github_client).unwrap();
        cgl::description_and_topics_to_github(&github_client);
    } else {
        let github_client = github_mod::GitHubClient::new_with_stored_secret_token();
        // if description or topics/keywords/tags have changed
        cgl::description_and_topics_to_github(&github_client);

        // separate commit for docs if they changed, to not make a lot of noise in the real commit
        if std::path::Path::new("docs").exists() {
            cl::run_shell_command_static(r#"git add docs && git diff --staged --quiet || git commit -m "update docs" "#).unwrap_or_else(|e| panic!("{e}"));
        }

        cl::add_message_to_unreleased(&message);
        // the real commit of code
        cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"git add -A && git diff --staged --quiet || git commit -m "{message_sanitized_for_double_quote}" "#)
            .unwrap_or_else(|e| panic!("{e}"))
            .arg("{message_sanitized_for_double_quote}", &message)
            .unwrap_or_else(|e| panic!("{e}"))
            .run()
            .unwrap_or_else(|e| panic!("{e}"));

        cl::run_shell_command_static("git push").unwrap_or_else(|e| panic!("{e}"));
    }

    println!(
        r#"
    {YELLOW}After `cargo auto commit_and_push "message"`{RESET}
{GREEN}cargo auto publish_to_crates_io{RESET}
"#
    );
}

/// publish to crates.io and git tag
fn task_publish_to_crates_io() {
    let cargo_toml = cl::CargoToml::read();
    let package_name = cargo_toml.package_name();
    let version = cargo_toml.package_version();
    // take care of tags
    let tag_name_version = cl::git_tag_sync_check_create_push(&version);

    // cargo publish with encrypted secret secret_token
    let crates_io_client = crates_io_mod::CratesIoClient::new_with_stored_secret_token();
    crates_io_client.publish_to_crates_io();

    println!(
        r#"
    {YELLOW}After `cargo auto publish_to_crates_io`, check in browser{RESET}
{GREEN}https://crates.io/crates/{package_name}{RESET}
    {YELLOW}Add the dependency to your Rust project and check how it works.{RESET}
{GREEN}{package_name} = "{version}"{RESET}

    {YELLOW}First write the content of the release in the RELEASES.md in the `## Unreleased` section, then{RESET}
    {YELLOW}Then create the GitHub-Release for {tag_name_version}.{RESET}
{GREEN}cargo auto github_new_release{RESET}
"#
    );
}

/// create a new release on github
fn task_github_new_release() {
    let cargo_toml = cl::CargoToml::read();
    let version = cargo_toml.package_version();
    // take care of tags
    let tag_name_version = cl::git_tag_sync_check_create_push(&version);

    let github_owner = cargo_toml.github_owner().unwrap();
    let repo_name = cargo_toml.package_name();
    let now_date = cl::now_utc_date_iso();
    let release_name = format!("Version {} ({})", &version, now_date);
    let branch = "main";

    // First, the user must write the content into file RELEASES.md in the section ## Unreleased.
    // Then the automation task will copy the content to GitHub release
    let body_md_text = cl::body_text_from_releases_md().unwrap();

    let github_client = github_mod::GitHubClient::new_with_stored_secret_token();
    let json_value = github_client.send_to_github_api(cgl::github_api_create_new_release(&github_owner, &repo_name, &tag_name_version, &release_name, branch, &body_md_text));
    // early exit on error
    if let Some(error_message) = json_value.get("message") {
        eprintln!("{RED}{error_message}{RESET}");
        if let Some(errors) = json_value.get("errors") {
            let errors = errors.as_array().unwrap();
            for error in errors.iter() {
                if let Some(code) = error.get("code") {
                    eprintln!("{RED}{code}{RESET}");
                }
            }
        }
        panic!("{RED}Call to GitHub API returned an error.{RESET}")
    }

    // Create a new Version title in RELEASES.md.
    cl::create_new_version_in_releases_md(&release_name).unwrap();

    println!(
        "
    {YELLOW}New GitHub release created: {release_name}.{RESET}
"
    );

    // region: upload asset only for executables, not for libraries

    let release_id = json_value.get("id").unwrap().as_i64().unwrap().to_string();
    println!(
        "
        {YELLOW}Now uploading release asset. This can take some time if the files are big. Wait...{RESET}
    "
    );
    // compress files tar.gz
    let tar_name = format!("{repo_name}-{tag_name_version}-x86_64-unknown-linux-gnu.tar.gz");

    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
        r#"tar -zcvf "{tar_name_sanitized_for_double_quote}" "target/release/{repo_name_sanitized_for_double_quote}" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{tar_name_sanitized_for_double_quote}", &tar_name).unwrap_or_else(|e| panic!("{e}"))
    .arg("{repo_name_sanitized_for_double_quote}", &repo_name).unwrap_or_else(|e| panic!("{e}"))
    .run().unwrap_or_else(|e| panic!("{e}"));

    // upload asset
    cgl::github_api_upload_asset_to_release(&github_client, &github_owner, &repo_name, &release_id, &tar_name);

    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
        r#"rm "{tar_name_sanitized_for_double_quote}" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{tar_name_sanitized_for_double_quote}", &tar_name).unwrap_or_else(|e| panic!("{e}"))
    .run().unwrap_or_else(|e| panic!("{e}"));

    println!(
        r#"
    {YELLOW}Asset uploaded. Open and edit the description on GitHub Releases in the browser.{RESET}
    "#
    );

    // endregion: upload asset only for executables, not for libraries

    println!(
        r#"
{GREEN}https://github.com/{github_owner}/{repo_name}/releases{RESET}
    "#
    );
}
// endregion: tasks
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/secrets_always_local_mod.rs",
        file_content: r###"// secrets_always_local_mod.rs

/// Secrets like GitHub API secret_token, crates.io secret token, docker hub secret_token, SSH private key passphrase and similar
/// must never go out of this crate. Never pass any secret to an external crate library as much as possible.
/// The user has the source code under his fingers in this crate. So he knows nobody will mess with this code
/// once he inspected and reviewed it.
/// All the modules are in one file to avoid clutter in the automation_tasks_rs folder.
/// The simple program flow of functions that need secrets is butchered to avoid secrets leaving this crate.
/// Now it looks like a mess, but the goal is achieved. The secrets never leave this crate.

// region: Public API constants
// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
/// ANSI color
pub const RED: &str = "\x1b[31m";
/// ANSI color
pub const GREEN: &str = "\x1b[32m";
/// ANSI color
pub const YELLOW: &str = "\x1b[33m";
/// ANSI color
pub const BLUE: &str = "\x1b[34m";
/// ANSI color
pub const RESET: &str = "\x1b[0m";
// endregion: Public API constants

pub use cargo_auto_encrypt_secret_lib::EncryptedString;
pub use secrecy::ExposeSecret;

pub(crate) mod decrypt_mod {

    use crate::secrets_always_local_mod::*;

    /// The secrets must not leave this crate.
    /// They are never going into an external library crate.
    /// This crate is "user code" and is easy to review and inspect.
    pub(crate) struct Decryptor<'a> {
        secret_string: secrecy::SecretString,
        secret_passcode_bytes: &'a secrecy::SecretVec<u8>,
    }

    impl<'a> Decryptor<'a> {
        pub(crate) fn new_for_decrypt(secret_passcode_bytes: &'a secrecy::SecretVec<u8>) -> Self {
            Decryptor {
                secret_string: secrecy::SecretString::new("".to_string()),
                secret_passcode_bytes,
            }
        }
        pub(crate) fn return_secret_string(&self) -> &secrecy::SecretString {
            &self.secret_string
        }

        /// Decrypts encrypted_string with secret_passcode_bytes
        ///
        /// secret_passcode_bytes must be 32 bytes or more
        /// Returns the secret_string
        pub(crate) fn decrypt_symmetric(&mut self, encrypted_string: &cargo_auto_encrypt_secret_lib::EncryptedString) {
            let encrypted_bytes = <base64ct::Base64 as base64ct::Encoding>::decode_vec(&encrypted_string.0).unwrap();
            //only first 32 bytes
            let mut secret_passcode_32bytes = [0u8; 32];
            secret_passcode_32bytes.copy_from_slice(&self.secret_passcode_bytes.expose_secret()[0..32]);

            let cipher = <aes_gcm::Aes256Gcm as aes_gcm::KeyInit>::new(&secret_passcode_32bytes.into());
            // nonce is salt
            let nonce = rsa::sha2::digest::generic_array::GenericArray::from_slice(&encrypted_bytes[..12]);
            let cipher_text = &encrypted_bytes[12..];

            let Ok(decrypted_bytes) = aes_gcm::aead::Aead::decrypt(&cipher, nonce, cipher_text) else {
                panic!("{RED}Error: Decryption failed. {RESET}");
            };
            let decrypted_string = String::from_utf8(decrypted_bytes).unwrap();
            self.secret_string = secrecy::SecretString::new(decrypted_string)
        }
    }
}

pub(crate) mod encrypt_mod {
    use crate::secrets_always_local_mod::*;

    /// The secrets must not leave this crate.
    /// They are never going into an external library crate.
    /// This crate is "user code" and is easy to review and inspect.
    pub(crate) struct Encryptor<'a> {
        secret_string: secrecy::SecretString,
        secret_passcode_bytes: &'a secrecy::SecretVec<u8>,
    }

    impl<'a> Encryptor<'a> {
        pub(crate) fn new_for_encrypt(secret_string: secrecy::SecretString, secret_passcode_bytes: &'a secrecy::SecretVec<u8>) -> Self {
            Encryptor { secret_string, secret_passcode_bytes }
        }

        /// Encrypts secret_string with secret_passcode_bytes
        ///
        /// secret_passcode_bytes must be 32 bytes or more
        /// returns the encrypted_string
        pub(crate) fn encrypt_symmetric(&self) -> Option<cargo_auto_encrypt_secret_lib::EncryptedString> {
            //only first 32 bytes
            let mut secret_passcode_32bytes = [0u8; 32];
            secret_passcode_32bytes.copy_from_slice(&self.secret_passcode_bytes.expose_secret()[0..32]);

            let cipher = <aes_gcm::Aes256Gcm as aes_gcm::KeyInit>::new(&secret_passcode_32bytes.into());
            // nonce is salt
            let nonce = <aes_gcm::Aes256Gcm as aes_gcm::AeadCore>::generate_nonce(&mut aes_gcm::aead::OsRng);

            let Ok(cipher_text) = aes_gcm::aead::Aead::encrypt(&cipher, &nonce, self.secret_string.expose_secret().as_bytes()) else {
                panic!("{RED}Error: Encryption failed. {RESET}");
            };

            let mut encrypted_bytes = nonce.to_vec();
            encrypted_bytes.extend_from_slice(&cipher_text);
            let encrypted_string = <base64ct::Base64 as base64ct::Encoding>::encode_string(&encrypted_bytes);
            Some(cargo_auto_encrypt_secret_lib::EncryptedString(encrypted_string))
        }
    }
}

pub(crate) mod secrecy_mod {

    //! The crate secrecy is probably great.
    //! But I want to encrypt the content, so I will make a wrapper.
    //! The secrets must always be moved to secrecy types as soon as possible.

    use crate::secrets_always_local_mod::*;

    pub struct SecretEncryptedString {
        encrypted_string: EncryptedString,
    }

    impl SecretEncryptedString {
        pub fn new_with_secret_string(secret_string: secrecy::SecretString, session_passcode: &secrecy::SecretVec<u8>) -> Self {
            let encryptor = super::encrypt_mod::Encryptor::new_for_encrypt(secret_string, &session_passcode);
            let encrypted_string = encryptor.encrypt_symmetric().unwrap();

            SecretEncryptedString { encrypted_string }
        }

        pub fn new_with_string(secret_string: String, session_passcode: &secrecy::SecretVec<u8>) -> Self {
            let secret_string = secrecy::SecretString::new(secret_string);
            Self::new_with_secret_string(secret_string, session_passcode)
        }

        pub fn expose_decrypted_secret(&self, session_passcode: &secrecy::SecretVec<u8>) -> secrecy::SecretString {
            let mut decryptor = super::decrypt_mod::Decryptor::new_for_decrypt(&session_passcode);
            decryptor.decrypt_symmetric(&self.encrypted_string);
            decryptor.return_secret_string().clone()
        }
    }
}

pub(crate) mod ssh_mod {

    use crate::secrets_always_local_mod::*;

    pub struct SshContext {
        signed_passcode_is_a_secret: secrecy::SecretVec<u8>,
        decrypted_string: secrecy::SecretString,
    }

    impl SshContext {
        pub fn new() -> Self {
            SshContext {
                signed_passcode_is_a_secret: secrecy::SecretVec::new(vec![]),
                decrypted_string: secrecy::SecretString::new("".to_string()),
            }
        }
        pub fn get_decrypted_string(&self) -> secrecy::SecretString {
            self.decrypted_string.clone()
        }
    }

    impl cargo_auto_encrypt_secret_lib::SshContextTrait for SshContext {
        /// decrypt from file data and write the decrypted secret in private field for later use in this crate, not in external library crates
        fn decrypt_from_file_data(&mut self, encrypted_string: &cargo_auto_encrypt_secret_lib::EncryptedString) {
            let mut decryptor = decrypt_mod::Decryptor::new_for_decrypt(&self.signed_passcode_is_a_secret);
            decryptor.decrypt_symmetric(encrypted_string);
            self.decrypted_string = decryptor.return_secret_string().clone();
        }

        /// get secret_token and encrypt
        fn get_secret_token_and_encrypt(&self) -> cargo_auto_encrypt_secret_lib::EncryptedString {
            /// Internal function used only for test configuration
            ///
            /// It is not interactive, but reads from a env var.
            #[cfg(test)]
            fn get_secret_token() -> secrecy::SecretString {
                secrecy::SecretString::new(std::env::var("TEST_TOKEN").unwrap())
            }
            /// Internal function get_passphrase interactively ask user to type the passphrase
            ///
            /// This is used for normal code execution.
            #[cfg(not(test))]
            fn get_secret_token() -> secrecy::SecretString {
                eprintln!(" ");
                eprintln!("   {BLUE}Enter the secret_token to encrypt:{RESET}");
                secrecy::SecretString::new(
                    inquire::Password::new("")
                        .without_confirmation()
                        .with_display_mode(inquire::PasswordDisplayMode::Masked)
                        .prompt()
                        .unwrap(),
                )
            }
            let secret_token = get_secret_token();
            // use this signed as password for symmetric encryption
            let encryptor = encrypt_mod::Encryptor::new_for_encrypt(secret_token, &self.signed_passcode_is_a_secret);

            let encrypted_token = encryptor.encrypt_symmetric().unwrap();
            // return
            encrypted_token
        }

        /// Sign with ssh-agent or with identity_file
        ///
        /// get passphrase interactively
        /// returns secret_password_bytes:Vec u8
        fn sign_with_ssh_agent_or_identity_file(&mut self, identity_private_file_path: &camino::Utf8Path, seed_bytes_not_a_secret: &[u8; 32]) {
            /// Internal function used only for test configuration
            ///
            /// It is not interactive, but reads from a env var.
            #[cfg(test)]
            fn get_passphrase() -> secrecy::SecretString {
                secrecy::SecretString::new(std::env::var("TEST_PASSPHRASE").unwrap())
            }
            /// Internal function get_passphrase interactively ask user to type the passphrase
            ///
            /// This is used for normal code execution.
            #[cfg(not(test))]
            fn get_passphrase() -> secrecy::SecretString {
                eprintln!(" ");
                eprintln!("   {BLUE}Enter the passphrase for the SSH private key:{RESET}");
                secrecy::SecretString::new(
                    inquire::Password::new("")
                        .without_confirmation()
                        .with_display_mode(inquire::PasswordDisplayMode::Masked)
                        .prompt()
                        .unwrap(),
                )
            }

            let identity_private_file_path_expanded = expand_path_check_private_key_exists(identity_private_file_path);

            let fingerprint_from_file = cargo_auto_encrypt_secret_lib::get_fingerprint_from_file(&identity_private_file_path_expanded);

            let mut ssh_agent_client = cargo_auto_encrypt_secret_lib::crate_ssh_agent_client();
            match cargo_auto_encrypt_secret_lib::ssh_add_list_contains_fingerprint(&mut ssh_agent_client, &fingerprint_from_file) {
                Some(public_key) => {
                    // sign with public key from ssh-agent
                    let signature_is_the_new_secret_password = ssh_agent_client.sign(&public_key, seed_bytes_not_a_secret).unwrap();
                    // only the data part of the signature goes into as_bytes.
                    self.signed_passcode_is_a_secret = secrecy::SecretVec::new(signature_is_the_new_secret_password.as_bytes().to_owned());
                }
                None => {
                    // ask user to think about adding with ssh-add
                    eprintln!("   {YELLOW}SSH key for encrypted secret_token is not found in the ssh-agent.{RESET}");
                    eprintln!("   {YELLOW}Without ssh-agent, you will have to type the private key passphrase every time. This is more secure, but inconvenient.{RESET}");
                    eprintln!("   {YELLOW}You can manually add the SSH identity to ssh-agent for 1 hour:{RESET}");
                    eprintln!("   {YELLOW}WARNING: using ssh-agent is less secure, because there is no need for user interaction.{RESET}");
                    eprintln!("{GREEN}ssh-add -t 1h {identity_private_file_path_expanded}{RESET}");

                    // just for test purpose I will use env var to read this passphrase. Don't use it in production.

                    let passphrase_is_a_secret = get_passphrase();
                    let private_key = ssh_key::PrivateKey::read_openssh_file(identity_private_file_path_expanded.as_std_path()).unwrap();
                    let mut private_key = private_key.decrypt(passphrase_is_a_secret.expose_secret()).unwrap();

                    // FYI: this type of signature is compatible with ssh-agent because it does not involve namespace
                    let signature_is_the_new_secret_password = rsa::signature::SignerMut::try_sign(&mut private_key, seed_bytes_not_a_secret).unwrap();

                    // only the data part of the signature goes into as_bytes.
                    self.signed_passcode_is_a_secret = secrecy::SecretVec::new(signature_is_the_new_secret_password.as_bytes().to_owned());
                }
            }
        }
    }

    /// Expand path and check if identity file exists
    ///
    /// Inform the user how to generate identity file.
    pub fn expand_path_check_private_key_exists(identity_private_file_path: &camino::Utf8Path) -> camino::Utf8PathBuf {
        let identity_private_file_path_expanded = cargo_auto_encrypt_secret_lib::file_path_home_expand(identity_private_file_path);
        if !camino::Utf8Path::new(&identity_private_file_path_expanded).exists() {
            eprintln!("{RED}Identity file {identity_private_file_path_expanded} that contains the SSH private key does not exist! {RESET}");
            eprintln!("    {YELLOW}Create the SSH key manually in bash with this command:{RESET}");
            if identity_private_file_path_expanded.as_str().contains("github_api") {
                eprintln!(r#"{GREEN}ssh-keygen -t ed25519 -f "{identity_private_file_path_expanded}" -C "github api secret_token"{RESET}"#);
            } else if identity_private_file_path_expanded.as_str().contains("crates_io") {
                eprintln!(r#"{GREEN}ssh-keygen -t ed25519 -f "{identity_private_file_path_expanded}" -C "crates io secret_token"{RESET}"#);
            } else if identity_private_file_path_expanded.as_str().contains("docker_hub") {
                eprintln!(r#"{GREEN}ssh-keygen -t ed25519 -f "{identity_private_file_path_expanded}" -C "docker hub secret_token"{RESET}"#);
            }
            eprintln!(" ");
            panic!("{RED}Error: File {identity_private_file_path_expanded} does not exist! {RESET}");
        }
        identity_private_file_path_expanded
    }
}

pub(crate) mod github_mod {

    //! Every API call needs the GitHub API secret_token. This is a secret important just like a password.
    //! I don't want to pass this secret to an "obscure" library crate that is difficult to review.
    //! This secret will stay here in this codebase that every developer can easily inspect.
    //! Instead of the secret_token, I will pass the struct GitHubClient with the trait SendToGitHubApi.
    //! This way, the secret_token will be encapsulated.

    use crate::secrets_always_local_mod::*;
    use cargo_auto_github_lib as cgl;
    use reqwest::Client;

    /// Struct GitHubClient contains only private fields
    /// This fields are accessible only to methods in implementation of traits.
    pub struct GitHubClient {
        /// Passcode for encrypt the secret_token to encrypted_token in memory.
        /// So that the secret is in memory as little as possible as plain text.
        /// For every session (program start) a new random passcode is created.
        session_passcode: secrecy::SecretVec<u8>,

        /// private field is set only once in the new() constructor
        encrypted_token: super::secrecy_mod::SecretEncryptedString,
    }

    impl GitHubClient {
        /// Create new GitHub client
        ///
        /// Interactively ask the user to input the GitHub secret_token.
        pub fn new_interactive_input_secret_token() -> Self {
            let mut github_client = Self::new_wo_secret_token();

            println!("{BLUE}Enter the GitHub API secret_token:{RESET}");
            github_client.encrypted_token =
                super::secrecy_mod::SecretEncryptedString::new_with_string(inquire::Password::new("").without_confirmation().prompt().unwrap(), &github_client.session_passcode);

            // return
            github_client
        }

        /// Create new GitHub client without secret_token
        fn new_wo_secret_token() -> Self {
            /// Internal function Generate a random password
            fn random_byte_passcode() -> [u8; 32] {
                let mut password = [0_u8; 32];
                use aes_gcm::aead::rand_core::RngCore;
                aes_gcm::aead::OsRng.fill_bytes(&mut password);
                password
            }

            let session_passcode = secrecy::SecretVec::new(random_byte_passcode().to_vec());
            let encrypted_token = super::secrecy_mod::SecretEncryptedString::new_with_string("".to_string(), &session_passcode);

            GitHubClient { session_passcode, encrypted_token }
        }

        /// Use the stored API secret_token
        ///
        /// If the secret_token not exists ask user to interactively input the secret_token.
        /// To decrypt it, use the SSH passphrase. That is much easier to type than typing the secret_token.
        /// it is then possible also to have the ssh key in ssh-agent and write the passphrase only once.
        /// But this great user experience comes with security concerns. The secret_token is accessible if the attacker is very dedicated.
        pub fn new_with_stored_secret_token() -> Self {
            /// Internal function for DRY Don't Repeat Yourself
            fn read_secret_token_and_decrypt_return_github_client(mut ssh_context: super::ssh_mod::SshContext, encrypted_string_file_path: &camino::Utf8Path) -> GitHubClient {
                // read the secret_token and decrypt
                cargo_auto_encrypt_secret_lib::decrypt_with_ssh_interactive_from_file(&mut ssh_context, encrypted_string_file_path);
                let secret_token = ssh_context.get_decrypted_string();
                let mut github_client = GitHubClient::new_wo_secret_token();
                github_client.encrypted_token = super::secrecy_mod::SecretEncryptedString::new_with_secret_string(secret_token, &github_client.session_passcode);
                github_client
            }

            let encrypted_string_file_path = camino::Utf8Path::new("~/.ssh/github_api_secret_token_encrypted.txt");
            let encrypted_string_file_path_expanded = cargo_auto_encrypt_secret_lib::file_path_home_expand(encrypted_string_file_path);

            let identity_private_file_path = camino::Utf8Path::new("~/.ssh/github_api_secret_token_ssh_1");
            let _identity_private_file_path_expanded = crate::secrets_always_local_mod::ssh_mod::expand_path_check_private_key_exists(identity_private_file_path);

            if !encrypted_string_file_path_expanded.exists() {
                // ask interactive
                println!("    {BLUE}Do you want to store the GitHub API secret_token encrypted with an SSH key? (y/n){RESET}");
                let answer = inquire::Text::new("").prompt().unwrap();
                if answer.to_lowercase() != "y" {
                    // enter the secret_token manually, not storing
                    return Self::new_interactive_input_secret_token();
                } else {
                    // get the passphrase and secret_token interactively
                    let mut ssh_context = super::ssh_mod::SshContext::new();
                    // encrypt and save the encrypted secret_token
                    cargo_auto_encrypt_secret_lib::encrypt_with_ssh_interactive_save_file(&mut ssh_context, identity_private_file_path, encrypted_string_file_path);
                    // read the secret_token and decrypt, return GitHubClient
                    read_secret_token_and_decrypt_return_github_client(ssh_context, encrypted_string_file_path)
                }
            } else {
                // file exists
                let ssh_context = super::ssh_mod::SshContext::new();
                // read the secret_token and decrypt, return GitHubClient
                read_secret_token_and_decrypt_return_github_client(ssh_context, encrypted_string_file_path)
            }
        }

        /// decrypts the secret_token in memory
        #[allow(dead_code)]
        pub fn decrypt_secret_token_in_memory(&self) -> secrecy::SecretString {
            self.encrypted_token.expose_decrypted_secret(&self.session_passcode)
        }
    }

    /// trait from the crate library, so the 2 crates can share a function
    impl cgl::SendToGitHubApi for GitHubClient {
        /// Send GitHub API request
        ///
        /// This function encapsulates the secret API secret_token.
        /// The RequestBuilder is created somewhere in the library crate.
        /// The client can be passed to the library. It will not reveal the secret_token.
        fn send_to_github_api(&self, req: reqwest::blocking::RequestBuilder) -> serde_json::Value {
            // I must build the request to be able then to inspect it.
            let req = req.bearer_auth(self.decrypt_secret_token_in_memory().expose_secret()).build().unwrap();

            // region: Assert the correct url and https
            // It is important that the request coming from a external crate/library
            // is only sent always and only to GitHub API and not some other malicious url,
            // because the request contains the secret GitHub API secret_token.
            // And it must always use https
            let host_str = req.url().host_str().unwrap();
            assert!(host_str == "api.github.com", "{RED}Error: Url is not correct: {host_str}. It must be always api.github.com.{RESET}");
            let scheme = req.url().scheme();
            assert!(scheme == "https", "{RED}Error: Scheme is not correct: {scheme}. It must be always https.{RESET}");
            // endregion: Assert the correct url and https

            let reqwest_client = reqwest::blocking::Client::new();
            let response_text = reqwest_client.execute(req).unwrap().text().unwrap();

            let json_value: serde_json::Value = serde_json::from_str(&response_text).unwrap();

            // panic if "message": String("Bad credentials"),
            if let Some(m) = json_value.get("message") {
                if m == "Bad credentials" {
                    panic!("{RED}Error: Bad credentials for GitHub API. {RESET}");
                }
            }

            // return
            json_value
        }

        /// Upload to GitHub
        ///
        /// This function encapsulates the secret API secret_token.
        /// The RequestBuilder is created somewhere in the library crate.
        /// The client can be passed to the library. It will not reveal the secret_token.
        /// This is basically an async fn, but use of `async fn` in public traits is discouraged...
        async fn upload_to_github(&self, req: reqwest::RequestBuilder) -> serde_json::Value {
            // I must build the request to be able then to inspect it.
            let req = req.bearer_auth(self.decrypt_secret_token_in_memory().expose_secret()).build().unwrap();

            // region: Assert the correct url and https
            // It is important that the request coming from a external crate/library
            // is only sent always and only to GitHub uploads and not some other malicious url,
            // because the request contains the secret GitHub API secret_token.
            // And it must always use https
            let host_str = req.url().host_str().unwrap();
            assert!(host_str == "uploads.github.com", "{RED}Error: Url is not correct: {host_str}. It must be always api.github.com.{RESET}");
            let scheme = req.url().scheme();
            assert!(scheme == "https", "{RED}Error: Scheme is not correct: {scheme}. It must be always https.{RESET}");
            // endregion: Assert the correct url and https

            let reqwest_client = Client::new();
            let response_text = reqwest_client.execute(req).await.unwrap().text().await.unwrap();

            let json_value: serde_json::Value = serde_json::from_str(&response_text).unwrap();

            // panic if "message": String("Bad credentials"),
            if let Some(m) = json_value.get("message") {
                if m == "Bad credentials" {
                    panic!("{RED}Error: Bad credentials for GitHub API. {RESET}");
                }
            }

            // return
            json_value
        }
    }
}

pub(crate) mod crates_io_mod {

    //! Publish to crates.io needs the crates.io secret_token. This is a secret important just like a password.
    //! I don't want to pass this secret to an "obscure" library crate that is difficult to review.
    //! This secret will stay here in this codebase that every developer can easily inspect.
    //! Instead of the secret_token, I will pass the struct CratesIoClient with the trait SendToCratesIo.
    //! This way, the secret_token will be encapsulated.

    use crate::secrets_always_local_mod::*;
    use cargo_auto_lib::ShellCommandLimitedDoubleQuotesSanitizer;
    use cargo_auto_lib::ShellCommandLimitedDoubleQuotesSanitizerTrait;

    /// Struct CratesIoClient contains only private fields
    /// This fields are accessible only to methods in implementation of traits.
    pub struct CratesIoClient {
        /// Passcode for encrypt the secret_token to encrypted_token in memory.
        /// So that the secret is in memory as little as possible as plain text.
        /// For every session (program start) a new random passcode is created.
        session_passcode: secrecy::SecretVec<u8>,

        /// private field is set only once in the new() constructor
        encrypted_token: super::secrecy_mod::SecretEncryptedString,
    }

    impl CratesIoClient {
        /// Create new CratesIo client
        ///
        /// Interactively ask the user to input the crates.io secret_token.
        #[allow(dead_code)]
        pub fn new_interactive_input_secret_token() -> Self {
            let mut crates_io_client = Self::new_wo_secret_token();

            println!("{BLUE}Enter the crates.io secret_token:{RESET}");
            crates_io_client.encrypted_token =
                super::secrecy_mod::SecretEncryptedString::new_with_string(inquire::Password::new("").without_confirmation().prompt().unwrap(), &crates_io_client.session_passcode);

            // return
            crates_io_client
        }

        /// Create new CratesIo client without secret_token
        #[allow(dead_code)]
        fn new_wo_secret_token() -> Self {
            /// Internal function Generate a random password
            fn random_byte_passcode() -> [u8; 32] {
                let mut password = [0_u8; 32];
                use aes_gcm::aead::rand_core::RngCore;
                aes_gcm::aead::OsRng.fill_bytes(&mut password);
                password
            }

            let session_passcode = secrecy::SecretVec::new(random_byte_passcode().to_vec());
            let encrypted_token = super::secrecy_mod::SecretEncryptedString::new_with_string("".to_string(), &session_passcode);

            CratesIoClient { session_passcode, encrypted_token }
        }

        /// Use the stored crates.io secret_token
        ///
        /// If the secret_token not exists ask user to interactively input the secret_token.
        /// To decrypt it, use the SSH passphrase. That is much easier to type than typing the secret_token.
        /// It is then possible also to have the ssh key in ssh-agent and write the passphrase only once.
        /// But this great user experience comes with security concerns. The secret_token is accessible if the attacker is very dedicated.
        #[allow(dead_code)]
        pub fn new_with_stored_secret_token() -> Self {
            /// Internal function for DRY Don't Repeat Yourself
            fn read_secret_token_and_decrypt_return_crates_io_client(mut ssh_context: super::ssh_mod::SshContext, encrypted_string_file_path: &camino::Utf8Path) -> CratesIoClient {
                cargo_auto_encrypt_secret_lib::decrypt_with_ssh_interactive_from_file(&mut ssh_context, encrypted_string_file_path);
                let secret_token = ssh_context.get_decrypted_string();
                let mut crates_io_client = CratesIoClient::new_wo_secret_token();
                crates_io_client.encrypted_token = super::secrecy_mod::SecretEncryptedString::new_with_secret_string(secret_token, &crates_io_client.session_passcode);
                crates_io_client
            }

            // check if the plain-text file from `podman login` exists and warn the user because it is a security vulnerability.
            let file_auth = "~/.cargo/credentials.toml";
            let file_auth = camino::Utf8Path::new(file_auth);
            // TODO: check for env variable also?
            let file_auth_expanded = cargo_auto_encrypt_secret_lib::file_path_home_expand(file_auth);
            let file_auth_expanded = camino::Utf8Path::new(&file_auth_expanded);
            if file_auth_expanded.exists() {
                eprintln!("{RED}Security vulnerability: Found the cargo file with plain-text secret_token: {file_auth_expanded}. It would be better to inspect and remove it. {RESET}")
            }

            let encrypted_string_file_path = camino::Utf8Path::new("~/.ssh/crates_io_secret_token_encrypted.txt");
            let encrypted_string_file_path_expanded = cargo_auto_encrypt_secret_lib::file_path_home_expand(encrypted_string_file_path);

            let identity_private_file_path = camino::Utf8Path::new("~/.ssh/crates_io_secret_token_ssh_1");
            let _identity_private_file_path_expanded = crate::secrets_always_local_mod::ssh_mod::expand_path_check_private_key_exists(identity_private_file_path);

            if !encrypted_string_file_path_expanded.exists() {
                // ask interactive
                println!("    {BLUE}Do you want to store the crates.io secret_token encrypted with an SSH key? (y/n){RESET}");
                let answer = inquire::Text::new("").prompt().unwrap();
                if answer.to_lowercase() != "y" {
                    // enter the secret_token manually, not storing
                    return Self::new_interactive_input_secret_token();
                } else {
                    // get the passphrase and secret_token interactively
                    let mut ssh_context = super::ssh_mod::SshContext::new();
                    // encrypt and save the encrypted secret_token
                    cargo_auto_encrypt_secret_lib::encrypt_with_ssh_interactive_save_file(&mut ssh_context, identity_private_file_path, encrypted_string_file_path);
                    // read the secret_token and decrypt, return CratesIoClient
                    read_secret_token_and_decrypt_return_crates_io_client(ssh_context, encrypted_string_file_path)
                }
            } else {
                // file exists
                let ssh_context = super::ssh_mod::SshContext::new();
                // read the secret_token and decrypt, return CratesIoClient
                read_secret_token_and_decrypt_return_crates_io_client(ssh_context, encrypted_string_file_path)
            }
        }

        /// decrypts the secret_token in memory
        #[allow(dead_code)]
        pub fn decrypt_secret_token_in_memory(&self) -> secrecy::SecretString {
            self.encrypted_token.expose_decrypted_secret(&self.session_passcode)
        }

        /// Publish to crates.io
        ///
        /// This function encapsulates the secret crates.io secret_token.
        /// The client can be passed to the library. It will not reveal the secret_token.
        #[allow(dead_code)]
        pub fn publish_to_crates_io(&self) {
            // the secret_token is redacted when print on screen
            ShellCommandLimitedDoubleQuotesSanitizer::new(r#"cargo publish --token "{secret_token}" "#)
                .unwrap_or_else(|e| panic!("{e}"))
                .arg_secret("{secret_token}", &self.decrypt_secret_token_in_memory())
                .unwrap_or_else(|e| panic!("{e}"))
                .run()
                .unwrap_or_else(|e| panic!("{e}"));
        }
    }
}

pub(crate) mod docker_hub_mod {

    //! Push to docker-hub needs the docker hub secret_token. This is a secret important just like a password.
    //! I don't want to pass this secret to an "obscure" library crate that is difficult to review.
    //! This secret will stay here in this codebase that every developer can easily inspect.
    //! Instead of the secret_token, I will pass the struct DockerHubClient with the trait SendToDockerHub.
    //! This way, the secret_token will be encapsulated.

    use crate::secrets_always_local_mod::*;
    use cargo_auto_lib::ShellCommandLimitedDoubleQuotesSanitizer;
    use cargo_auto_lib::ShellCommandLimitedDoubleQuotesSanitizerTrait;

    /// Struct DockerHubClient contains only private fields
    /// This fields are accessible only to methods in implementation of traits.
    pub struct DockerHubClient {
        /// Passcode for encrypt the secret_token to encrypted_token in memory.
        /// So that the secret is in memory as little as possible as plain text.
        /// For every session (program start) a new random passcode is created.
        session_passcode: secrecy::SecretVec<u8>,

        /// private field is set only once in the new() constructor
        encrypted_token: super::secrecy_mod::SecretEncryptedString,
    }

    impl DockerHubClient {
        /// Create new DockerHub client
        ///
        /// Interactively ask the user to input the docker hub secret_token.
        #[allow(dead_code)]
        pub fn new_interactive_input_secret_token() -> Self {
            let mut docker_hub_client = Self::new_wo_secret_token();

            println!("{BLUE}Enter the docker hub secret_token:{RESET}");
            docker_hub_client.encrypted_token =
                super::secrecy_mod::SecretEncryptedString::new_with_string(inquire::Password::new("").without_confirmation().prompt().unwrap(), &docker_hub_client.session_passcode);

            // return
            docker_hub_client
        }

        /// Create new DockerHub client without secret_token
        #[allow(dead_code)]
        fn new_wo_secret_token() -> Self {
            /// Internal function Generate a random password
            fn random_byte_passcode() -> [u8; 32] {
                let mut password = [0_u8; 32];
                use aes_gcm::aead::rand_core::RngCore;
                aes_gcm::aead::OsRng.fill_bytes(&mut password);
                password
            }

            let session_passcode = secrecy::SecretVec::new(random_byte_passcode().to_vec());
            let encrypted_token = super::secrecy_mod::SecretEncryptedString::new_with_string("".to_string(), &session_passcode);

            DockerHubClient { session_passcode, encrypted_token }
        }

        /// Use the stored docker hub secret_token
        ///
        /// If the secret_token not exists ask user to interactively input the secret_token.
        /// To decrypt it, use the SSH passphrase. That is much easier to type than typing the secret_token.
        /// It is then possible also to have the ssh key in ssh-agent and write the passphrase only once.
        /// But this great user experience comes with security concerns. The secret_token is accessible if the attacker is very dedicated.
        #[allow(dead_code)]
        pub fn new_with_stored_secret_token(user_name: &str, registry: &str) -> Self {
            /// Internal function for DRY Don't Repeat Yourself
            fn read_secret_token_and_decrypt_return_docker_hub_client(mut ssh_context: super::ssh_mod::SshContext, encrypted_string_file_path: &camino::Utf8Path) -> DockerHubClient {
                cargo_auto_encrypt_secret_lib::decrypt_with_ssh_interactive_from_file(&mut ssh_context, encrypted_string_file_path);
                let secret_token = ssh_context.get_decrypted_string();
                let mut docker_hub_client = DockerHubClient::new_wo_secret_token();
                docker_hub_client.encrypted_token = super::secrecy_mod::SecretEncryptedString::new_with_secret_string(secret_token, &docker_hub_client.session_passcode);
                docker_hub_client
            }

            // check if the plain-text file from `podman login` exists and warn the user because it is a security vulnerability.
            let file_auth = "${XDG_RUNTIME_DIR}/containers/auth.json";
            // TODO: check for env variable also?
            if let Some(xdg_runtime_dir) = std::env::var_os("XDG_RUNTIME_DIR"){
                let xdg_runtime_dir=xdg_runtime_dir.to_string_lossy().to_string();
                let file_auth_expanded = file_auth.replace("${XDG_RUNTIME_DIR}", &xdg_runtime_dir);
                let file_auth_expanded = camino::Utf8Path::new(&file_auth_expanded);
                if file_auth_expanded.exists() {
                    eprintln!("{RED}Security vulnerability: Found the docker hub file with plain-text secret_token: {file_auth_expanded}. It would be better to inspect and remove it. {RESET}")
                }
            }

            // registry: docker.io -> replace dot into "--""
            // username: bestiadev
            let registry_escaped = registry.replace(".", "--");
            let encrypted_string_file_path = format!("~/.ssh/docker_hub_{registry_escaped}_{user_name}.txt");
            let encrypted_string_file_path = camino::Utf8Path::new(&encrypted_string_file_path);
            let encrypted_string_file_path_expanded = cargo_auto_encrypt_secret_lib::file_path_home_expand(encrypted_string_file_path);

            let identity_private_file_path = camino::Utf8Path::new("~/.ssh/docker_hub_secret_token_ssh_1");
            let _identity_private_file_path_expanded = crate::secrets_always_local_mod::ssh_mod::expand_path_check_private_key_exists(identity_private_file_path);

            if !encrypted_string_file_path_expanded.exists() {
                // ask interactive
                println!("    {BLUE}Do you want to store the docker hub secret_token encrypted with an SSH key? (y/n){RESET}");
                let answer = inquire::Text::new("").prompt().unwrap();
                if answer.to_lowercase() != "y" {
                    // enter the secret_token manually, not storing
                    return Self::new_interactive_input_secret_token();
                } else {
                    // get the passphrase and secret_token interactively
                    let mut ssh_context = super::ssh_mod::SshContext::new();
                    // encrypt and save the encrypted secret_token
                    cargo_auto_encrypt_secret_lib::encrypt_with_ssh_interactive_save_file(&mut ssh_context, identity_private_file_path, encrypted_string_file_path);
                    // read the secret_token and decrypt, return DockerHubClient
                    read_secret_token_and_decrypt_return_docker_hub_client(ssh_context, encrypted_string_file_path)
                }
            } else {
                // file exists
                let ssh_context = super::ssh_mod::SshContext::new();
                // read the secret_token and decrypt, return DockerHubClient
                read_secret_token_and_decrypt_return_docker_hub_client(ssh_context, encrypted_string_file_path)
            }
        }

        /// decrypts the secret_token in memory
        #[allow(dead_code)]
        pub fn decrypt_secret_token_in_memory(&self) -> secrecy::SecretString {
            self.encrypted_token.expose_decrypted_secret(&self.session_passcode)
        }

        /// Push to docker hub
        ///
        /// This function encapsulates the secret docker hub secret_token.
        /// The client can be passed to the library. It will not reveal the secret_token.
        #[allow(dead_code)]
        pub fn push_to_docker_hub(&self, image_url: &str, user_name: &str) {
            // the secret_token can be used in place of the password in --cred
            ShellCommandLimitedDoubleQuotesSanitizer::new(r#"podman push --creds "{user_name}:{secret_token}" "{image_url}" "#)
                .unwrap_or_else(|e| panic!("{e}"))
                .arg("{user_name}", user_name)
                .unwrap_or_else(|e| panic!("{e}"))
                .arg_secret("{secret_token}", &self.decrypt_secret_token_in_memory())
                .unwrap_or_else(|e| panic!("{e}"))
                .arg("{image_url}", image_url)
                .unwrap_or_else(|e| panic!("{e}"))
                .run()
                .unwrap_or_else(|e| panic!("{e}"));
        }
    }
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "Cargo.toml",
        file_content: r###"[package]
name = "automation_tasks_rs"
version = "1.0.0"
authors = ["bestia.dev"]
homepage = "https://bestia.dev"
edition = "2021"
description = "Automation tasks coded in Rust language for the workflow of Rust projects"
publish = false

[dependencies]
cargo_auto_lib = "2.4.8"
cargo_auto_github_lib = "1.1.6"
cargo_auto_encrypt_secret_lib = "1.1.7"

inquire = "0.7.0"
serde_json = {version= "1.0.114", features=["std"]}

# the version of reqwest must be the same as the version in the library cargo_auto_github_lib
reqwest = { version = "0.12.3", features = ["blocking", "stream"] }

camino = "1.1.6"
aes-gcm = "0.10.3"
ssh-key = { version = "0.6.4", features = [ "rsa", "encryption"] }
rsa = { version = "0.9.6", features = ["sha2","pem"] }
secrecy = { version="0.8.0", features=["alloc"]}
base64ct = {version = "1.6.0", features = ["alloc"] }

tracing = "0.1.40"
tracing-subscriber = { version = "0.3.18", features = ["env-filter", "std", "fmt", "time"] }
tracing-appender="0.2.2"
time = {version="0.3.36", features=["macros","local-offset"]}
"###,
    });
    vec_file.push(crate::FileItem{
            file_name :"README.md",
            file_content : r###"[//]: # (auto_md_to_doc_comments segment start A)

# automation_tasks_rs

In this sub-project `automation_tasks_rs`, you can write tasks that you need when compiling or managing your Rust project.  
The simple `cargo build` and `cargo build --release` are sometimes not enough. We need to copy some files, to prepare some environment. It is nice to have `all` the tasks in one place with a sort order that new users can easily follow.  
It is a Rust project, so you don't have to learn another strange language for automation.  
This helper project is used in combination with the program `cargo-auto`. Install it with `cargo install cargo-auto`.
You can use also the cargo bash completion program `cargo install dev_bestia_cargo_completion`.  

Don't put any secrets like passwords, passphrases, or tokens here, because this helper project is pushed to the remote repository together with the main Rust project.  

In the main  project folder (where the Cargo.toml file is) run

```bash
cargo auto
```

You will get the list of possible tasks with descriptions like this:
user-defined tasks

You can write any task you need. You have all the power of the Rust language under your fingertips.  
You can use or write a library for some specific tasks you need.  
For example, there is the crate `cargo_auto_github_lib` if you need to create a Release on GitHub.  

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
"###,
});
    vec_file.push(crate::FileItem {
        file_name: ".vscode/settings.json",
        file_content: r###"{
    "workbench.colorCustomizations": {
        "titleBar.activeForeground": "#fff",
        "titleBar.inactiveForeground": "#ffffffcc",
        "titleBar.activeBackground": "#a81c1c",
        "titleBar.inactiveBackground": "#630b0bcc"
    },
    "spellright.language": [
        "en"
    ],
    "spellright.documentTypes": [
        "markdown",
        "latex",
        "plaintext"
    ],
    "files.associations": {
        "LICENSE": "plain text"
    },
    "rust-analyzer.showUnlinkedFileNotification": false,
    "cSpell.words": [
        "Alla",
        "alloc",
        "appender",
        "bestia",
        "bestiadev",
        "camino",
        "CRUSTDE",
        "decryptor",
        "encryptor",
        "endregion",
        "keygen",
        "Nazdravlje",
        "new_cli",
        "octocrab",
        "passcode",
        "plantuml",
        "Prost",
        "reqwest",
        "rustdevuser",
        "rustprojects",
        "serde",
        "sshadd",
        "struct",
        "subsecond",
        "substack",
        "thiserror",
        "zcvf",
        "zdravje",
        "zeroize"
    ]
}"###,
    });
    // endregion: files copied into strings by automation tasks

    // return
    vec_file
}
