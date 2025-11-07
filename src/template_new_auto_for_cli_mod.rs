// template_new_auto_for_cli_mod.rs

//! Template for new_auto_for_cli (automation_tasks_rs).
//!
//! The template is downloaded from github:  
//! <https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/releases/latest/download/automation_tasks_rs.tar.gz>

use crossplatform_path::CrossPathBuf;

#[allow(unused)]
use crate::{GREEN, RED, RESET, YELLOW};

// Bring trait for Result into scope.
use crate::{pos, ResultLogError};

/// Copies the template to the `automation_tasks_rs` directory.  \
///  
/// In development use: `cargo run -- new_auto_for_cli`.  \
/// In runtime use: `cargo auto new_auto_for_cli`.  
pub fn new_auto_for_cli() -> anyhow::Result<()> {
    let destination_folder = CrossPathBuf::new("automation_tasks_rs").log(pos!())?;
    download_decompress_and_copy_files(&destination_folder, "template.tar.gz").log(pos!())?;

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

/// Download the compressed file and decompress and copy into destination folder.  
fn download_decompress_and_copy_files(destination_folder: &CrossPathBuf, file_to_download: &str) -> anyhow::Result<()> {
    if destination_folder.exists() {
        anyhow::bail!("{RED}Error: Folder {destination_folder} already exists! {RESET}");
    }

    // download latest template.tar.gz or automation_tasks_rs.tar.gz
    println!("  {YELLOW}Downloading {file_to_download}...{RESET}");
    let tmp_folder_path = CrossPathBuf::new("tmp").log(pos!())?;
    tmp_folder_path.create_dir_all().log(pos!())?;
    let file_path = CrossPathBuf::new(&format!("tmp/{file_to_download}")).log(pos!())?;
    let url = format!("https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/releases/latest/download/{file_to_download}");
    let reqwest_client = reqwest::blocking::Client::new();
    let http_response = reqwest_client.get(&url).send();
    if let Ok(body) = http_response {
        let body = body.bytes().log(pos!())?;
        // Get the content of the response
        file_path.write_bytes_to_file(&body).log(pos!())?;
    } else {
        anyhow::bail!("Error while retrieving data: {:#?}", http_response.err());
    }

    // decompress into folder_path
    file_path.decompress_tar_gz(destination_folder).log(pos!())?;
    file_path.remove_file().log(pos!())?;
    Ok(())
}

/// Updates the files in automation_tasks_rs.  \
///  
/// Downloads the template into `automation_tasks_rs_update` directory.  \
/// Checks what files are different. The old file are moved to the tmp folder 'automation_tasks_rs_old_date'.  \
/// Prints the diff command for different files.  
pub fn update_automation_tasks_rs() -> anyhow::Result<()> {
    println!("  {YELLOW}cargo auto update_automation_tasks_rs {RESET}");
    let update_folder = CrossPathBuf::new("tmp/automation_tasks_rs_update/").log(pos!())?;
    update_folder.remove_dir_all().log(pos!())?;
    download_decompress_and_copy_files(&update_folder, "automation_tasks_rs.tar.gz").log(pos!())?;
    let update_folder_str = update_folder.as_str();

    let utc_now = chrono::Utc::now();
    // 2018_01_26T18_30_09Z
    let utc_now = utc_now
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
        .replace(":", "_")
        .replace("-", "_");
    let old_rs_folder = CrossPathBuf::new(&format!("tmp/automation_tasks_rs_old_{utc_now}/")).log(pos!())?;
    old_rs_folder.create_dir_all().log(pos!())?;

    // all files inside 'src' with exception of main.rs must be updated or equal
    let mut vec_updated_diff_files = vec![];
    let mut vec_other_diff_files = vec![];
    for entry in walkdir::WalkDir::new(update_folder.to_path_buf_current_os()).min_depth(1) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let file_path = CrossPathBuf::from_path(entry.path()).log(pos!())?;
            // tracing::debug!(?file_path);
            let content_1 = file_path.read_to_string().log(pos!())?;

            let relative_path = file_path.as_str().trim_start_matches(update_folder_str);
            let old_file_path_str = CrossPathBuf::new(&format!("automation_tasks_rs/{relative_path}")).log(pos!())?;
            let old_file_move_destination = CrossPathBuf::new(&format!("{old_rs_folder}{relative_path}")).log(pos!())?;
            // tracing::debug!(?old_file_move_destination);
            let content_2 = if old_file_path_str.exists() {
                old_file_path_str.read_to_string()?
            } else {
                String::new()
            };
            if content_1 != content_2 {
                if content_2.is_empty() {
                    // the file does not yet exist, maybe even the folder does not exist
                    let subfolder = old_file_path_str.parent().log(pos!())?;
                    subfolder.create_dir_all().log(pos!())?;
                    println!("copy {file_path}, {old_file_path_str};");
                    file_path.copy_file_to_file(&old_file_path_str).log(pos!())?;
                } else if file_path.extension()? == "rs" && !file_path.as_str().ends_with("/main.rs") {
                    old_file_path_str.rename_or_move(&old_file_move_destination).log(pos!())?;
                    file_path.copy_file_to_file(&old_file_path_str).log(pos!())?;
                    vec_updated_diff_files.push(format!("{GREEN}code --diff {old_file_move_destination} {file_path} {RESET}\n"));
                } else {
                    // Some files must be different, because every automation is a little bit different.
                    // For them just write a warning to manually run the diff.
                    vec_other_diff_files.push(format!("{GREEN}code --diff {file_path} {old_file_path_str} {RESET}\n"));
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
