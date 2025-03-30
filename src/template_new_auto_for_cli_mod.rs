// template_new_auto_for_cli_mod.rs

//! Template for new_auto_for_cli (automation_tasks_rs).
//!
//! The template is downloaded from github:  
//! <https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/releases/latest/download/automation_tasks_rs.tar.gz>

use std::ffi::OsStr;

#[allow(unused)]
use crate::{GREEN, RED, RESET, YELLOW};

/// Copies the template to the `automation_tasks_rs` directory.
///  
/// In development use: `cargo run -- new_auto_for_cli`.  
/// In runtime use: `cargo auto new_auto_for_cli`.  
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

/// Copy the Rust project into a compressed file.  
fn copy_to_files(rust_project_name: &str) {
    let folder_path = std::path::Path::new(rust_project_name);
    if folder_path.exists() {
        panic!("{RED}Error: Folder {rust_project_name} already exists! {RESET}");
    }
    std::fs::create_dir_all(folder_path).unwrap();

    // download latest template.tar.gz
    println!("  {YELLOW}Downloading template.tar.gz...{RESET}");
    std::fs::create_dir_all("tmp").unwrap();
    let path = "tmp/template.tar.gz";
    let url = "https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/releases/latest/download/automation_tasks_rs.tar.gz";
    let reqwest_client = reqwest::blocking::Client::new();
    let http_response = reqwest_client.get(url).send();
    if http_response.is_err() {
        panic!("Error while retrieving data: {:#?}", http_response.err());
    } else {
        let body = http_response.unwrap().bytes().unwrap();
        // Get the content of the response
        std::fs::write(path, &body).unwrap_or_else(|_| panic!("Download failed for {path}"));
    }

    // decompress into folder_path
    let tar_gz = std::fs::File::open(path).unwrap();
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(folder_path).unwrap();
    std::fs::remove_file(path).unwrap();
}

/// Updates the files in automation_tasks_rs.
///  
/// Downloads the template into `automation_tasks_rs_update` directory.
/// Checks what files are different. The old file changes the extension to '.old_rs'
/// Prints the diff command for different files.
pub fn update_automation_tasks_rs() {
    let update_folder = "automation_tasks_rs_update";
    let automation_folder = "automation_tasks_rs";
    if std::fs::exists(update_folder).unwrap() {
        std::fs::remove_dir_all(update_folder).unwrap();
    }
    copy_to_files(update_folder);
    // all files inside 'src' with exception of main.rs must be updated or equal
    for entry in walkdir::WalkDir::new(update_folder).min_depth(1) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let file_path_str = file_path.to_string_lossy().to_string();
            let content_1 = std::fs::read_to_string(&file_path_str).unwrap();
            let old_file_path_str = format!("{automation_folder}{}", file_path_str.trim_start_matches(update_folder));
            let content_2 = if std::fs::exists(&old_file_path_str).unwrap() {
                std::fs::read_to_string(&old_file_path_str).unwrap()
            } else {
                String::new()
            };
            if content_1 != content_2 {
                if content_2.is_empty() {
                    // the file does not yet exist
                    std::fs::copy(file_path_str, old_file_path_str).unwrap();
                } else if file_path.extension().unwrap_or_else(|| OsStr::new("")).to_string_lossy() == "rs"
                    && !file_path_str.ends_with("/main.rs")
                {
                    std::fs::rename(&old_file_path_str, format!("{old_file_path_str}_old")).unwrap();
                    std::fs::copy(&file_path_str, &old_file_path_str).unwrap();
                    println!("    {GREEN}code --diff {old_file_path_str}_old {file_path_str} {RESET}");
                } else {
                    // Some files must be different, because every automation is a little bit different.
                    // For them just write a warning to manually run the diff.
                    println!("{GREEN}code --diff {old_file_path_str} {file_path_str} {RESET}");
                }
            }
        }
    }
    println!("  {YELLOW}After manually comparing the old and new files, remove the folder and old files.{RESET}");
    println!("{GREEN}rm -r {update_folder}{RESET}");
    println!("{GREEN}rm -r {automation_folder}/**/*.rs_old {RESET}");
}
