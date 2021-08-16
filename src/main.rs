// region: auto_md_to_doc_comments include README.md A //!
//! # cargo-auto  
//!
//! **cargo-auto : automation tasks written in Rust language for the build process of rust projects**  
//! ***[repository](https://github.com/LucianoBestia/cargo-auto); version: 2021.816.1045  date: 2021-08-16 authors: Luciano Bestia***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-424-green.svg)](https://github.com/LucianoBestia/cargo-auto/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-314-blue.svg)](https://github.com/LucianoBestia/cargo-auto/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-47-purple.svg)](https://github.com/LucianoBestia/cargo-auto/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/cargo-auto/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/cargo-auto/)
//!
//! [![crates.io](https://img.shields.io/crates/v/cargo-auto.svg)](https://crates.io/crates/cargo-auto) [![Documentation](https://docs.rs/cargo-auto/badge.svg)](https://docs.rs/cargo-auto/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/cargo-auto.svg)](https://web.crev.dev/rust-reviews/crate/cargo-auto/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/cargo-auto/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/cargo-auto/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/cargo-auto/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/cargo-auto/)  
//!
//! ## Try it
//!
//! Inside your main rust project directory (the one with Cargo.toml) run:  
//!
//! ```bash
//! cargo install cargo-auto
//! cargo auto new
//! ```
//!
//! ![cargo_auto_new](https://github.com/LucianoBestia/cargo-auto/raw/main/images/cargo_auto_new.png "cargo_auto_new")
//!
//! Now try to run a task:
//!
//! ```bash
//! cargo auto build
//! ```
//!
//! ![cargo_auto_build](https://github.com/LucianoBestia/cargo-auto/raw/main/images/cargo_auto_build.png "cargo_auto_build")
//!
//! Congratulations! You are already using `cargo-auto`. Simple as that.  
//!
//! ## Motivation
//!
//! Cargo is a great tool for building rust projects. It has all the basics: `cargo build`, `cargo build --release`, `cargo fmt`, `cargo test`, `cargo doc`,...  
//! But sometimes we need to do more things like copying some files, publish to ftp or enter long commands. These repetitive tasks must be automated.  
//! Task automation makes work easier and faster, simplifies the workflow, while improving the consistency and accuracy of workflows.  
//! This is also sometimes referred to as "workflow automation."  
//! There are many different build systems and task runners there: `make`, `cmake`, `shell scripts`, `cargo-xtask`, `cargo-make`, `cargo-task`, `cargo-script`, `cargo-run-script`, `runner`, `python scripts`, `powershell scripts`, `cmd prompt scripts`, ...  
//! Sadly there is no standard in the rust community for now.  
//! I want something similar to [build.rs](https://doc.rust-lang.org/cargo/reference/build-scripts.html), so I can write my "tasks" in pure rust. I don't want to learn another meta language with weird syntax and difficult to debug. So I will make something really simple, easy, rusty and extensible.  
//!
//! ## scripting with rust
//!
//! Rust is a compiled language. It is not really a scripting or interpreted language. But the compilation of small projects is really fast and can be ignored. Subsequent calls will use the already built binary and so the speed will be even faster.  
//! This tool `cargo-auto` is meant for rust projects, so it means that all the rust infrastructure is already in place.  
//!
//! ## automation_tasks_rs helper project
//!
//! The command `cargo auto new` will create a new directory `automation_tasks_rs` with a template for a helper rust project in the root directory of your `main rust project` . It should not interfere with the main rust project. This directory will be added into git commits and pushed to remote repositories as part of the main project. It has its own `.gitignore` to avoid committing its target directory.  
//! The `automation_tasks_rs` helper project contains user defined tasks in rust code. Your tasks. This helper project should be opened in a new editor starting from the `automation_tasks_rs` directory. It does not share dependencies with the main project. It is completely separate and independent.  
//! You can edit it and add your dependencies and rust code. No limits. Freedom of expression.  
//! This is now your code, your tasks and your helper rust project!  
//! Because only you know what you want to automate and how to do it.  
//! Basic example:  
//!
//! ```rust
//! /// match arguments and call tasks functions
//! fn match_arguments_and_call_tasks(mut args: std::env::Args){
//!     // the first argument is the user defined task: (no argument for help), build, release,...
//!     let arg_1 = args.next();
//!     match arg_1 {
//!         None => print_help(),
//!         Some(task) => {            
//!             println!("Running auto task: {}", &task);
//!             if &task == "build" || &task == "b" {
//!                 task_build();
//!             } else if &task == "release" || &task == "r" {
//!                 task_release();
//!             } else if &task == "docs" || &task == "doc" || &task == "d" {
//!                 task_docs();
//!             } else {
//!                 println!("Task {} is unknown.", &task);
//!                 print_help();
//!             }
//!         }
//!     }
//! }
//!
//! /// write a comprehensible help for user defined tasks
//! fn print_help() {
//!     println!("User defined tasks in automation_tasks_rs:");
//!     println!("cargo auto build - builds the crate in debug mode");
//!     println!("cargo auto release - builds the crate in release mode");
//!     println!("cargo auto docs - builds the docs");
//! }
//!
//! // region: tasks
//!
//! /// example how to call a list of shell commands
//! fn task_build() {
//!     #[rustfmt::skip]
//!     let shell_commands = [
//!         "echo $ cargo fmt",
//!         "cargo fmt",
//!         "echo $ cargo build",
//!         "cargo build"];
//!     run_shell_commands(shell_commands.to_vec());
//! }
//!
//! /// example how to call one shell command and combine with rust code
//! fn task_release() {
//!     println!("$ cargo fmt");
//!     run_shell_command("cargo fmt");
//!     println!("$ cargo build --release");
//!     run_shell_command("cargo build --release");
//! }
//!
//! /// example how to call a list of shell commands and combine with rust code
//! fn task_docs() {
//!     #[rustfmt::skip]
//!     let shell_commands = [
//!         "echo $ cargo doc --no-deps --document-private-items --open",
//!         "cargo doc --no-deps --document-private-items --open",        
//!         // copy to /docs/ because it is github standard
//!         "echo $ rsync -a --info=progress2 --delete-after target/doc/ docs/",
//!         "rsync -a --info=progress2 --delete-after target/doc/ docs/",
//!         "echo Create simple index.html file in docs directory",
//!         &format!("echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",&project_directory_name()) ,
//!         // message to help user with next move
//!         "echo After successful doc, commit and push changes",
//!         ];
//!     run_shell_commands(shell_commands.to_vec());
//! }
//!
//! // endregion: tasks
//!
//! ```
//!
//! ## cargo auto subcommand
//!
//! The command `cargo install cargo-auto` will add a new subcommand to cargo:
//!
//! ```bash
//! cargo auto
//! ```
//!
//! This binary is super simple. It has only 3 trivial dependencies: `unwrap`, `termion` and `lazy_static`.  
//! The binary only reads the CLI arguments and runs the `automation_tasks_rs` binary with them. If needed it will compile `automation_tasks_rs` first.  
//! The code-flow of the source code of `cargo-auto` is simple, fully commented and straightforward to audit.  
//! The source code is on [GitHub](https://github.com/LucianoBestia/cargo-auto) with MIT open-source licensing.  
//!
//! ## templates
//!
//! Inside the cargo-auto project there are 2 directories with rust sub-projects as templates. I can open a new editor for these directories and build this crates independently. So it is easy to debug and develop.  
//! Sadly, I cannot publish these directories and files to `crates.io`. I can effectively publish only the source code inside my main rust project `cargo-auto`.  
//! Therefor, before publishing I must copy the text of these files into the modules `template_basic_mod.rs` and `template_with_lib_mod.rs`. It is not difficult now that rust has fantastic [raw strings](https://doc.rust-lang.org/rust-by-example/std/str.html).  
//!
//! ## template_basic
//!
//! The command  
//!
//! ```rust
//! cargo auto new
//! ```
//!
//! will copy the `template_basic` into `automation_tasks_rs` directory.  
//! This has no dependencies at all, except `std`. It is really simple to understand how it works.  
//! Open the directory `automation_tasks_rs` in a new editor, explore and add your own tasks in rust code. It is a plain CLI rust project, you can do everything you need with it. Add dependencies and stuff. No limits. This helper project will be added to you commits and stay part of your project.  
//! Then in the main project, run your task (the task name here is `build`) simply like this:  
//!
//! ```rust
//! cargo auto build
//! ```
//!
//! Your code will be compiled (if needed) and executed.  
//!
//! ## template_with_lib
//!
//! This is a work in progress.  
//! The goal is to create a library crate [cargo_auto_lib](https://github.com/LucianoBestia/cargo_auto_lib) with many functions that are commonly used when building rust projects.  
//!
//! ```rust
//! cargo auto new with_lib
//! ```
//!
//! ## development
//!
//! Usually I compile and run the code of `cargo-auto` with added arguments like this:  
//!
//! ```bash
//! cargo run -- new
//! cargo run -- build
//! cargo run -- release
//! ```
//!
//! After my first publish to crates.io I discovered that the position of the argument changes if it is used as:  
//! standalone `cargo-auto new`      - new is 1st arg  
//! or  
//! subcommand `cargo auto new`      - new is 2nd arg  
//! I added some code to solve that.  
//!
//! ## cargo crev reviews and advisory
//!
//! We leave in times of danger with `supply chain attacks`.  
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
//! to verify the trustworthiness of each of your dependencies.  
//! Please, spread this info.  
//! You can also read reviews quickly on the web. Example for the crate `num-traits`:  
//! <https://web.crev.dev/rust-reviews/crate/num-traits/>  
//!
//! ## open-source free and free as a beer
//!
//! My open-source projects are free and free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer or two donating on my [paypal](https://www.paypal.com/paypalme/LucianoBestia). You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) !
//!
// endregion: auto_md_to_doc_comments include README.md A //!

