// tasks_mod.rs

//! Generic functions that does not usually change.
//!
//! Don't change this code, so it can be updated regularly with
//! cargo auto update_automation_tasks_rs
//! If you want to customize it, copy the code into main.rs and modify it there.

use crate::cargo_auto_github_api_mod as cgl;
use crate::cargo_auto_lib as cl;
use crate::encrypt_decrypt_with_ssh_key_mod as ende;
// Bring trait for Result into scope.
use crate::generic_functions_mod::ResultLogError;

use crate::cargo_auto_lib::CargoTomlPublicApiMethods;
use crate::cargo_auto_lib::ShellCommandLimitedDoubleQuotesSanitizerTrait;
use anyhow::Context;
#[allow(unused_imports)]
pub use cl::{BLUE, GREEN, RED, RESET, YELLOW};

use crossplatform_path::CrossPathBuf;

#[allow(dead_code)]
/// cargo doc, then copies to /docs/ folder, because this is a GitHub standard folder
pub fn task_doc() -> anyhow::Result<()> {
    let cargo_toml = cl::CargoToml::read().log(pos!())?;
    cl::auto_cargo_toml_to_md().log(pos!())?;
    cl::auto_lines_of_code("").log(pos!())?;
    // In cargo_auto_lib we have sample data that we don't want to change, avoid this lines.
    if cargo_toml.package_name() != "cargo_auto_lib" {
        cl::auto_plantuml(&cargo_toml.package_repository().context("repository is None").log(pos!())?).log(pos!())?;
        cl::auto_playground_run_code().log(pos!())?;
    }
    cl::auto_md_to_doc_comments().log(pos!())?;

    cl::run_shell_command_static("cargo doc --no-deps --document-private-items").log(pos!())?;
    // copy target/doc into docs/ because it is GitHub standard
    cl::run_shell_command_static("rsync -a --info=progress2 --delete-after target/doc/ docs/").log(pos!())?;

    // Create simple index.html file in docs directory
    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
        r#"printf "<meta http-equiv=\"refresh\" content=\"0; url={url_sanitized_for_double_quote}/index.html\" />\n" > docs/index.html"#,
    )
    .log(pos!())?
    .arg("{url_sanitized_for_double_quote}", &cargo_toml.package_name().replace("-", "_"))
    .log(pos!())?
    .run()
    .log(pos!())?;

    // pretty html
    #[cfg(target_family = "unix")]
    cl::auto_doc_tidy_html().log(pos!())?;

    cl::run_shell_command_static("cargo fmt").log(pos!())?;
    // message to help user with next move
    println!(
        r#"
  {YELLOW}After `cargo auto doc`, ctrl-click on `docs/index.html`. 
    It will show the index.html in VSCode Explorer, then right-click and choose "Show Preview".
    This works inside the CRUSTDE container, because of the extension "Live Preview" 
    <https://marketplace.visualstudio.com/items?itemName=ms-vscode.live-server>
"#
    );
    Ok(())
}

/// commit and push
pub fn task_commit_and_push(arg_2: Option<String>) -> anyhow::Result<()> {
    let Some(message) = arg_2 else {
        eprintln!("{RED}Error: Message for commit is mandatory.{RESET}");
        // early exit
        return Ok(());
    };

    // If needed, ask to create new local git repository
    if !cl::git_is_local_repository().log(pos!())? {
        cl::new_local_repository(&message)
            .context("new_local_repository is None")
            .log(pos!())?;
    }

    // If needed, ask to create a GitHub remote repository
    if !cgl::git_has_remote().log(pos!())? || !cgl::git_has_upstream().log(pos!())? {
        cgl::new_remote_github_repository().log(pos!())?;
        cgl::description_and_topics_to_github().log(pos!())?;
    } else {
        // if description or topics/keywords/tags have changed
        cgl::description_and_topics_to_github().log(pos!())?;

        // separate commit for docs if they changed, to not make a lot of noise in the real commit
        if CrossPathBuf::new("docs").log(pos!())?.exists() {
            cl::run_shell_command_static(r#"git add docs && git diff --staged --quiet || git commit -m "update docs" "#).log(pos!())?;
        }

        cl::add_message_to_unreleased(&message).log(pos!())?;
        // the real commit of code
        cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
            r#"git add -A && git diff --staged --quiet || git commit -m "{message_sanitized_for_double_quote}" "#,
        )
        .log(pos!())?
        .arg("{message_sanitized_for_double_quote}", &message)
        .log(pos!())?
        .run()
        .log(pos!())?;

        cl::run_shell_command_static("git push").log(pos!())?;
    }
    Ok(())
}

