//! this strings are copied from the template_new_auto folder
//! because when publishing to crates.io, only the main bin-executable is transferred

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
        panic!("{RED}Cannot compile automation_tasks_rs. Exiting.\nCorrect automation_tasks_rs/main.rs and try again.{RESET}");
    }
}

pub fn copy_to_files(project_name: &str) {
    let folder_path = std::path::Path::new(project_name);
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
    vec_file.push(crate::FileItem{
            file_name :"README.md",
            file_content : r###"# automation_tasks_rs

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
"###,
});
    vec_file.push(crate::FileItem {
        file_name: ".vscode/settings.json",
        file_content: r###"{
    "workbench.colorCustomizations": {
        "titleBar.activeForeground": "#fff",
        "titleBar.inactiveForeground": "#ffffffcc",
        "titleBar.activeBackground": "#404040",
        "titleBar.inactiveBackground": "#2d2d2dcc"
    },
    "spellright.language": [
        "en"
    ],
    "spellright.documentTypes": [
        "markdown",
        "latex",
        "plaintext"
    ],
    "rust-analyzer.showUnlinkedFileNotification": false,
    "cSpell.words": [
        "bestia",
        "deps",
        "endregion",
        "plantuml",
        "zcvf"
    ]
}"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "Cargo.toml",
        file_content: r###"[package]
name = "automation_tasks_rs"
version = "1.0.0"
authors = ["bestia.dev"]
homepage = "https://bestia.dev"
edition = "2021"
description = "cargo auto - automation tasks written in Rust language"
publish = false

[dependencies]
cargo_auto_lib = "1.3.36""###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/main.rs",
        file_content: r###"// automation_tasks_rs for project_name

// region: library with basic automation tasks
use cargo_auto_lib as cl;
// traits must be in scope (Rust strangeness)
use cl::CargoTomlPublicApiMethods;

use cargo_auto_lib::GREEN;
use cargo_auto_lib::RED;
use cargo_auto_lib::RESET;
use cargo_auto_lib::YELLOW;

// region: library with basic automation tasks

fn main() {
    cl::exit_if_not_run_in_rust_project_root_directory();

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
    {YELLOW}<https://github.com/bestia-dev/docker_rust_development/blob/main/ssh_easy.md>{YELLOW}
    {YELLOW}On the very first commit, this task will initialize a new local git repository and create a remote GitHub repo.{RESET}
    {YELLOW}In that case the task needs the Personal Access Token Classic from <https://github.com/settings/tokens>{RESET}
{GREEN}cargo auto publish_to_crates_io{RESET} - {YELLOW}publish to crates.io, git tag{RESET}
    {YELLOW}You need the API token for publishing. Get the token on <https://crates.io/settings/tokens>. Then use the command{RESET}
    {YELLOW}`cargo login` and paste the token when prompted. This will save it to a local credentials file.{RESET}
{GREEN}cargo auto github_new_release{RESET} - {YELLOW}creates new release on github{RESET}
    {YELLOW}This task needs the Personal Access Token Classic from <https://github.com/settings/tokens>{RESET}

    {YELLOW}© 2024 bestia.dev  MIT License github.com/bestia-dev/cargo-auto{RESET}
"#
    );
    print_examples_cmd();
}

/// all example commands in one place
fn print_examples_cmd() {
    /*
        println!(r#"{YELLOW}run examples:{RESET}
    {GREEN}cargo run --example example1{RESET}
    "#);
    */
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
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
    let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::run_shell_command("cargo fmt");
    cl::run_shell_command("cargo build");
    println!(
        r#"
    {YELLOW}After `cargo auto build`, run the compiled binary, examples and/or tests{RESET}
{GREEN}./target/debug/{package_name} argument{RESET}
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

    cl::run_shell_command("cargo fmt");
    cl::run_shell_command("cargo build --release");
    cl::run_shell_command(&format!(
        "strip target/release/{package_name}",
        package_name = cargo_toml.package_name()
    ));
    println!(
        r#"
    {YELLOW}After `cargo auto release`, run the compiled binary, examples and/or tests{RESET}
{GREEN}./target/release/{package_name} argument{RESET}
    {YELLOW}if ok then{RESET}
{GREEN}cargo auto doc{RESET}
"#,
        package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
}

/// cargo doc, then copies to /docs/ folder, because this is a github standard folder
fn task_doc() {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");
    cl::auto_plantuml(&cargo_toml.package_repository().unwrap());
    cl::auto_playground_run_code();
    cl::auto_md_to_doc_comments();

    cl::run_shell_command("cargo doc --no-deps --document-private-items");
    // copy target/doc into docs/ because it is github standard
    cl::run_shell_command("rsync -a --info=progress2 --delete-after target/doc/ docs/");
    // Create simple index.html file in docs directory
    cl::run_shell_command(&format!(
        r#"echo "<meta http-equiv=\"refresh\" content=\"0; url={}/index.html\" />" > docs/index.html"#,
        cargo_toml.package_name().replace("-", "_")
    ));
    // pretty html
    cl::auto_doc_tidy_html().unwrap();
    cl::run_shell_command("cargo fmt");
    // message to help user with next move
    println!(
        r#"
    {YELLOW}After `cargo auto doc`, check `docs/index.html`. If ok then test the documentation code examples{RESET}
{GREEN}cargo auto test{RESET}
"#
    );
}

/// cargo test
fn task_test() {
    cl::run_shell_command("cargo test");
    println!(
r#"
    {YELLOW}After `cargo auto test`. If ok then {RESET}
{GREEN}cargo auto commit_and_push "message"{RESET}
    {YELLOW}with mandatory commit message{RESET}
"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    let Some(message) = arg_2 else {
        eprintln!("{RED}Error: Message for commit is mandatory. Exiting.{RESET}");
        // early exit
        return;
    };

    // if description or topics/keywords/tags have changed
    cl::description_and_topics_to_github();

    // init repository if needed. If it is not init then normal commit and push.
    if !cl::init_repository_if_needed(&message) {
        // separate commit for docs if they changed, to not make a lot of noise in the real commit
        if std::path::Path::new("docs").exists() {
            cl::run_shell_command(r#"git add docs && git diff --staged --quiet || git commit -m "update docs" "#);
        }
        cl::add_message_to_unreleased(&message);
        // the real commit of code
        cl::run_shell_command(&format!( r#"git add -A && git diff --staged --quiet || git commit -m "{message}" "#));
        cl::run_shell_command("git push");
        println!(
r#"
    {YELLOW}After `cargo auto commit_and_push "message"`{RESET}
{GREEN}cargo auto publish_to_crates_io{RESET}
"#
        );
    }
}

/// publish to crates.io and git tag
fn task_publish_to_crates_io() {
    let cargo_toml = cl::CargoToml::read();
    let package_name = cargo_toml.package_name();
    let version = cargo_toml.package_version();
    // take care of tags
    let tag_name_version = cl::git_tag_sync_check_create_push(&version);

    // cargo publish
    cl::run_shell_command("cargo publish");
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

    let owner = cargo_toml.github_owner().unwrap();
    let repo_name = cargo_toml.package_name();
    let now_date = cl::now_utc_date_iso();
    let release_name = format!("Version {} ({})", &version, now_date);
    let branch = "main";

    // First, the user must write the content into file RELEASES.md in the section ## Unreleased.
    // Then the automation task will copy the content to GitHub release
    // and create a new Version title in RELEASES.md.
    let body_md_text = cl::body_text_from_releases_md(&release_name).unwrap();

    let _release_id = cl::github_api_create_new_release(
        &owner,
        &repo_name,
        &tag_name_version,
        &release_name,
        branch,
        &body_md_text,
    );

    println!(
        "
    {YELLOW}New GitHub release created: {release_name}.{RESET}
"
    );

    /*
        // region: upload asset only for executables, not for libraries
        println!("
        {YELLOW}Now uploading release asset. This can take some time if the files are big. Wait...{RESET}
    ");
        // compress files tar.gz
        let tar_name = format!("{repo_name}-{tag_name_version}-x86_64-unknown-linux-gnu.tar.gz");
        cl::run_shell_command(&format!("tar -zcvf {tar_name} target/release/{repo_name}"));

        // upload asset
        cl::github_api_upload_asset_to_release(&owner, &repo_name, &release_id, &tar_name).await;
        cl::run_shell_command(&format!("rm {tar_name}"));

        println!("
        {YELLOW}Asset uploaded. Open and edit the description on GitHub Releases in the browser.{RESET}
    ");
        // endregion: upload asset only for executables, not for libraries

        */
    println!(
        "
{GREEN}https://github.com/{owner}/{repo_name}/releases{RESET}
    "
    );
}
// endregion: tasks
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: ".gitignore",
        file_content: r###"/target

# not needed in commits, but also not a problem if they are committed
/.file_hashes.json"###,
    });
    // endregion: files copied into strings by automation tasks

    // return
    vec_file
}
