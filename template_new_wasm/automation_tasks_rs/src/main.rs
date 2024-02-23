// automation_tasks_rs for cargo_auto_template_new_wasm

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
                } else if &task == "publish_to_web" {
                    task_publish_to_web();
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
{GREEN}cargo auto publish_to_web - publish to web, git tag{RESET}
    {YELLOW}It is preferred to use SSH to publish to web and remotely manage the web server.{RESET}
    {YELLOW}<https://github.com/bestia-dev/docker_rust_development/blob/main/ssh_easy.md>{YELLOW}
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
        let sub_commands = vec!["build", "release", "doc", "test", "commit_and_push", "publish_to_web", "github_new_release",];
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

/// wasm-pack build
fn task_build() {
    cl::auto_version_increment_semver_or_date();
    cl::run_shell_command("cargo fmt");
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");
    cl::run_shell_command("wasm-pack build --target web");
    cl::run_shell_command("\\rsync -a --delete-after pkg/ web_server_folder/cargo_auto_template_new_wasm/pkg/");
    println!(
        r#"
    {YELLOW}After `cargo auto build`, open port 4000 in VSCode and run the basic web server{RESET}
    {YELLOW}in a separate VSCode bash terminal, so it can serve constantly in the background.{RESET}
{GREEN}basic-http-server -a 0.0.0.0:4000 ./web_server_folder{RESET}
    {YELLOW}and open the browser on{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#print/world{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#upper/world{RESET}
    {YELLOW}This will return an error:{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#upper/WORLD{RESET}
    {YELLOW}If all is fine, run{RESET}
{GREEN}cargo auto release{RESET}
"#
    );
}

/// wasm-pack build --release
fn task_release() {
    cl::auto_version_increment_semver_or_date();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");

    cl::run_shell_command("cargo fmt");
    cl::run_shell_command("wasm-pack build --target web");
    cl::run_shell_command("\\rsync -a --delete-after pkg/ web_server_folder/cargo_auto_template_new_wasm/pkg/");
    println!(
        r#"
    {YELLOW}After `cargo auto build`, open port 4000 in VSCode and run the basic web server{RESET}
    {YELLOW}in a separate VSCode bash terminal, so it can serve constantly in the background.{RESET}
{GREEN}basic-http-server -a 0.0.0.0:4000 ./web_server_folder{RESET}
    {YELLOW}and open the browser on{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm{RESET}    
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#print/world{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#upper/world{RESET}
    {YELLOW}This will return an error:{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#upper/WORLD{RESET}
    {YELLOW}If all is fine, run{RESET}
{GREEN}cargo auto doc{RESET}
"#
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
    println!(r#"    {YELLOW}Wasm is a cdylib and therefore doc-tests are not run !{RESET}"#);
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

    // init repository if needed. If it is not init then normal commit and push.
    if !cl::init_repository_if_needed(&message) {
        // separate commit for docs if they changed, to not make a lot of noise in the real commit
        if std::path::Path::new("docs").exists() {
            cl::run_shell_command(
                r#"git add docs && git diff --staged --quiet || git commit -m "update docs" "#,
            );
        }
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

/// publish to web
fn task_publish_to_web() {
    let cargo_toml = cl::CargoToml::read();
    // take care of tags
    let tag_name_version = cl::git_tag_sync_check_create_push(&version);

    // Find the filename of the identity_file for ssh connection to host_name, to find out if need ssh-add or not.
    // parse the ~/.ssh/config. 99% probably there should be a record for host_name and there is the identity_file.
    // else ask user for filename, then run ssh-add
    cl::ssh_add_resolve("project_homepage","bestia_dev_ssh_1");

    // rsync to copy to server over ssh
    let shell_command = format!(
        r#"rsync -e ssh -a --info=progress2 --delete-after ~/rustprojects/{package_name}/web_server_folder/ project_author@project_homepage:/var/www/project_homepage/pwa_short_name/"#,
        package_name = cargo_toml.package_name()
    );
    cl::run_shell_command(&shell_command);

    println!(
        r#"{YELLOW}
    After `cargo auto publish_to_web`, 
    check 
https://bestia.dev/{package_name}
{RESET}"#,
        package_name = cargo_toml.package_name()
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