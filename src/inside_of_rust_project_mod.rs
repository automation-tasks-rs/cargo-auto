// inside_of_rust_project_mod.rs

//! Commands accessible inside a Rust project folder.

// region: use statements

// endregion

use crate::{ResultLogError, GREEN, RED, RESET, YELLOW};

/// Parse when it is run inside a Rust project.  
pub fn parse_args(args: &mut std::env::Args) -> anyhow::Result<()> {
    // the first argument is the task: (no argument for help), new_auto_for_cli, ...
    // wooow! There is a difference if I call the standalone binary or as a cargo subcommand:
    let arg_1 = args.next();
    match arg_1 {
        None => print_help_from_cargo_auto()?,
        Some(task) => {
            if task != "auto" {
                // when calling as `cargo auto new_auto_for_cli`
                match_first_argument(&task, args).log()?;
            } else {
                // when calling as `cargo-auto new_auto_for_cli`
                let arg_2 = args.next();
                match arg_2 {
                    None => print_help_from_cargo_auto()?,
                    Some(task) => match_first_argument(&task, args)?,
                }
            }
        }
    }
    Ok(())
}

/// Is there already the automation_tasks_rs directory.
fn already_exists_automation_tasks_rs() -> bool {
    // return
    crate::PATH_AUTOMATION_TASKS_RS.exists()
}

/// If there is no argument then print help.  \
///   
/// If there exists `automation_tasks_rs/Cargo.toml` and `automation_tasks_rs/src/main.rs`  \
/// call automation_tasks_rs with no arguments to print the help prepared in user defined automation_tasks_rs.  \
/// Else print the help for `cargo auto new_auto_for_cli`.  \
/// In development use: `cargo run`.  \
/// In runtime use: `cargo auto`.  
fn print_help_from_cargo_auto() -> anyhow::Result<()> {
    if !crate::PATH_CARGO_TOML.exists() || !crate::PATH_SRC_MAIN_RS.exists() {
        println!(
            r#"
  {YELLOW}Welcome to cargo-auto !
    This program automates your custom tasks when developing a Rust project.
        
    To start using `cargo auto` inside your Rust project, you must create a new `automation_tasks_rs` directory with the command:{RESET}
{GREEN}cargo auto new_auto_for_cli{RESET}

  {YELLOW}© 2025 bestia.dev  MIT License github.com/automation-tasks-rs/cargo-auto{RESET}
"#
        );
    } else {
        // Error if cannot compile automation_tasks_rs
        compile_automation_tasks_rs_if_needed().log()?;
        let _success = crate::utils_mod::run_shell_command_success(&crate::PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS.to_string_lossy());
    }
    Ok(())
}

/// Get the first argument is the task: new_auto_for_cli.  \
///
/// The task `new` is processed by `cargo-auto`.  \
/// All other tasks are processed by the used defined `automation_tasks_rs`.  \
/// In development use: `cargo run -- new_auto_for_cli`.  
fn match_first_argument(task: &str, args: &mut std::env::Args) -> anyhow::Result<()> {
    if task == "completion" {
        completion();
    } else if task == "new_auto_for_cli" {
        // this task is inside cargo-auto
        if already_exists_automation_tasks_rs() {
            println!("{RED}Error: Directory automation_tasks_rs already exists. Cannot create new directory automation_tasks_rs.{RESET}");
            // early exit
            std::process::exit(0);
        }
        crate::template_new_auto_for_cli_mod::new_auto_for_cli().log()?;
    } else if task == "update_automation_tasks_rs" {
        // this task is inside cargo-auto
        if !already_exists_automation_tasks_rs() {
            println!("{RED}Error: Directory automation_tasks_rs does not exists. Use 'cargo auto new_auto_for_cli'.{RESET}");
            // early exit
            std::process::exit(0);
        }
        crate::template_new_auto_for_cli_mod::update_automation_tasks_rs().log()?;
    } else {
        // these tasks are user defined in automation_tasks_rs
        if !already_exists_automation_tasks_rs() {
            println!("{RED}Error: Directory automation_tasks_rs does not exist.{RESET}");
            print_help_from_cargo_auto().log()?;
            // early exit
            std::process::exit(0);
        }
        // Error if cannot compile automation_tasks_rs
        compile_automation_tasks_rs_if_needed().log()?;
        // call automation_tasks_rs/target/debug/automation_tasks_rs with all the arguments
        let mut command = std::process::Command::new(crate::PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS.as_os_str());
        command.arg(task);
        for arg_x in args.by_ref() {
            command.arg(&arg_x);
        }
        let mut child = command.spawn().log()?;
        child.wait().log()?;
    }
    Ok(())
}

/// Sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`.
fn completion() {
    /// println one, more or all sub_commands
    fn completion_return_one_or_more_sub_commands(sub_commands: Vec<&str>, word_being_completed: &str) {
        let mut sub_found = false;
        for sub_command in sub_commands.iter() {
            if sub_command.starts_with(word_being_completed) {
                println!("{}", sub_command);
                sub_found = true;
            }
        }
        if !sub_found {
            // print all sub-commands
            for sub_command in sub_commands.iter() {
                println!("{}", sub_command);
            }
        }
    }

    let args: Vec<String> = std::env::args().collect();
    let last_word = args[2].as_str();
    let mut word_being_completed = " ";
    if args.len() > 3 {
        word_being_completed = args[3].as_str();
    }
    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["new_auto_for_cli"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
}

/// Build if the files are different then the hashes in automation_tasks_rs/file_hashes.json.  \
///
/// Error if cannot compile automation_tasks_rs.  
pub fn compile_automation_tasks_rs_if_needed() -> anyhow::Result<()> {
    if !crate::PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS.exists() || crate::file_hashes_mod::is_project_changed()? {
        compile_project_automation_tasks_rs().log()?;
        let vec_of_metadata = crate::file_hashes_mod::read_file_metadata().log()?;
        crate::file_hashes_mod::save_json_file_for_file_meta_data(vec_of_metadata).log()?;
    }
    Ok(())
}

/// Build automation_tasks_rs.  \
///
/// Error if cannot compile automation_tasks_rs.  
pub fn compile_project_automation_tasks_rs() -> anyhow::Result<()> {
    // build in other directory (not in working current directory)
    // cargo build --manifest-path=dir/Cargo.toml
    if !crate::utils_mod::run_shell_command_success("cargo build --manifest-path=automation_tasks_rs/Cargo.toml") {
        anyhow::bail!("{RED}Cannot compile automation_tasks_rs.{RESET}");
    }
    Ok(())
}