mod template_basic_mod;
mod template_with_lib_mod;

// region: use statements
use lazy_static::lazy_static;
use std::path::{Path, PathBuf};
use unwrap::unwrap;
// endregion

// colors for terminal
lazy_static! {
    /// ansi code for color
    static ref GREEN: String = termion::color::Fg(termion::color::Green).to_string();
    /// ansi code for color
    static ref YELLOW: String = termion::color::Fg(termion::color::Yellow).to_string();
    /// ansi code for color
    static ref RED: String = termion::color::Fg(termion::color::Red).to_string();
    /// ansi code for reset color
    static ref RESET: String = termion::color::Fg(termion::color::Reset).to_string();
}
// paths
lazy_static! {
    /// constant paths for read/write
    static ref PATH_AUTOMATION_TASKS_RS: PathBuf = PathBuf::from("automation_tasks_rs");
    /// constant paths for read/write
    static ref PATH_CARGO_TOML: PathBuf = PathBuf::from("automation_tasks_rs/Cargo.toml");
    /// constant paths for read/write
    static ref PATH_GITIGNORE: PathBuf = PathBuf::from("automation_tasks_rs/.gitignore");
    /// constant paths for read/write
    static ref PATH_SRC_MAIN_RS: PathBuf = PathBuf::from("automation_tasks_rs/src/main.rs");
    /// constant paths for read/write
    static ref PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS: PathBuf =
        PathBuf::from("automation_tasks_rs/target/debug/automation_tasks_rs");
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
    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    // the first argument is the task: (no argument for help), new, build, release,...
    // wooow! There is a difference if I call the standalone binary or as a cargo subcommand:
    // cargo-auto new     - new is the arg_1
    // cargo auto new     - new is the arg_2
    let arg_1 = args.next();
    match arg_1 {
        None => print_help_from_cargo_auto(),
        Some(task) => {
            if task != "auto" {
                match_first_argument(&task, args);
            } else {
                let arg_2 = args.next();
                match arg_2 {
                    None => print_help_from_cargo_auto(),
                    Some(task) => match_first_argument(&task, args),
                }
            }
        }
    }
}

