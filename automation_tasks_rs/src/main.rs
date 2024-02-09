//! automation_tasks_rs for cargo-auto

mod copy_files_to_strings_mod;

// region: library with basic automation tasks
use cargo_auto_lib as cl;
// traits must be in scope (Rust strangeness)
use cl::CargoTomlPublicApiMethods;

use cargo_auto_lib::GREEN;
use cargo_auto_lib::RED;
use cargo_auto_lib::RESET;
use cargo_auto_lib::YELLOW;
// region: library with basic automation tasks

use cargo_auto_github_lib::*;

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
                    println!("{RED}Error: Task {task} is unknown.{RESET}");
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
{GREEN}cargo auto build{RESET}{YELLOW} - builds the crate in debug mode, fmt, increment version{RESET}
{GREEN}cargo auto release{RESET}{YELLOW} - builds the crate in release mode, fmt, increment version{RESET}
{GREEN}cargo auto doc{RESET}{YELLOW} - builds the docs, copy to docs directory{RESET}
{GREEN}cargo auto test{RESET}{YELLOW} - runs all the tests{RESET}
{GREEN}cargo auto commit_and_push "message"{RESET}{YELLOW} - commits with message and push with mandatory message
    (If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git.){RESET}
{GREEN}cargo auto publish_to_crates_io{RESET}{YELLOW} - publish to crates.io, git tag
    (You need credentials for publishing. On crates.io get the 'access token'. Then save it locally once and forever with the command 
    ` cargo login TOKEN` use a space before the command to avoid saving the secret token in bash history.){RESET}
{GREEN}cargo auto github_new_release{RESET}{YELLOW} - creates new release on github
    This task needs PAT (personal access token from github) in the env variable:{RESET}
{GREEN} export GITHUB_TOKEN=paste_token_here{RESET}

    {YELLOW}© 2023 bestia.dev  MIT License github.com/bestia-dev/cargo-auto{RESET}
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
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// cargo build
fn task_build() {
    let cargo_toml = cl::CargoToml::read();

    copy_files_to_strings_mod::copy_folder_files_into_module(
        std::path::Path::new("template_new_auto"),
        std::path::Path::new("src/template_new_auto_mod.rs"),
    );

    copy_files_to_strings_mod::copy_folder_files_into_module(
        std::path::Path::new("template_new_cli"),
        std::path::Path::new("src/template_new_cli_mod.rs"),
    );

    copy_files_to_strings_mod::copy_folder_files_into_module(
        std::path::Path::new("template_new_wasm"),
        std::path::Path::new("src/template_new_wasm_mod.rs"),
    );

    copy_files_to_strings_mod::copy_folder_files_into_module(
        std::path::Path::new("template_new_pwa_wasm"),
        std::path::Path::new("src/template_new_pwa_wasm_mod.rs"),
    );

    cl::auto_version_increment_semver_or_date();
    cl::run_shell_command("cargo fmt");
    cl::run_shell_command("cargo build");
    println!(
        r#"
    {YELLOW}After `cargo auto build`, run the compiled binary, examples and/or tests{RESET}
{GREEN}./target/debug/{package_name} argument{RESET}
    {YELLOW}if ok, then,{RESET}
{GREEN}cargo auto release{RESET}
"#,
        package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
}

/// cargo build --release
fn task_release() {
    copy_files_to_strings_mod::copy_folder_files_into_module(
        std::path::Path::new("template_new_auto"),
        std::path::Path::new("src/template_new_auto_mod.rs"),
    );

    copy_files_to_strings_mod::copy_folder_files_into_module(
        std::path::Path::new("template_new_cli"),
        std::path::Path::new("src/template_new_cli_mod.rs"),
    );

    copy_files_to_strings_mod::copy_folder_files_into_module(
        std::path::Path::new("template_new_wasm"),
        std::path::Path::new("src/template_new_wasm_mod.rs"),
    );

    copy_files_to_strings_mod::copy_folder_files_into_module(
        std::path::Path::new("template_new_pwa_wasm"),
        std::path::Path::new("src/template_new_pwa_wasm_mod.rs"),
    );

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
    {YELLOW}if ok, then,{RESET}
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
    cl::auto_md_to_doc_comments();

    cl::run_shell_command("cargo doc --no-deps --document-private-items");
    // copy target/doc into docs/ because it is github standard
    cl::run_shell_command("rsync -a --info=progress2 --delete-after target/doc/ docs/");
    // Create simple index.html file in docs directory
    cl::run_shell_command(&format!(
        "echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",
        cargo_toml.package_name().replace("-", "_")
    ));

    // region: tidy HTML
    // The HTML generated by `cargo doc` is ugly and difficult to `git diff`
    // tidy HTML is a HTML checker and formatter installed on most Linuxes.
    // If it is not installed run: `sudo apt install -y tidy`
    // From the bash you can install it inside the podman container like this:
    // `podman exec --user root rust_dev_vscode_cnt apt install -y tidy`
    //
    // First we check if tidy is installed on the system
    // Run a dummy command and write the std/err output to tidy_warnings.txt.
    // The command `2>` will overwrite the file and not append like `2>>`.
    cl::run_shell_command("tidy xxx 2> docs/tidy_warnings.txt");
    // Check if it contains `command not found`
    let text = std::fs::read_to_string("docs/tidy_warnings.txt").unwrap();
    // don't need this file anymore
    cl::run_shell_command("rm -f docs/tidy_warnings.txt");
    if !text.contains("command not found") {
        // Use tidy HTML to format the docs/*.html files to be human readable and usable for git diff.
        // Options: -m modify file, -q quiet suppress nonessential output, -w wrap at 160,
        // The warnings and errors are appended to the file docs/tidy_warnings.txt
        cl::run_shell_command(
            r#"find ./docs -name '*.html' -type f -print -exec tidy -mq -w 160 '{}' \; >> docs/tidy_warnings.txt 2>&1 "#,
        );
    }
    // endregion: tidy HTML
    cl::run_shell_command("cargo fmt");
    // message to help user with next move
    println!(
        r#"
    {YELLOW}After `cargo auto doc`, check `docs/index.html`. If ok, then test the documentation code examples{RESET}
{GREEN}cargo auto test{RESET}
    {YELLOW}{RESET}"#
    );
}

/// cargo test
fn task_test() {
    cl::run_shell_command("cargo test");
    println!(
        r#"
    {YELLOW}After `cargo auto test`. If ok, then {RESET}
{GREEN}cargo auto commit_and_push "message"{RESET}
    {YELLOW}with mandatory commit message{RESET}
"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    match arg_2 {
        None => println!("{RED}Error: Message for commit is mandatory.{RESET}"),
        Some(message) => {
            // separate commit for docs if they changed, to not make a lot of noise in the real commit
            cl::run_shell_command(r#"git add docs && git diff --staged --quiet || git commit -m "update docs" "#);
            // the real commit of code
            cl::run_shell_command(&format!(
                r#"git add -A && git diff --staged --quiet || git commit -m "{}" "#,
                message
            ));
            cl::run_shell_command("git push");
            println!(
                r#"
    {YELLOW}After `cargo auto commit_and_push "message"`{RESET}
{GREEN}cargo auto publish_to_crates_io{RESET}
"#
            );
        }
    }
}

/// publish to crates.io and git tag
fn task_publish_to_crates_io() {
    println!(r#"{YELLOW}The crates.io access token must already be saved locally with `cargo login TOKEN`{RESET}"#);

    let cargo_toml = cl::CargoToml::read();
    // git tag
    let shell_command = format!(
        "git tag -f -a v{version} -m version_{version}",
        version = cargo_toml.package_version()
    );
    cl::run_shell_command(&shell_command);

    // cargo publish
    cl::run_shell_command("cargo publish");
    println!(
        r#"
    {YELLOW}After `cargo auto publish_to_crates_io`, check in browser{RESET}
{GREEN}https://crates.io/crates/{package_name}{RESET}
    {YELLOW}Install the crate with{RESET}
{GREEN}cargo install {package_name}{RESET}
    {YELLOW}and check how it works.{RESET}
    {YELLOW}Then create the GitHub-Release and upload the assets.{RESET}    
{GREEN}cargo auto github_new_release{RESET}
"#,
        package_name = cargo_toml.package_name(),
        // package_version = cargo_toml.package_version()
    );
}

/// create a new release on github
fn task_github_new_release() {
    let cargo_toml = cl::CargoToml::read();
    println!("    {YELLOW}The env variable GITHUB_TOKEN must be set:  export GITHUB_TOKEN=paste_token_here{RESET}");

    // the git tag was already created when we published to crates.io

    // async block inside sync code with tokio
    use tokio::runtime::Runtime;
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        let owner = cargo_auto_github_lib::github_owner();
        let repo_name = cargo_toml.package_name();
        let tag_name_version = format!("v{}", cargo_toml.package_version());
        let release_name = format!("Release v{}", cargo_toml.package_version());
        let branch = "main";

        let body_md_text = &format!(
r#"## Changed

- edit the list of changes
          
"#);

        let release_id =  auto_github_create_new_release(&owner, &repo_name, &tag_name_version, &release_name, branch, body_md_text).await;
        println!("    {YELLOW}New release created, now uploading release asset. This can take some time if the files are big. Wait...{RESET}");

        // compress files tar.gz
        let tar_name = format!("{repo_name}-{tag_name_version}-x86_64-unknown-linux-gnu.tar.gz");
        cl::run_shell_command(&format!("tar -zcvf {tar_name} target/release/{repo_name}"));

        // upload asset     
        auto_github_upload_asset_to_release(&owner, &repo_name, &release_id, &tar_name).await;
        cl::run_shell_command(&format!("rm {tar_name}"));  

        println!("    {YELLOW}Asset uploaded. Open and edit the description on GitHub-Releases in the browser.{RESET}");
        println!("{GREEN}https://github.com/{owner}/{repo_name}/releases{RESET}");
    });
}
// endregion: tasks
