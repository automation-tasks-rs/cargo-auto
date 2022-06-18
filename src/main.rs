// region: auto_md_to_doc_comments include README.md A //!
//! # cargo-auto  
//!
//! **cargo-auto - automation tasks written in Rust language for the build process of Rust projects**  
//! ***version: 2022.618.1547 date: 2022-06-18 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/cargo-auto)***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1946-green.svg)](https://github.com/bestia-dev/cargo-auto/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-532-blue.svg)](https://github.com/bestia-dev/cargo-auto/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-118-purple.svg)](https://github.com/bestia-dev/cargo-auto/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/cargo-auto/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-2177-orange.svg)](https://github.com/bestia-dev/cargo-auto/)
//!
//! [![crates.io](https://img.shields.io/crates/v/cargo-auto.svg)](https://crates.io/crates/cargo-auto)
//! [![Documentation](https://docs.rs/cargo-auto/badge.svg)](https://docs.rs/cargo-auto/)
//! [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/cargo-auto.svg)](https://web.crev.dev/rust-reviews/crate/cargo-auto/)
//! [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/cargo-auto/)
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo-auto/blob/master/LICENSE)
//! [![Rust](https://github.com/bestia-dev/cargo-auto/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/cargo-auto/)
//! [![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fbestia-dev%2Fcargo-auto&count_bg=%2379C83D&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=false)](https://hits.seeyoufarm.com)
//!
//! Hashtags: #rustlang #buildtool #developmenttool #cli
//!
//! ## Try it
//!
//! First, we will use `cargo-auto` to create a new empty CLI Rust project similar to `cargo new`, but with a more complete project structure.  
//!
//!  ```bash
//! cargo install cargo-auto
//! cargo auto new_cli my_hello_project
//! cd my_hello_project
//! cargo auto
//! # it lists all the prepared automation tasks
//! # try a few
//! cargo auto build
//! cargo auto release
//! cargo auto doc
//! cargo auto test
//! ```
//!
//! We can also add `automation tasks` to an existing Rust project.
//! Inside your Rust project directory (the one with Cargo.toml) run:  
//!
//! ```bash
//! cargo auto new_auto
//! cargo auto
//! # it lists all the prepared automation tasks
//! # try to build
//! cargo auto build
//! ```
//!
//! Congratulations! You are already using `cargo-auto`. Simple as that.  
//! Now you can modify the tasks to your needs. It is all Rust language.  
//!
//! ## Motivation
//!
//! Cargo is a great tool for building Rust projects. It has all the basics: `cargo build`, `cargo build --release`, `cargo fmt`, `cargo test`, `cargo doc`,...  
//! But sometimes we need to do more things like copying some files, publish to ftp or enter long commands. These repetitive tasks must be automated.  
//! Task automation makes work easier and faster, simplifies the workflow, while improving the consistency and accuracy of workflows.  
//! This is also sometimes referred to as "workflow automation."  
//! There are many different build systems and task runners there: `make`, `cmake`, `shell scripts`, `cargo-xtask`, `cargo-make`, `cargo-task`, `cargo-script`, `cargo-run-script`, `runner`, `python scripts`, `powershell scripts`, `cmd prompt scripts`, ...  
//! Sadly there is no standard in the Rust community for now.  
//! I want something similar to [build.rs](https://doc.rust-lang.org/cargo/reference/build-scripts.html), so I can write my "tasks" in pure Rust I don't want to learn another meta language with weird syntax and difficult to debug. So I will make something really simple, easy, rusty and extensible.  
//!
//! ## cargo auto new_cli
//!
//! I like very much that Rust has the command `cargo new project_name`. It creates a super simple Rust hello project that can be build and run immediately. But this example is too simple. It lacks basic file structures of a serious CLI program.  
//! I composed an opinionated template for a Rust CLI project. It is easy to run:
//!
//! ```bash
//! cargo auto new_cli project_name
//! ```
//!
//! ## scripting with rust
//!
//! Rust is a compiled language. It is not really a scripting or interpreted language. But the compilation of small projects is really fast and can be ignored. Subsequent calls will use the already built binary and so the speed will be even faster.  
//! This tool `cargo-auto` is meant for Rust projects, so it means that all the Rust infrastructure is already in place.  
//!
//! ## automation_tasks_rs helper project
//!
//! The command `cargo auto new_auto` will create a new directory `automation_tasks_rs` with a template for a helper Rust project in the root directory of your `main Rust project` . It should not interfere with the main Rust project. This directory will be added into git commits and pushed to remote repositories as part of the main project. It has its own `.gitignore` to avoid committing its target directory.  
//! The `automation_tasks_rs` helper project contains user defined tasks in Rust code. Your tasks. This helper project should be opened in a new editor starting from the `automation_tasks_rs` directory. It does not share dependencies with the main project. It is completely separate and independent.  
//! You can edit it and add your dependencies and Rust code. No limits. Freedom of expression.  
//! This is now your code, your tasks and your helper Rust project!  
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
//!             if &task == "build"{
//!                 task_build();
//!             } else if &task == "release" {
//!                 task_release();
//!             } else if &task == "doc" {
//!                 task_doc();
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
//! /// cargo build
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
//! /// cargo build --release
//! fn task_release() {
//!     println!("$ cargo fmt");
//!     run_shell_command("cargo fmt");
//!     println!("$ cargo build --release");
//!     run_shell_command("cargo build --release");
//! }
//!
//! /// cargo doc, then copies to /docs/ folder, because this is a github standard folder
//! fn task_doc() {
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
//! The source code is on [GitHub](https://github.com/bestia-dev/cargo-auto) with MIT open-source licensing.  
//!
//! ## bash auto-completion
//!
//! With the help of the crate [dev_bestia_cargo_completion](https://crates.io/crates/dev_bestia_cargo_completion) the commands `cargo` and `cargo auto` get bash auto-completion. Try it!  
//!
//! ## cargo auto new_auto
//!
//! Inside the cargo-auto project there is a Rust sub-projects that is a template. I can open a new editor for this directories and build this crate independently. So it is easy to debug and develop.  
//! Sadly, I cannot publish these directories and files to `crates.io`. I can effectively publish only the source code inside my main Rust project `cargo-auto`.  
//! Therefor, before publishing I copy the content of these files into the modules `template_new_auto_mod.rs` on every build. It is not difficult now that Rust has fantastic [raw strings](https://doc.rust-lang.org/rust-by-example/std/str.html).  
//!
//! ## more complex tasks
//!
//! You can write more complex tasks in Rust language.  
//! For example in this project I use automation to create github Releases : <https://github.com/bestia-dev/dropbox_backup_to_external_disk>  
//! Here is pretty complex workspace with more sub-projects:  
//! <https://github.com/bestia-dev/cargo_crev_reviews_workspace>  
//! There is no end to your imagination. If you write something that looks it can help other developers, please share it with me and I will add it here.
//!
//! ## development
//!
//! Usually I compile and run the code of `cargo-auto` with added arguments like this:  
//!
//! ```bash
//! cargo run -- new_auto
//! cargo run -- build
//! cargo run -- release
//! ```
//!
//! ## TODO
//!
//! Get the username from git, to use it in Cargo.toml of new_cli
//!
//! ## cargo crev reviews and advisory
//!
//! We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).  
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
//! to verify the trustworthiness of each of your dependencies.  
//! Please, spread this info.  
//! You can also read reviews quickly on the web:  
//! <https://web.crev.dev/rust-reviews/crates/>  
//!
//! ## open-source free and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful,  
//! please buy me a beer donating on my [paypal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
// endregion: auto_md_to_doc_comments include README.md A //!

mod inside_of_rust_project_mod;
mod outside_of_rust_project_mod;
mod template_new_auto_mod;
mod template_new_cli_mod;

// region: use statements
use lazy_static::lazy_static;
use std::path::{Path, PathBuf};
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

pub struct FileItem {
    file_name: &'static str,
    file_content: &'static str,
}

fn main() {
    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();

    if is_not_run_in_rust_project_root_directory() {
        outside_of_rust_project_mod::parse_args(&mut args);
    } else {
        inside_of_rust_project_mod::parse_args(&mut args);
    }
}

/// check if is not run in Rust project root directory
fn is_not_run_in_rust_project_root_directory() -> bool {
    // return negation of exists
    !Path::new("Cargo.toml").exists()
}
