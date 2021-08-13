// region: lmake_readme include "readme.md" //! A

// endregion: lmake_readme include "readme.md" //! A

use std::{path::{Path, PathBuf}};

mod template_basic;
mod template_with_lib;

// region: use statements
use lazy_static::lazy_static;
use unwrap::unwrap;
// endregion

// colors for terminal
lazy_static! {
    static ref GREEN: String = termion::color::Fg(termion::color::Green).to_string();
    static ref YELLOW: String = termion::color::Fg(termion::color::Yellow).to_string();
    static ref RED: String = termion::color::Fg(termion::color::Red).to_string();
    static ref RESET: String = termion::color::Fg(termion::color::Reset).to_string();
}
// paths
lazy_static! {
    static ref PATH_AUTOMATION_TASKS_RS: PathBuf = PathBuf::from("automation_tasks_rs");
    static ref PATH_CARGO_TOML: PathBuf = PathBuf::from("automation_tasks_rs/Cargo.toml");
    static ref PATH_GITIGNORE: PathBuf = PathBuf::from("automation_tasks_rs/.gitignore");
    static ref PATH_SRC_MAIN_RS: PathBuf = PathBuf::from("automation_tasks_rs/src/main.rs");
    static ref PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS: PathBuf = PathBuf::from("automation_tasks_rs/target/debug/automation_tasks_rs");
}

fn main() {
    if is_not_run_in_rust_project_root_directory() {
        println!(
            "{}Error: cargo-auto must be called in the root directory of the rust project beside the Cargo.toml file.{}",
            *RED, *RESET
        );
        // early exit
        std::process::exit(0);
    }
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    // the first argument is the task: (no argument for help), new, build, release,...
    // wooow! There is a difference if I call the standalone binary or as a cargo subcommand:
    // cargo-auto new     - new is the arg_1
    // cargo auto new     - new is the arg_2
    let arg_1 = args.next();
    match arg_1 {
        None => print_help_when_no_argument(),
        Some(task) => {
            if task!="auto"{
                match_first_argument(&task, args);
            }else{                
                let arg_2 = args.next();
                match arg_2 {
                    None => print_help_when_no_argument(),
                    Some(task) => match_first_argument(&task, args),                    
                }
            }
        }
    }
}

/// check if run in rust project root directory error and exit if not
fn is_not_run_in_rust_project_root_directory() -> bool {
    // return negation of exists
    !Path::new("Cargo.toml").exists()
}

/// if there is no argument then print help
/// if there exists `automation_tasks_rs/Cargo.toml` and `automation_tasks_rs/src/main.rs`
/// call automation_tasks_rs with no arguments to print the help prepared in user defined automation_tasks_rs
/// else print the help for `cargo auto new`
/// in development use: `cargo run`
/// in runtime use: `cargo auto`
fn print_help_when_no_argument() {
    if !PATH_CARGO_TOML.exists() || !PATH_SRC_MAIN_RS.exists() {
        println!("To start using `cargo auto` you must create a new automation_tasks_rs folder with the command:");
        println!("$ cargo auto new");
        println!("or more advanced");
        println!("$ cargo auto new with_lib");
    } else {
        build_automation_tasks_rs_if_needed();
        unwrap!(unwrap!(std::process::Command::new(PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS.as_os_str()).spawn()).wait());
    }
}