/// check if is not run in rust project root directory
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
fn print_help_from_cargo_auto() {
    if !PATH_CARGO_TOML.exists() || !PATH_SRC_MAIN_RS.exists() {
        println!("");
        println!("To start using `cargo auto` you must create a new `automation_tasks_rs` directory with the command:");
        println!("$ cargo auto new");
        println!("or more advanced:");
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
                "{}Error: Directory automation_tasks_rs already exists. Cannot create new directory automation_tasks_rs.{}",
                *RED, *RESET
            );
            // early exit
            std::process::exit(0);
        }
        auto_new(&mut args);
    } else {
        if !already_exists_automation_tasks_rs() {
            println!("{}Error: Directory automation_tasks_rs does not exist.{}", *RED, *RESET);
            print_help_from_cargo_auto();
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
        build_project_automation_tasks_rs();
        // early return
        return ();
    }
    let modified_automation_tasks_rs =
        unwrap!(unwrap!(std::fs::metadata(PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS.as_os_str())).modified());
    let modified_cargo_toml = unwrap!(unwrap!(std::fs::metadata(PATH_CARGO_TOML.as_os_str())).modified());
    let modified_main_rs = unwrap!(unwrap!(std::fs::metadata(PATH_SRC_MAIN_RS.as_os_str())).modified());

    if modified_automation_tasks_rs < modified_cargo_toml || modified_automation_tasks_rs < modified_main_rs {
        build_project_automation_tasks_rs();
    }
}

