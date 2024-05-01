// template_new_cli_mod.rs

//! template for new_cli
//!
//! The template is downloaded from github
//! <https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/releases/download/v0.0.3/template.tar.gz>

use crate::{GREEN, RED, RESET, YELLOW};

pub fn new_cli(arg_2: Option<String>, arg_3: Option<String>) {
    if arg_2.is_none() {
        println!("{RED}Error: Project name argument is missing: `cargo auto new_cli project_name github_owner_or_organization`{RESET}");
        return;
    }
    if arg_3.is_none() {
        println!("{RED}Error: Github owner argument is missing: `cargo auto new_cli project_name github_owner_or_organization`{RESET}");
        return;
    }
    let project_name = arg_2.unwrap();
    let github_owner_or_organization = arg_3.unwrap();

    copy_to_files(&project_name, &github_owner_or_organization);

    println!("");
    println!("    {YELLOW}The command `cargo auto new_cli` generated the directory `{project_name}`.{RESET}");
    println!("    {YELLOW}You can open this new Rust project in VSCode:{RESET}",);
    println!("{GREEN}code {project_name}{RESET}");
    println!("    {YELLOW}Then build inside the VSCode terminal with:{RESET}");
    println!("{GREEN}cargo auto build{RESET}");
    println!("    {YELLOW}and follow the detailed instructions.{RESET}");
}

pub fn copy_to_files(project_name: &str, github_owner_or_organization: &str) {
    let folder_path = std::path::Path::new(project_name);
    if folder_path.exists() {
        panic!("{RED}Error: Folder {project_name} already exists! {RESET}");
    }
    std::fs::create_dir_all(folder_path).unwrap();

    // download latest template.tar.gz
    println!("{YELLOW}Downloading template.tar.gz...{RESET}");
    let file_name = "template.tar.gz";
    let path = "./template.tar.gz";
    let url = "https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/releases/download/v0.0.3/template.tar.gz";
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
            let content = content.replace("cargo_auto_template_new_cli", project_name);
            let content = content.replace("automation-tasks-rs", github_owner_or_organization);
            let content = content.replace("automation--tasks--rs", "automation-tasks-rs");
            std::fs::write(entry.path(), content).unwrap();
        }
    }
    // renaming files is tricky and must be traverse  in reverse.
    let mut traverse_reverse: Vec<walkdir::DirEntry> = walkdir::WalkDir::new(folder_path).into_iter().filter_map(Result::ok).collect();
    traverse_reverse.reverse();
    for entry in traverse_reverse.iter() {
        if entry.file_name() == "cargo_auto_template_new_cli" {
            println!("rename: {}", entry.path().to_string_lossy());
            std::fs::rename(entry.path(), entry.path().parent().unwrap().join(project_name)).unwrap();
        }
    }
}
