// automation_tasks_rs for rust_project_name

// region: library and modules with basic automation tasks

// for projects that don't use GitHub, delete all the mentions of GitHub
mod secrets_always_local_mod;
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
    {YELLOW}<https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod/blob/main/ssh_easy.md>{YELLOW}
    {YELLOW}On the very first commit, this task will initialize a new local git repository and create a remote GitHub repo.{RESET}
    {YELLOW}For the GitHub API the task needs the Personal Access Token Classic from <https://github.com/settings/tokens>{RESET}
    {YELLOW}You can choose to type the token every time or to store it in a file encrypted with an SSH key.{RESET}
    {YELLOW}Then you can type the passphrase of the private key every time. This is pretty secure.{RESET}
    {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}
{GREEN}cargo auto publish_to_web - publish to web, git tag{RESET}
    {YELLOW}It is preferred to use SSH to publish to web and remotely manage the web server.{RESET}
    {YELLOW}<https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod/blob/main/ssh_easy.md>{YELLOW}
{GREEN}cargo auto github_new_release{RESET} - {YELLOW}creates new release on GitHub{RESET}
    {YELLOW}For the GitHub API the task needs the Personal Access Token Classic from <https://github.com/settings/tokens>{RESET}
    {YELLOW}You can choose to type the token every time or to store it in a file encrypted with an SSH key.{RESET}
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
        let sub_commands = vec!["build", "release", "doc", "test", "commit_and_push", "publish_to_web", "github_new_release",];
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

/// wasm-pack build
fn task_build() {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::run_shell_command_static("cargo fmt").unwrap_or_else(|e| panic!("{e}"));
    cl::run_shell_command_static("wasm-pack build --target web").unwrap_or_else(|e| panic!("{e}"));

    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"rsync -a --delete-after pkg/ "web_server_folder/{package_name}/pkg/" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{package_name}", &cargo_toml.package_name()).unwrap_or_else(|e| panic!("{e}"))
    .run().unwrap_or_else(|e| panic!("{e}"));

    println!(
        r#"
    {YELLOW}After `cargo auto build`, open port 4000 in VSCode and run the basic web server{RESET}
    {YELLOW}in a separate VSCode bash terminal, so it can serve constantly in the background.{RESET}
{GREEN}basic-http-server -a 0.0.0.0:4000 ./web_server_folder{RESET}
    {YELLOW}and open the browser on{RESET}
{GREEN}http://localhost:4000/{package_name}{RESET}
{GREEN}http://localhost:4000/{package_name}#print/world{RESET}
{GREEN}http://localhost:4000/{package_name}#upper/world{RESET}
    {YELLOW}This will return an error:{RESET}
{GREEN}http://localhost:4000/{package_name}#upper/WORLD{RESET}
    {YELLOW}If all is fine, run{RESET}
{GREEN}cargo auto release{RESET}
"#,
    package_name = cargo_toml.package_name()

    );
}

/// wasm-pack build --release
fn task_release() {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");

    cl::run_shell_command_static("cargo fmt").unwrap_or_else(|e| panic!("{e}"));
    cl::run_shell_command_static("wasm-pack build --target web").unwrap_or_else(|e| panic!("{e}"));

    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"rsync -a --delete-after pkg/ "web_server_folder/{package_name}/pkg/" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{package_name}", &cargo_toml.package_name()).unwrap_or_else(|e| panic!("{e}"))
    .run().unwrap_or_else(|e| panic!("{e}"));

    println!(
        r#"
    {YELLOW}After `cargo auto build`, open port 4000 in VSCode and run the basic web server{RESET}
    {YELLOW}in a separate VSCode bash terminal, so it can serve constantly in the background.{RESET}
{GREEN}basic-http-server -a 0.0.0.0:4000 ./web_server_folder{RESET}
    {YELLOW}and open the browser on{RESET}
{GREEN}http://localhost:4000/{package_name}{RESET}    
{GREEN}http://localhost:4000/{package_name}#print/world{RESET}
{GREEN}http://localhost:4000/{package_name}#upper/world{RESET}
    {YELLOW}This will return an error:{RESET}
{GREEN}http://localhost:4000/{package_name}#upper/WORLD{RESET}
    {YELLOW}If all is fine, run{RESET}
{GREEN}cargo auto doc{RESET}
"#,
    package_name = cargo_toml.package_name()
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
    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"printf "<meta http-equiv=\"refresh\" content=\"0; url={url_sanitized_for_double_quote}/index.html\" />\n" > docs/index.html"#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{url_sanitized_for_double_quote}", &cargo_toml.package_name().replace("-", "_")).unwrap_or_else(|e| panic!("{e}"))
    .run().unwrap_or_else(|e| panic!("{e}"));

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
    println!(r#"    {YELLOW}Wasm is a cdylib and therefore doc-tests are not run !{RESET}"#);
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
        let github_client = github_mod::GitHubClient::new_with_stored_token();
        cgl::new_remote_github_repository(&github_client).unwrap();
        cgl::description_and_topics_to_github(&github_client);
    } else {
        let github_client = github_mod::GitHubClient::new_with_stored_token();
        // if description or topics/keywords/tags have changed
        cgl::description_and_topics_to_github(&github_client);

        // separate commit for docs if they changed, to not make a lot of noise in the real commit
        if std::path::Path::new("docs").exists() {
            cl::run_shell_command_static(r#"git add docs && git diff --staged --quiet || git commit -m "update docs" "#).unwrap_or_else(|e| panic!("{e}"));
        }

        cl::add_message_to_unreleased(&message);
        // the real commit of code
        cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"git add -A && git diff --staged --quiet || git commit -m "{message_sanitized_for_double_quote}" "#).unwrap_or_else(|e| panic!("{e}"))
        .arg("{message_sanitized_for_double_quote}", &message).unwrap_or_else(|e| panic!("{e}"))
        .run().unwrap_or_else(|e| panic!("{e}"));

        cl::run_shell_command_static("git push").unwrap_or_else(|e| panic!("{e}"));
    }

    println!(
        r#"
    {YELLOW}After `cargo auto commit_and_push "message"`{RESET}
{GREEN}cargo auto publish_to_web{RESET}
"#
    );
}

