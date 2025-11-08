// auto_helper_functions_mod

//! Various helper functions.

use crate::cargo_auto_lib::error_mod::{Error, Result};
use crate::cargo_auto_lib::public_api_mod::{RED, RESET};
use crate::utils_mod::{pos, ResultLogError};

// region: auto_md_to_doc_comments include doc_comments/exit_if_not_run_in_rust_project_root_directory.md A ///
/// Check if the code was run inside the Rust project root directory.  
///
/// There must be the `Cargo.toml` file and the directory `automation_tasks_rs`  
/// If not, exit with error message.  
///
// endregion: auto_md_to_doc_comments include doc_comments/exit_if_not_run_in_rust_project_root_directory.md A ///
pub fn exit_if_not_run_in_rust_project_root_directory() {
    if !(camino::Utf8Path::new("automation_tasks_rs").exists() && (camino::Utf8Path::new("Cargo.toml").exists())) {
        eprintln!("{RED}Error: `automation_tasks_rs` must be run inside the Rust project in the dir that contains");
        println!("`Cargo.toml` file and `automation_tasks_rs` directory. Exiting...{RESET}");
        // early exit
        std::process::exit(1);
    }
}

/// Print one or more sub_commands.
pub fn completion_return_one_or_more_sub_commands(sub_commands: Vec<&str>, word_being_completed: &str) {
    let mut sub_found = false;
    for sub_command in sub_commands.iter() {
        if sub_command.starts_with(word_being_completed) {
            println!("{sub_command}");
            sub_found = true;
        }
    }
    if !sub_found {
        // print all sub-commands
        for sub_command in sub_commands.iter() {
            println!("{sub_command}");
        }
    }
}

/// Get home dir using the home crate.
///
/// Error if HOME not found.
pub fn home_dir() -> Result<std::path::PathBuf> {
    match home::home_dir() {
        Some(path_buff) => {
            if !path_buff.as_os_str().is_empty() {
                Ok(path_buff)
            } else {
                Err(Error::ErrorFromStr("{RED}Unable to get your home dir!{RESET}"))
            }
        }
        None => Err(Error::ErrorFromStr("{RED}Unable to get your home dir!{RESET}")),
    }
}

/// Replace tilde with home::home_dir, only for utf8.
pub fn tilde_expand_to_home_dir_utf8(path_str: &str) -> Result<camino::Utf8PathBuf> {
    let mut expanded = String::new();
    if path_str.starts_with("~") {
        let base = home::home_dir()
            .ok_or_else(|| Error::ErrorFromStr("Cannot find home_dir in this OS."))
            .log(pos!())?;
        // only utf8 is accepted
        let base = base.to_string_lossy();
        expanded.push_str(&base);
        expanded.push_str(path_str.trim_start_matches("~"));
        use std::str::FromStr;
        Ok(camino::Utf8PathBuf::from_str(&expanded).log(pos!())?)
    } else {
        use std::str::FromStr;
        Ok(camino::Utf8PathBuf::from_str(path_str).log(pos!())?)
    }
}