#[allow(dead_code)]
/// create a new release on github
pub fn task_github_new_release() -> anyhow::Result<()> {
    let cargo_toml = cl::CargoToml::read().log(pos!())?;
    let version = cargo_toml.package_version();
    // take care of tags
    let tag_name_version = cl::git_tag_sync_check_create_push(&version).log(pos!())?;

    let github_owner = cargo_toml.github_owner().context("repository is None").log(pos!())?;
    let repo_name = cargo_toml.package_name();
    let now_date = cl::now_utc_date_iso();
    let release_name = format!("Version {} ({})", &version, now_date);
    let branch = "main";

    // First, the user must write the content into file RELEASES.md in the section ## Unreleased.
    // Then the automation task will copy the content to GitHub release
    let version_body_text = cl::body_text_from_releases_md()
        .context("body_text_from_releases_md is None")
        .log(pos!())?;
    // Create a new Version title and modify RELEASES.md.
    cl::create_new_version_in_releases_md(&release_name)
        .context("create_new_version_in_releases_md is None")
        .log(pos!())?;

    // Commit and push of modified Version in RELEASES.md
    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"git add -A && git commit -m "{message_sanitized_for_double_quote}" "#)
        .log(pos!())?
        .arg("{message_sanitized_for_double_quote}", &release_name)
        .log(pos!())?
        .run()
        .log(pos!())?;

    cl::run_shell_command_static("git push").log(pos!())?;

    // GitHub api call to create the Release
    let request = cgl::github_api_create_new_release(
        &github_owner,
        &repo_name,
        &tag_name_version,
        &release_name,
        branch,
        &version_body_text,
    )
    .log(pos!())?;
    let json_value = ende::github_api_token_with_oauth2_mod::send_to_github_api_with_secret_token(request).log(pos!())?;
    // early exit on error
    if let Some(error_message) = json_value.get("message") {
        eprintln!("{RED}{error_message}{RESET}");
        if let Some(errors) = json_value.get("errors") {
            let errors = errors.as_array().context("errors as array is None").log(pos!())?;
            for error in errors.iter() {
                if let Some(code) = error.get("code") {
                    eprintln!("{RED}{code}{RESET}");
                }
            }
        }
        anyhow::bail!("{RED}Call to GitHub API returned an error.{RESET}");
    }

    println!("  {YELLOW}New GitHub release created: {release_name}.{RESET}");

    // region: upload asset only for executables, not for libraries

    let release_id = json_value
        .get("id")
        .context("id is None")
        .log(pos!())?
        .as_i64()
        .context("id is None")
        .log(pos!())?
        .to_string();

    println!("  {YELLOW}Now uploading release asset. This can take some time if the files are big. Wait...{RESET}");
    std::fs::create_dir_all("tmp").log(pos!())?;
    // Linux executable binary tar-gz-ed compress files tar.gz
    let executable_path = format!("target/release/{repo_name}");
    if std::fs::exists(&executable_path).log(pos!())? {
        let compressed_name = format!("{repo_name}-{tag_name_version}-x86_64-unknown-linux-gnu.tar.gz");

        cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
            r#"tar -zcvf "tmp/{compressed_name_sanitized_for_double_quote}" "{executable_path_sanitized_for_double_quote}" "#,
        )
        .log(pos!())?
        .arg("{compressed_name_sanitized_for_double_quote}", &compressed_name)
        .log(pos!())?
        .arg("{executable_path_sanitized_for_double_quote}", &executable_path)
        .log(pos!())?
        .run()
        .log(pos!())?;

        // upload asset
        cgl::github_api_upload_asset_to_release(&github_owner, &repo_name, &release_id, &format!("tmp/{compressed_name}")).log(pos!())?;

        cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"rm "tmp/{compressed_name_sanitized_for_double_quote}" "#)
            .log(pos!())?
            .arg("{compressed_name_sanitized_for_double_quote}", &compressed_name)
            .log(pos!())?
            .run()
            .log(pos!())?;
        println!(r#"  {YELLOW}Asset uploaded. Open and edit the description on GitHub Releases in the browser.{RESET}"#);
    }

    // Windows executable binary zipped
    // Prerequisites: Install zip into the container from the parent WSL:
    // podman exec --user=root crustde_vscode_cnt   apt-get install -y zip
    // compress file with zip because it is Windows
    let executable_path = format!("target/x86_64-pc-windows-gnu/release/{repo_name}.exe");
    if std::fs::exists(&executable_path).log(pos!())? {
        let compressed_name = format!("{repo_name}-{tag_name_version}-x86_64-pc-windows-gnu.zip");

        cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
            r#"zip "tmp/{compressed_name_sanitized_for_double_quote}" "{executable_path_sanitized_for_double_quote}" "#,
        )
        .log(pos!())?
        .arg("{compressed_name_sanitized_for_double_quote}", &compressed_name)
        .log(pos!())?
        .arg("{executable_path_sanitized_for_double_quote}", &executable_path)
        .log(pos!())?
        .run()
        .log(pos!())?;

        // upload asset
        cgl::github_api_upload_asset_to_release(&github_owner, &repo_name, &release_id, &format!("tmp/{compressed_name}")).log(pos!())?;

        cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"rm "tmp/{compressed_name_sanitized_for_double_quote}" "#)
            .log(pos!())?
            .arg("{compressed_name_sanitized_for_double_quote}", &compressed_name)
            .log(pos!())?
            .run()
            .log(pos!())?;

        println!(r#"  {YELLOW}Asset uploaded. Open and edit the description on GitHub Releases in the browser.{RESET}"#);
    }
    // endregion: upload asset only for executables, not for libraries

    println!(r#"{GREEN}https://github.com/{github_owner}/{repo_name}/releases{RESET} "#);
    Ok(())
}
