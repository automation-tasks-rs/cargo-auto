// generic_functions_mod.rs

//! These functions does not usually change

use crate::cgl;
use crate::cl;
use crate::ende;

use cargo_auto_lib::CargoTomlPublicApiMethods;
use cargo_auto_lib::ShellCommandLimitedDoubleQuotesSanitizerTrait;
#[allow(unused_imports)]
pub use cl::{BLUE, GREEN, RED, RESET, YELLOW};

/// Initialize tracing to file logs/automation_tasks_rs.log
///
/// The folder logs/ is in .gitignore and will not be committed.
pub fn tracing_init() {
    // uncomment this line to enable tracing to file
    // let file_appender = tracing_appender::rolling::daily("logs", "automation_tasks_rs.log");

    let offset = time::UtcOffset::current_local_offset().expect("should get local offset!");
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(
        offset,
        time::macros::format_description!("[hour]:[minute]:[second].[subsecond digits:6]"),
    );

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
pub fn panic_set_hook(panic_info: &std::panic::PanicHookInfo) {
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

#[allow(dead_code)]
/// cargo doc, then copies to /docs/ folder, because this is a GitHub standard folder
pub fn task_doc() {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");
    // In cargo_auto_lib we have sample data that we don't want to change, avoid this lines.
    if cargo_toml.package_name() != "cargo_auto_lib" {
        cl::auto_plantuml(&cargo_toml.package_repository().unwrap());
        cl::auto_playground_run_code();
    }
    cl::auto_md_to_doc_comments();

    cl::run_shell_command_static("cargo doc --no-deps --document-private-items").unwrap_or_else(|e| panic!("{e}"));
    // copy target/doc into docs/ because it is GitHub standard
    cl::run_shell_command_static("rsync -a --info=progress2 --delete-after target/doc/ docs/").unwrap_or_else(|e| panic!("{e}"));

    // Create simple index.html file in docs directory
    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
        r#"printf "<meta http-equiv=\"refresh\" content=\"0; url={url_sanitized_for_double_quote}/index.html\" />\n" > docs/index.html"#,
    )
    .unwrap_or_else(|e| panic!("{e}"))
    .arg("{url_sanitized_for_double_quote}", &cargo_toml.package_name().replace("-", "_"))
    .unwrap_or_else(|e| panic!("{e}"))
    .run()
    .unwrap_or_else(|e| panic!("{e}"));

    // pretty html
    #[cfg(target_family = "unix")]
    cl::auto_doc_tidy_html().unwrap_or_else(|e| panic!("{e}"));

    cl::run_shell_command_static("cargo fmt").unwrap_or_else(|e| panic!("{e}"));
    // message to help user with next move
    println!(
        r#"
  {YELLOW}After `cargo auto doc`, ctrl-click on `docs/index.html`. 
    It will show the index.html in VSCode Explorer, then right-click and choose "Show Preview".
    This works inside the CRUSTDE container, because of the extension "Live Preview" 
    <https://marketplace.visualstudio.com/items?itemName=ms-vscode.live-server>
"#
    );
}

/// commit and push
pub fn task_commit_and_push(arg_2: Option<String>) {
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
        cgl::new_remote_github_repository().unwrap();
        cgl::description_and_topics_to_github();
    } else {
        // if description or topics/keywords/tags have changed
        cgl::description_and_topics_to_github();

        // separate commit for docs if they changed, to not make a lot of noise in the real commit
        if std::path::Path::new("docs").exists() {
            cl::run_shell_command_static(r#"git add docs && git diff --staged --quiet || git commit -m "update docs" "#)
                .unwrap_or_else(|e| panic!("{e}"));
        }

        cl::add_message_to_unreleased(&message);
        // the real commit of code
        cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
            r#"git add -A && git diff --staged --quiet || git commit -m "{message_sanitized_for_double_quote}" "#,
        )
        .unwrap_or_else(|e| panic!("{e}"))
        .arg("{message_sanitized_for_double_quote}", &message)
        .unwrap_or_else(|e| panic!("{e}"))
        .run()
        .unwrap_or_else(|e| panic!("{e}"));

        cl::run_shell_command_static("git push").unwrap_or_else(|e| panic!("{e}"));
    }
}

