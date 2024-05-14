// template_new_auto_for_cli_mod.rs

//! template for new_auto_for_cli (automation_tasks_rs)
//!
//! The template is downloaded from github
//! <https://github.com/automation-tasks-rs/cargo_auto_template_new_auto_for_cli/releases/latest/download/template.tar.gz>

#[allow(unused)]
use crate::{GREEN, RED, RESET, YELLOW};

/// copies the template to the `automation_tasks_rs` directory  
/// in development use: `cargo run -- new_auto_for_cli`  
/// in runtime use: `cargo auto new_auto_for_cli`  
pub fn new_auto_for_cli() {
    copy_to_files("automation_tasks_rs");

    println!(
        r#"
    {YELLOW}The command `cargo auto new_auto_for_cli` generated the sub-project `automation_tasks_rs` in your main Rust project.
    {YELLOW}The sub-project will be automatically compiled on the next use of `cargo auto task_name` command.{RESET}
{GREEN}cargo auto build{RESET}
    {YELLOW}The new directory is automatically added to your git commit.
    There is a local .gitignore file to avoid commit of the `target/` directory.{RESET}
    the sub-project is independent from the main project. You can open it in a new Rust editor.{RESET}
{GREEN}code automation_tasks_rs{RESET}
"#
    );
}

pub fn copy_to_files(rust_project_name: &str) {
    let folder_path = std::path::Path::new(rust_project_name);
    if folder_path.exists() {
        panic!("{RED}Error: Folder {rust_project_name} already exists! {RESET}");
    }
    std::fs::create_dir_all(folder_path).unwrap();

    // download latest template.tar.gz
    println!("{YELLOW}Downloading template.tar.gz...{RESET}");
    let file_name = "template.tar.gz";
    let path = "./template.tar.gz";
    let url = "https://github.com/automation-tasks-rs/cargo_auto_template_new_auto_for_cli/releases/latest/download/template.tar.gz";
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

    // remove unused files/folders
    std::fs::remove_dir_all(folder_path.join(".github")).unwrap();
}