/// the first argument is the task: new, build, release,...
/// the task `new` is processed by `cargo-auto`,
/// all other tasks are processed by the used defined `automation_tasks_rs`  
/// in development use: `cargo run -- new`
/// in development use: `cargo run -- build`
/// in development use: `cargo run -- release`
fn match_first_argument(task: &str, mut args: std::env::Args) {
    if task == "new" {
        if already_exists_automation_tasks_rs() {
            println!(
                "{}Error: Directory automation_tasks_rs already exists. Cannot create new automation_tasks_rs.{}",
                *RED, *RESET
            );
            // early exit
            std::process::exit(0);
        }
        auto_new(&mut args);
    } else {
        if !already_exists_automation_tasks_rs() {
            println!("{}Error: Directory automation_tasks_rs does not exist.{}", *RED, *RESET);
            print_help_when_no_argument();
            // early exit
            std::process::exit(0);
        }
        build_automation_tasks_rs_if_needed();
        // call automation_tasks_rs/target/debug/automation_tasks_rs with all the arguments
        let mut command = std::process::Command::new(PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS.as_os_str());
        command.arg(&task);
        while let Some(arg_x) = args.next() {
            command.arg(&arg_x);
        }
        let mut child = unwrap!(command.spawn());
        unwrap!(child.wait());
    }
}

/// build if the date of Cargo.toml or main.rs is newer then of automation_tasks_rs/target/automation_tasks_rs
fn build_automation_tasks_rs_if_needed() {
    if !PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS.exists() {
        build_automation_tasks_rs();
        // early return
        return ();
    }
    let modified_automation_tasks_rs =
        unwrap!(unwrap!(std::fs::metadata(PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS.as_os_str())).modified());
    let modified_cargo_toml = unwrap!(unwrap!(std::fs::metadata(PATH_CARGO_TOML.as_os_str())).modified());
    let modified_main_rs = unwrap!(unwrap!(std::fs::metadata(PATH_SRC_MAIN_RS.as_os_str())).modified());

    if modified_automation_tasks_rs < modified_cargo_toml || modified_automation_tasks_rs < modified_main_rs {
        build_automation_tasks_rs();
    }
}

/// build automation_tasks_rs
fn build_automation_tasks_rs() {
    // build in other directory (not in working current directory)
    // cargo build --manifest-path=dir/Cargo.toml
    unwrap!(unwrap!(std::process::Command::new("cargo")
        .arg("build")
        .arg("--manifest-path=automation_tasks_rs/Cargo.toml")
        .spawn())
    .wait());
}

/// already exists automation_tasks_rs
fn already_exists_automation_tasks_rs() -> bool {
    // return
    PATH_AUTOMATION_TASKS_RS.exists()
}

/// copies the template to the automation_tasks_rs directory
/// the second argument is the template name
/// without template_name copies the template_basic
/// in development use: `cargo run -- new`
/// in runtime use: `cargo auto new`
/// with the argument `with_lib` copies template_with_lib
/// in development use: `cargo run -- new with_lib`
/// in runtime use: `cargo auto new with_lib`
fn auto_new(args: &mut std::env::Args) {
    let arg_2 = args.next();
    match arg_2 {
        None => copy_template("basic"),
        Some(template_name) => copy_template(&template_name),
    }
}

/// creates directory if needed and copy files from templates: Cargo.toml, .gitignore and main.rs
/// I have to copy this files into the modules crate::template_basic and crate::template_with_lib
/// because when publishing to crates.io I loose all other files except the main binary.
fn copy_template(template_name: &str) {    
    unwrap!(std::fs::create_dir_all(Path::new("automation_tasks_rs/src")));

    if template_name=="basic"{
        unwrap!(std::fs::write(PATH_CARGO_TOML.as_os_str(), crate::template_basic::cargo_toml().as_bytes()));
        unwrap!(std::fs::write(PATH_GITIGNORE.as_os_str(), crate::template_basic::gitignore().as_bytes()));
        unwrap!(std::fs::write(PATH_SRC_MAIN_RS.as_os_str(), crate::template_basic::src_main_rs().as_bytes()));
    } else 
    if template_name=="basic"{
        unwrap!(std::fs::write(PATH_CARGO_TOML.as_os_str(), crate::template_with_lib::cargo_toml().as_bytes()));
        unwrap!(std::fs::write(PATH_GITIGNORE.as_os_str(), crate::template_with_lib::gitignore().as_bytes()));
        unwrap!(std::fs::write(PATH_SRC_MAIN_RS.as_os_str(), crate::template_with_lib::src_main_rs().as_bytes()));
    }
}