/// build automation_tasks_rs
fn build_project_automation_tasks_rs() {
    // build in other directory (not in working current directory)
    // cargo build --manifest-path=dir/Cargo.toml
    unwrap!(unwrap!(std::process::Command::new("cargo")
        .arg("build")
        .arg("--manifest-path=automation_tasks_rs/Cargo.toml")
        .spawn())
    .wait());
}

/// already exists automation_tasks_rs directory
fn already_exists_automation_tasks_rs() -> bool {
    // return
    PATH_AUTOMATION_TASKS_RS.exists()
}

/// copies the template to the `automation_tasks_rs` directory  
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
    build_automation_tasks_rs_if_needed();
    println!("");
    println!("`crate auto new` generated the directory `automation_tasks_rs` in your main rust project.");
    println!("You can open this new helper rust project in a new rust editor.");
    println!("View and edit the rust code in `automation_tasks_rs`. It is independent from the main project.");
    println!("It will be automatically compiled on the next use of `crate auto task_name` command.");
    println!("The new directory will be added to your git commit.");
    println!("There is a local .gitignore file to avoid commit of the `target/` directory.");
    // call `cargo auto` to show the help of the new automation_tasks_rs
    unwrap!(unwrap!(std::process::Command::new("cargo").arg("auto").spawn()).wait());
}

/// creates directory if needed and copy files from templates: Cargo.toml, .gitignore and main.rs  
/// The template text is in the modules template_basic_mod.rs and template_with_lib_mod.rs
fn copy_template(template_name: &str) {
    unwrap!(std::fs::create_dir_all(Path::new("automation_tasks_rs/src")));

    if template_name == "basic" {
        unwrap!(std::fs::write(
            PATH_CARGO_TOML.as_os_str(),
            crate::template_basic_mod::cargo_toml().as_bytes()
        ));
        unwrap!(std::fs::write(
            PATH_GITIGNORE.as_os_str(),
            crate::template_basic_mod::gitignore().as_bytes()
        ));
        unwrap!(std::fs::write(
            PATH_SRC_MAIN_RS.as_os_str(),
            crate::template_basic_mod::src_main_rs().as_bytes()
        ));
    } else if template_name == "with_lib" {
        unwrap!(std::fs::write(
            PATH_CARGO_TOML.as_os_str(),
            crate::template_with_lib_mod::cargo_toml().as_bytes()
        ));
        unwrap!(std::fs::write(
            PATH_GITIGNORE.as_os_str(),
            crate::template_with_lib_mod::gitignore().as_bytes()
        ));
        unwrap!(std::fs::write(
            PATH_SRC_MAIN_RS.as_os_str(),
            crate::template_with_lib_mod::src_main_rs().as_bytes()
        ));
    }
}
