// template_new_wasm_mod.rs

//! template for new_wasm
//!
//! The template is downloaded from github
//! <https://github.com/automation-tasks-rs/cargo_auto_template_new_wasm/releases/download/v0.0.2/template.tar.gz>

use crate::{GREEN, RED, RESET, YELLOW};

pub fn new_wasm(rust_project_name: Option<String>, github_owner_or_organization: Option<String>, web_server_domain: Option<String>, server_username: Option<String>) {
    if rust_project_name.is_none() {
        println!("{RED}Error: Project name argument is missing: `cargo auto new_wasm project_name github_owner_or_organization web_server server_username`{RESET}");
        return;
    }
    if github_owner_or_organization.is_none() {
        println!("{RED}Error: github_owner or Organization argument is missing: `cargo auto new_wasm project_name github_owner_or_organization web_server server_username`{RESET}");
        return;
    }
    if web_server_domain.is_none() {
        println!("{RED}Error: Web server argument is missing: `cargo auto new_wasm project_name github_owner_or_organization web_server server_username`{RESET}");
        return;
    }
    if server_username.is_none() {
        println!("{RED}Error: Server username argument is missing: `cargo auto new_wasm project_name github_owner_or_organization web_server server_username`{RESET}");
        return;
    }
    let rust_project_name = rust_project_name.unwrap();
    let github_owner_or_organization = github_owner_or_organization.unwrap();
    let web_server_domain = web_server_domain.unwrap();
    let server_username = server_username.unwrap();

    copy_to_files(&rust_project_name, &github_owner_or_organization, &web_server_domain, &server_username);

    println!("");
    println!("    {YELLOW}The command `cargo auto new_wasm` generated the directory `{rust_project_name}`{RESET}");
    println!("    {YELLOW}You can open this new Rust project `{rust_project_name}` in a new Rust editor.{RESET}",);
    println!("    {YELLOW}For example VSCode:{RESET}");
    println!("{GREEN}code {rust_project_name}{RESET}");
    println!("    {YELLOW}Then build with:{RESET}");
    println!("{GREEN}cargo auto build{RESET}");
    println!("    {YELLOW}and follow the detailed instructions.{RESET}");
}

pub fn copy_to_files(rust_project_name: &str, github_owner_or_organization: &str, web_server_domain: &str, server_username: &str) {
    let folder_path = std::path::Path::new(rust_project_name);
    if folder_path.exists() {
        panic!("{RED}Error: Folder {rust_project_name} already exists! {RESET}");
    }
    std::fs::create_dir_all(folder_path).unwrap();

    // download latest template.tar.gz
    println!("{YELLOW}Downloading template.tar.gz...{RESET}");
    let file_name = "template.tar.gz";
    let path = "./template.tar.gz";
    let url = "https://github.com/automation-tasks-rs/cargo_auto_template_new_wasm/releases/download/v0.0.2/template.tar.gz";
    let reqwest_client = reqwest::blocking::Client::new();
    let http_response = reqwest_client.get(url).send();
    if http_response.is_err() {
        panic!("Error while retrieving data: {:#?}", http_response.err());
    } else {
        let body = http_response.unwrap().bytes().unwrap();
        // Get the content of the response
        std::fs::write(path, &body).expect(&format!("Download failed for {file_name}"));
    }

    // decompress into folder_path
    let tar_gz = std::fs::File::open(path).unwrap();
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(folder_path).unwrap();
    std::fs::remove_file(path).unwrap();

    // replace placeholders inside text files
    for entry in walkdir::WalkDir::new(folder_path).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            // template has only valid utf8 files
            println!("replace: {}", entry.path().to_string_lossy());
            let content = std::fs::read_to_string(entry.path()).unwrap();
            let content = content.replace("cargo_auto_template_new_wasm", rust_project_name);
            let content = content.replace("automation-tasks-rs", github_owner_or_organization);
            let content = content.replace("automation--tasks--rs", "automation-tasks-rs");
            let content = content.replace("web_server_domain", web_server_domain);
            let content = content.replace("server_username", server_username);
            std::fs::write(entry.path(), content).unwrap();
        }
    }
    // renaming files is tricky and must be traverse  in reverse.
    let mut traverse_reverse: Vec<walkdir::DirEntry> = walkdir::WalkDir::new(folder_path).into_iter().filter_map(Result::ok).collect();
    traverse_reverse.reverse();
    for entry in traverse_reverse.iter() {
        if entry.file_name() == "cargo_auto_template_new_wasm" {
            println!("rename: {}", entry.path().to_string_lossy());
            std::fs::rename(entry.path(), entry.path().parent().unwrap().join(rust_project_name)).unwrap();
        }
    }
}