#[allow(dead_code)]
/// create a new release on github
pub fn task_github_new_release() {
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
    let version_body_text = cl::body_text_from_releases_md().unwrap();
    // Create a new Version title and modify RELEASES.md.
    cl::create_new_version_in_releases_md(&release_name).unwrap();

    // Commit and push of modified Version in RELEASES.md
    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
        r#"git add -A && git commit -m "{message_sanitized_for_double_quote}" "#,
    )
    .unwrap_or_else(|e| panic!("{e}"))
    .arg("{message_sanitized_for_double_quote}", &release_name)
    .unwrap_or_else(|e| panic!("{e}"))
    .run()
    .unwrap_or_else(|e| panic!("{e}"));

    cl::run_shell_command_static("git push").unwrap_or_else(|e| panic!("{e}"));

    // GitHub api call to create the Release
    let request = cgl::github_api_create_new_release(&github_owner, &repo_name, &tag_name_version, &release_name, branch, &version_body_text);
    let json_value = ende::github_api_token_with_oauth2_mod::send_to_github_api_with_secret_token(request).unwrap();
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

    println!("  {YELLOW}New GitHub release created: {release_name}.{RESET}");

    // region: upload asset only for executables, not for libraries

    let release_id = json_value.get("id").unwrap().as_i64().unwrap().to_string();
    println!("  {YELLOW}Now uploading release asset. This can take some time if the files are big. Wait...{RESET}");
    // Linux executable binary tar-gz-ed compress files tar.gz
    let executable_path = format!("target/release/{repo_name}");
    if std::fs::exists(&executable_path).unwrap() {
        let compressed_name = format!("{repo_name}-{tag_name_version}-x86_64-unknown-linux-gnu.tar.gz");

        cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
            r#"tar -zcvf "{compressed_name_sanitized_for_double_quote}" "{executable_path_sanitized_for_double_quote}" "#,
        )
        .unwrap_or_else(|e| panic!("{e}"))
        .arg("{compressed_name_sanitized_for_double_quote}", &compressed_name)
        .unwrap_or_else(|e| panic!("{e}"))
        .arg("{executable_path_sanitized_for_double_quote}", &executable_path)
        .unwrap_or_else(|e| panic!("{e}"))
        .run()
        .unwrap_or_else(|e| panic!("{e}"));

        // upload asset
        cgl::github_api_upload_asset_to_release(&github_owner, &repo_name, &release_id, &compressed_name);

        cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"rm "{compressed_name_sanitized_for_double_quote}" "#)
            .unwrap_or_else(|e| panic!("{e}"))
            .arg("{compressed_name_sanitized_for_double_quote}", &compressed_name)
            .unwrap_or_else(|e| panic!("{e}"))
            .run()
            .unwrap_or_else(|e| panic!("{e}"));
        println!(r#"  {YELLOW}Asset uploaded. Open and edit the description on GitHub Releases in the browser.{RESET}"#);
    }

    // Windows executable binary zipped
    // Prerequisites: Install zip into the container from the parent WSL:
    // podman exec --user=root crustde_vscode_cnt   apt-get install -y zip
    // compress file with zip because it is Windows
    let executable_path = format!("target/x86_64-pc-windows-gnu/release/{repo_name}.exe");
    if std::fs::exists(&executable_path).unwrap(){
        let compressed_name = format!("{repo_name}-{tag_name_version}-x86_64-pc-windows-gnu.zip");

        cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"zip "{compressed_name_sanitized_for_double_quote}" "{executable_path_sanitized_for_double_quote}" "#)
            .unwrap_or_else(|e| panic!("{e}"))
            .arg("{compressed_name_sanitized_for_double_quote}", &compressed_name)
            .unwrap_or_else(|e| panic!("{e}"))
            .arg("{executable_path_sanitized_for_double_quote}", &executable_path)
            .unwrap_or_else(|e| panic!("{e}"))
            .run()
            .unwrap_or_else(|e| panic!("{e}"));

        // upload asset
        cgl::github_api_upload_asset_to_release(&github_owner, &repo_name, &release_id, &compressed_name);

        cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"rm "{compressed_name_sanitized_for_double_quote}" "#)
            .unwrap_or_else(|e| panic!("{e}"))
            .arg("{compressed_name_sanitized_for_double_quote}", &compressed_name)
            .unwrap_or_else(|e| panic!("{e}"))
            .run()
            .unwrap_or_else(|e| panic!("{e}"));

        println!(r#"  {YELLOW}Asset uploaded. Open and edit the description on GitHub Releases in the browser.{RESET}"#);
    }
    // endregion: upload asset only for executables, not for libraries

    println!(r#"{GREEN}https://github.com/{github_owner}/{repo_name}/releases{RESET} "#);
}
