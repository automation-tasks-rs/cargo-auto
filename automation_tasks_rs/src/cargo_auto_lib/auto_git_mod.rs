// auto_git_mod

//! Functions to work with git from automation_tasks_rs.

use crate::cargo_auto_lib::error_mod::{Error, Result};
use crate::cargo_auto_lib::public_api_mod::{BLUE, RED, RESET};
use crate::generic_functions_mod::{pos, ResultLogError};

/// Does git have settings for remote.
pub fn git_has_remote() -> Result<bool> {
    // it returns only "origin" if exists or nothing if it does not exist
    let output = std::process::Command::new("git").arg("remote").output().log(pos!())?;
    // return
    Ok(!(String::from_utf8(output.stdout).log(pos!())?).is_empty())
}

/// Check if this folder is a local Git repository.
pub fn git_is_local_repository() -> Result<bool> {
    let output = std::process::Command::new("git").arg("status").output().log(pos!())?;
    let output = String::from_utf8(output.stderr).log(pos!())?;
    // return bool
    Ok(!output.contains("not a git repository"))
}

/// Return Url to repository: <https://github.com/automation-tasks-rs/auto_lines_of_code/>.  
///
/// Get the output string after $ git remote -v.  
/// Then finds out the link to the repository with regex.  
/// Returns empty string if something goes wrong: no git, no remote,...  
pub fn process_git_remote() -> String {
    /// Internal function for git remote
    fn git_remote_output() -> Result<String> {
        let output = std::process::Command::new("git").arg("remote").arg("-v").output().log(pos!())?;

        let output = String::from_utf8(output.stdout).log(pos!())?;
        // return
        Ok(output)
    }

    /// Internal function returns remote repository url
    ///
    /// on GitHub actions they don't use SSH, but https, I need to check that also
    /// I test my regex on https://regex101.com/
    /// regex capture 3 groups: website, user_name and repo_name
    /// "origin  git@github.com:automation-tasks-rs/auto_lines_of_code.git (fetch)"
    /// origin    https://github.com/automation-tasks-rs/auto_lines_of_code (fetch)
    /// println!("{}", &output);
    fn regex_capture(output: String) -> Result<String> {
        let reg = regex::Regex::new(
            r#"origin\s*(?:https://)?(?:git@)?([^:/]*?)[:/]([^/]*?)/([^. ]*?)(?:\.git)?\s*\(fetch\)"#,
        )
        .log(pos!())?;
        let cap = reg
            .captures(&output)
            .ok_or(Error::ErrorFromStr("Error: reg.captures is None"))
            .log(pos!())?;

        if cap.len() != 4 {
            return Err(Error::ErrorFromStr(
                "Error: cap len is not 4, because there are 4 capture groups in regex.",
            ));
        }
        Ok(format!("https://{}/{}/{}/", &cap[1], &cap[2], &cap[3]))
    }

    let output = match git_remote_output() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{RED}{e}{RESET}");
            return "".to_string();
        }
    };
    match regex_capture(output) {
        Ok(s) => s,
        Err(_e) => {
            // eprintln!("{RED}process_git_remote error: {}{RESET}", e);
            "".to_string()
        }
    }
}

/// Interactive ask to create a new local git repository.
pub fn new_local_repository(message: &str) -> Result<()> {
    // ask interactive
    println!("{BLUE}This project folder is not yet a Git repository.{RESET}");
    let answer = inquire::Text::new(&format!("{BLUE}Do you want to initialize a new local git repository? (y/n){RESET}"))
        .prompt()
        .log(pos!())?;
    // continue if answer is "y"
    if answer.to_lowercase() != "y" {
        // early exit
        return Err(Error::ErrorFromStr("Ok. You don't want to initialize a new local git repository."));
    }

    // the docs folder is mandatory because of GitHub action for pages deployment
    if !camino::Utf8Path::new("docs").exists() {
        std::fs::create_dir("docs").log(pos!())?;
        std::fs::write("docs/index.html", "project docs").log(pos!())?;
    }

    // create new local git repository and commit all on branch main
    crate::cargo_auto_lib::run_shell_command_static("git config --global init.defaultBranch main").log(pos!())?;
    crate::cargo_auto_lib::run_shell_command_static("git init").log(pos!())?;
    crate::cargo_auto_lib::run_shell_command_static("git add .").log(pos!())?;
    crate::cargo_auto_lib::run_shell_command(&format!(r#"git commit -m "{message}""#)).log(pos!())?;
    crate::cargo_auto_lib::run_shell_command_static("git branch -M main").log(pos!())?;
    Ok(())
}