/// publish to web
fn task_publish_to_web() {
    let cargo_toml = cl::CargoToml::read();
    let version = cargo_toml.package_version();
    // take care of tags
    let _tag_name_version = cl::git_tag_sync_check_create_push(&version);

    // rsync to copy to server over ssh into a temporary installation folder
    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
r#"rsync -e ssh -a --info=progress2 --delete-after "web_server_folder/{package_name}/" "{server_username}@{web_server_domain}:/var/www/transfer_folder/{package_name}" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{package_name}", &cargo_toml.package_name()).unwrap_or_else(|e| panic!("{e}"))
    .arg("{server_username}", "server_username").unwrap_or_else(|e| panic!("{e}"))
    .arg("{web_server_domain}", "web_server_domain").unwrap_or_else(|e| panic!("{e}"))    
    .run().unwrap_or_else(|e| panic!("{e}"));

    // rsync to copy to server over ssh the installation script
    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
r#"rsync -e ssh -a --info=progress2 --delete-after "publish_script/hello_world_publish.sh" "{server_username}@{web_server_domain}:/var/www/scripts/{package_name}/" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{package_name}", &cargo_toml.package_name()).unwrap_or_else(|e| panic!("{e}"))
    .arg("{server_username}", "server_username").unwrap_or_else(|e| panic!("{e}"))
    .arg("{web_server_domain}", "web_server_domain").unwrap_or_else(|e| panic!("{e}"))    
    .run().unwrap_or_else(|e| panic!("{e}"));

    //make the bash script executable
    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
r#"ssh "{server_username}@{web_server_domain}" chmod +x  "/var/www/scripts/{package_name}/hello_world_publish.sh" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{package_name}", &cargo_toml.package_name()).unwrap_or_else(|e| panic!("{e}"))
    .arg("{server_username}", "server_username").unwrap_or_else(|e| panic!("{e}"))
    .arg("{web_server_domain}", "web_server_domain").unwrap_or_else(|e| panic!("{e}"))    
    .run().unwrap_or_else(|e| panic!("{e}"));

    // run installation script over ssh on the server to copy from the installation folder to production folder
    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
r#"ssh "{server_username}@{web_server_domain}" "/var/www/scripts/{package_name}/hello_world_publish.sh" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{package_name}", &cargo_toml.package_name()).unwrap_or_else(|e| panic!("{e}"))
    .arg("{server_username}", "server_username").unwrap_or_else(|e| panic!("{e}"))
    .arg("{web_server_domain}", "web_server_domain").unwrap_or_else(|e| panic!("{e}"))    
    .run().unwrap_or_else(|e| panic!("{e}"));

    println!(
        r#"
    {YELLOW}After `cargo auto publish_to_web`check {RESET}
{GREEN}https://web_server_domain/{package_name}{RESET}
    {YELLOW}    {YELLOW}If all is fine, run{RESET}
{GREEN}cargo auto github_new_release{RESET}
"#,
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
    let body_md_text = cl::body_text_from_releases_md().unwrap();

    let github_client = github_mod::GitHubClient::new_with_stored_token();
    let json_value = github_client.send_to_github_api(cgl::github_api_create_new_release(&owner, &repo_name, &tag_name_version, &release_name, branch, &body_md_text));
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
/*
    // region: upload asset only for executables, not for libraries

    let release_id = json_value.get("id").unwrap().as_i64().unwrap().to_string();
    println!(
        "
        {YELLOW}Now uploading release asset. This can take some time if the files are big. Wait...{RESET}
    "
    );
    // compress files tar.gz
    let tar_name = format!("{repo_name}-{tag_name_version}-x86_64-unknown-linux-gnu.tar.gz");

    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"tar -zcvf "{tar_name_sanitized_for_double_quote}" "target/release/{repo_name_sanitized_for_double_quote}" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{tar_name_sanitized_for_double_quote}", &tar_name).unwrap_or_else(|e| panic!("{e}"))
    .arg("{repo_name_sanitized_for_double_quote}", &repo_name).unwrap_or_else(|e| panic!("{e}"))
    .run().unwrap_or_else(|e| panic!("{e}"));

    // upload asset
    cgl::github_api_upload_asset_to_release(&github_client, &owner, &repo_name, &release_id, &tar_name);

    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"rm "{tar_name_sanitized_for_double_quote}" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{tar_name_sanitized_for_double_quote}", &tar_name).unwrap_or_else(|e| panic!("{e}"))
    .run().unwrap_or_else(|e| panic!("{e}"));

    println!(
        r#"
    {YELLOW}Asset uploaded. Open and edit the description on GitHub Releases in the browser.{RESET}
    "#
    );

    // endregion: upload asset only for executables, not for libraries

*/
    println!(
        r#"
{GREEN}https://github.com/{owner}/{repo_name}/releases{RESET}
    "#
    );
}
// endregion: tasks
