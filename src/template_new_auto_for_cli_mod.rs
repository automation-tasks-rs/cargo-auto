// template_new_auto_for_cli_mod.rs

//! Template for new_auto_for_cli (automation_tasks_rs).
//!
//! The template is downloaded from github:  
//! <https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/releases/latest/download/automation_tasks_rs.tar.gz>

use std::ffi::OsStr;

use anyhow::Context;

#[allow(unused)]
use crate::{GREEN, RED, RESET, YELLOW};

/// Copies the template to the `automation_tasks_rs` directory.
///  
/// In development use: `cargo run -- new_auto_for_cli`.  
/// In runtime use: `cargo auto new_auto_for_cli`.  
pub fn new_auto_for_cli() -> anyhow::Result<()> {
    copy_to_files("automation_tasks_rs", "template.tar.gz")?;

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
    Ok(())
}

/// Copy the Rust project into a compressed file.  
fn copy_to_files(rust_project_name: &str, file_to_download: &str) -> anyhow::Result<()> {
    let folder_path = std::path::Path::new(rust_project_name);
    if folder_path.exists() {
        anyhow::bail!("{RED}Error: Folder {rust_project_name} already exists! {RESET}");
    }
    std::fs::create_dir_all(folder_path)?;

    // download latest template.tar.gz or automation_tasks_rs.tar.gz
    println!("  {YELLOW}Downloading {file_to_download}...{RESET}");
    std::fs::create_dir_all("tmp")?;
    let path = &format!("tmp/{file_to_download}");
    let url = format!("https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/releases/latest/download/{file_to_download}");
    let reqwest_client = reqwest::blocking::Client::new();
    let http_response = reqwest_client.get(&url).send();
    if let Ok(body) = http_response {
        let body = body.bytes()?;
        // Get the content of the response
        std::fs::write(path, &body).or_else(|err| anyhow::bail!("Download failed for {path} {err}"))?;
    } else {
        anyhow::bail!("Error while retrieving data: {:#?}", http_response.err());
    }

    // decompress into folder_path
    let tar_gz = std::fs::File::open(path)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(folder_path)?;
    std::fs::remove_file(path)?;
    Ok(())
}

/// Updates the files in automation_tasks_rs.
///  
/// Downloads the template into `automation_tasks_rs_update` directory.
/// Checks what files are different. The old file are moved to the tmp folder 'automation_tasks_rs_old_date'.
/// Prints the diff command for different files.
pub fn update_automation_tasks_rs() -> anyhow::Result<()> {
    println!("  {YELLOW}cargo auto update_automation_tasks_rs {RESET}");
    std::fs::create_dir_all("tmp/automation_tasks_rs_update")?;
    let utc_now = chrono::Utc::now();
    // 2018_01_26T18_30_09Z
    let utc_now = utc_now
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
        .replace(":", "_")
        .replace("-", "_");
    let old_rs_folder = format!("tmp/automation_tasks_rs_old_{utc_now}/");
    std::fs::create_dir_all(&old_rs_folder)?;
    // must end with the slash
    let update_folder = "tmp/automation_tasks_rs_update/".to_string();
    if std::fs::exists(&update_folder)? {
        std::fs::remove_dir_all(&update_folder)?;
    }
    copy_to_files(&update_folder, "automation_tasks_rs.tar.gz")?;
    // all files inside 'src' with exception of main.rs must be updated or equal
    let mut vec_updated_diff_files = vec![];
    let mut vec_other_diff_files = vec![];
    for entry in walkdir::WalkDir::new(&update_folder).min_depth(1) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let file_path_str = file_path.to_string_lossy().to_string();
            let content_1 = std::fs::read_to_string(&file_path_str)?;
            let old_file_path_str = format!("automation_tasks_rs/{}", file_path_str.trim_start_matches(&update_folder));
            let old_file_move_destination = format!("{old_rs_folder}{}", file_path_str.trim_start_matches(&update_folder));
            let content_2 = if std::fs::exists(&old_file_path_str)? {
                std::fs::read_to_string(&old_file_path_str)?
            } else {
                String::new()
            };
            if content_1 != content_2 {
                if content_2.is_empty() {
                    // the file does not yet exist, maybe even the folder does not exist
                    let subfolder = std::path::Path::new(&old_file_path_str).parent().context("No parent")?;
                    if !subfolder.exists() {
                        std::fs::create_dir(subfolder)?;
                    }
                    println!("copy {file_path_str}, {old_file_path_str};");
                    std::fs::copy(file_path_str, old_file_path_str)?;
                } else if file_path.extension().unwrap_or_else(|| OsStr::new("")).to_string_lossy() == "rs"
                    && !file_path_str.ends_with("/main.rs")
                {
                    std::fs::rename(&old_file_path_str, &old_file_move_destination)?;
                    std::fs::copy(&file_path_str, &old_file_path_str)?;
                    vec_updated_diff_files.push(format!("{GREEN}code --diff {old_file_move_destination} {file_path_str} {RESET}\n"));
                } else {
                    // Some files must be different, because every automation is a little bit different.
                    // For them just write a warning to manually run the diff.
                    vec_other_diff_files.push(format!("{GREEN}code --diff {file_path_str} {old_file_path_str} {RESET}\n"));
                }
            }
        }
    }
    println!("  {YELLOW}All '.rs' files will be updated except for 'main.rs'.{RESET}");
    println!("  {YELLOW}You can diff between the 'old' and 'new' files to see what has changed.'.{RESET}");
    if !vec_updated_diff_files.is_empty() {
        println!();
        println!("{}", vec_updated_diff_files.concat());
    }
    println!();
    println!("  {YELLOW}Other files will have custom user tasks or information and will not be updated automatically.{RESET}");
    println!("  {YELLOW}You can diff those to see what is the new default content and choose what to update or not.{RESET}");
    if !vec_other_diff_files.is_empty() {
        println!();
        println!("{}", vec_other_diff_files.concat());
    }
    println!();
    println!("  {YELLOW}After manually diffing the files, remove the obsolete files.{RESET}");
    println!("{GREEN}rm -r {update_folder}{RESET}");
    println!("{GREEN}rm -r {old_rs_folder}{RESET}");
    Ok(())
}
