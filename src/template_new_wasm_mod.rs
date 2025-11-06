// template_new_wasm_mod.rs

//! Template for new_wasm.
//!
//! The template is downloaded from github:  
//! <https://github.com/automation-tasks-rs/cargo_auto_template_new_wasm/releases/latest/download/template.tar.gz>

use crate::{ResultLogError, GREEN, RED, RESET, YELLOW};

/// Creates a new Rust project from template.
pub fn new_wasm(
    rust_project_name: Option<String>,
    github_owner_or_organization: Option<String>,
    web_server_domain: Option<String>,
    server_username: Option<String>,
) -> anyhow::Result<()> {
    if rust_project_name.is_none() {
        println!("{RED}Error: Project name argument is missing: `cargo auto new_wasm project_name github_owner_or_organization web_server server_username`{RESET}");
        return Ok(());
    }
    if github_owner_or_organization.is_none() {
        println!("{RED}Error: github_owner or Organization argument is missing: `cargo auto new_wasm project_name github_owner_or_organization web_server server_username`{RESET}");
        return Ok(());
    }
    if web_server_domain.is_none() {
        println!("{RED}Error: Web server argument is missing: `cargo auto new_wasm project_name github_owner_or_organization web_server server_username`{RESET}");
        return Ok(());
    }
    if server_username.is_none() {
        println!("{RED}Error: Server username argument is missing: `cargo auto new_wasm project_name github_owner_or_organization web_server server_username`{RESET}");
        return Ok(());
    }
    use anyhow::Context;
    let rust_project_name = rust_project_name.context("rust_project_name is None").log()?;
    let github_owner_or_organization = github_owner_or_organization.context("github_owner_or_organization is None").log()?;
    let web_server_domain = web_server_domain.context("web_server_domain is None").log()?;
    let server_username = server_username.context("server_username is None").log()?;

    copy_to_files(
        &rust_project_name,
        &github_owner_or_organization,
        &web_server_domain,
        &server_username,
    )
    .log()?;

    println!();
    println!("  {YELLOW}The command `cargo auto new_wasm` generated the directory `{rust_project_name}`{RESET}");
    println!("  {YELLOW}You can open this new Rust project `{rust_project_name}` in a new Rust editor.{RESET}",);
    println!("  {YELLOW}For example VSCode:{RESET}");
    println!("{GREEN}code {rust_project_name}{RESET}");
    println!("  {YELLOW}Then build with:{RESET}");
    println!("{GREEN}cargo auto build{RESET}");
    println!("  {YELLOW}and follow the detailed instructions.{RESET}");
    Ok(())
}

/// Copy the Rust project into a compressed file.  
fn copy_to_files(
    rust_project_name: &str,
    github_owner_or_organization: &str,
    web_server_domain: &str,
    server_username: &str,
) -> anyhow::Result<()> {
    let folder_path = std::path::Path::new(rust_project_name);
    if folder_path.exists() {
        anyhow::bail!("{RED}Error: Folder {rust_project_name} already exists! {RESET}");
    }
    std::fs::create_dir_all(folder_path).log()?;

    // download latest template.tar.gz
    println!("  {YELLOW}Downloading template.tar.gz...{RESET}");
    let file_name = "template.tar.gz";
    let path = "./template.tar.gz";
    let url = "https://github.com/automation-tasks-rs/cargo_auto_template_new_wasm/releases/latest/download/template.tar.gz";
    let reqwest_client = reqwest::blocking::Client::new();
    let http_response = reqwest_client.get(url).send();
    if let Ok(body) = http_response {
        let body = body.bytes().log()?;
        // Get the content of the response
        std::fs::write(path, &body)
            .or_else(|err| anyhow::bail!("Download failed for {file_name} {err}"))
            .log()?;
    } else {
        anyhow::bail!("Error while retrieving data: {:#?}", http_response.err());
    }

    // decompress into folder_path
    let tar_gz = std::fs::File::open(path).log()?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(folder_path).log()?;
    std::fs::remove_file(path).log()?;

    // replace placeholders inside text files
    for entry in walkdir::WalkDir::new(folder_path).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            // template has only valid utf8 files
            println!("replace: {}", entry.path().to_string_lossy());
            let content = std::fs::read_to_string(entry.path()).log()?;
            let content = content.replace("cargo_auto_template_new_wasm", rust_project_name);
            let content = content.replace(&"cargo_auto_template_new_wasm".to_uppercase(), &rust_project_name.to_uppercase());
            let content = content.replace("automation-tasks-rs", github_owner_or_organization);
            let content = content.replace("automation--tasks--rs", "automation-tasks-rs");
            let content = content.replace("web_server_domain", web_server_domain);
            let content = content.replace("server_username", server_username);
            std::fs::write(entry.path(), content).log()?;
        }
    }
    // renaming files is tricky and must be traverse  in reverse.
    let mut traverse_reverse: Vec<walkdir::DirEntry> = walkdir::WalkDir::new(folder_path).into_iter().filter_map(Result::ok).collect();
    traverse_reverse.reverse();
    for entry in traverse_reverse.iter() {
        if entry.file_name().to_string_lossy().contains("cargo_auto_template_new_wasm") {
            println!("rename: {}", entry.path().to_string_lossy());
            std::fs::rename(
                entry.path(),
                entry
                    .path()
                    .to_string_lossy()
                    .replace("cargo_auto_template_new_wasm", rust_project_name),
            )
            .log()?;
        }
    }
    Ok(())
}
