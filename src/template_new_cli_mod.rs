//! this strings are copied from the template_new_cli folder
//! because when publishing to crates.io, only the main bin-executable is transferred

pub fn copy_to_files(project_name: &str) {
    let folder_path = std::path::Path::new(project_name);
    std::fs::create_dir_all(folder_path).unwrap();
    for file_item in get_vec_file() {
        // rename/replace the project_name
        let file_name = file_item
            .file_name
            .replace("bestia_dev_cargo_auto_new_cli", project_name);
        let file_content = file_item
            .file_content
            .replace("bestia_dev_cargo_auto_new_cli", project_name);

        // create directory if needed
        std::fs::create_dir_all(folder_path.join(&file_name).parent().unwrap()).unwrap();
        std::fs::write(folder_path.join(&file_name), file_content.as_bytes()).unwrap();
    }
}

pub fn get_vec_file() -> Vec<crate::FileItem> {
    let mut vec_file = vec![];

    // region: files copied into strings by automation tasks
    vec_file.push(crate::FileItem {
        file_name: ".vscode/settings.json",
        file_content: r###"{
    "cSpell.words": [
        "Alla",
        "bestia",
        "crev",
        "debuginfo",
        "deps",
        "endregion",
        "Nazdravlje",
        "plantuml",
        "Prost",
        "rustdevuser",
        "rustprojects",
        "struct",
        "termion",
        "thiserror",
        "unoptimized",
        "zdravje"
    ]
}"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "Cargo.toml",
        file_content: r###"[package]
name = "bestia_dev_cargo_auto_new_cli"
version = "1.0.12"
description = "Basic Rust project template for CLI and library, more than just `cargo new hello`"
authors = ["bestia.dev"]
homepage = "https://bestia.dev"
edition = "2021"
license = "MIT"
readme = "README.md"
repository = "https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli"
categories = ["rust-patterns"]
keywords = ["Rust cli and library project template"]
publish = false

[dependencies]
log = "0.4"
pretty_env_logger="0.4.0"
thiserror = "1.0.30"
anyhow="1.0.56""###,
    });
    vec_file.push(crate::FileItem {
        file_name: ".gitignore",
        file_content: r###"/target
**/*.rs.bk
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/hello_mod.rs",
        file_content: r###"// bestia_dev_cargo_auto_new_cli/src/hello_mod.rs

//! All the real code is inside modules in separate files.
//!
//! This doc-comments will be compiled into the `docs`.

/// format the hello phrase
pub fn format_hello_phrase(greet_name: &str) -> String {
    log::info!("start format_hello_phrase()");
    // return
    format!("Hello {}!", greet_name)
}

/// format the hello phrase with uppercase name
/// if it is already uppercase, return error with thiserror
pub fn format_upper_hello_phrase(greet_name: &str) -> Result<String, crate::LibraryError> {
    log::info!("start format_upper_hello_phrase()");
    // shadowing the same variable name:
    let upper_greet_name = make_uppercase(greet_name);
    if upper_greet_name == greet_name {
        return Err(crate::LibraryError::Uppercase(greet_name.to_string()));
    }

    // return
    Ok(format!("Hello {}!", &upper_greet_name))
}

/// return uppercase
pub fn make_uppercase(greet_name: &str) -> String {
    // return
    greet_name.to_uppercase()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_format_upper_hello_phrase() {
        assert_eq!(format_upper_hello_phrase("abcd").expect("error"), "Hello ABCD!");
        assert!(format_upper_hello_phrase("ABCD").is_err());
    }

    #[test]
    pub fn test_make_uppercase() {
        assert_eq!(make_uppercase("abcd"), "ABCD");
        assert_eq!(make_uppercase("1234abcd"), "1234ABCD");
        assert_eq!(make_uppercase("čšž"), "ČŠŽ");
    }
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/bin/bestia_dev_cargo_auto_new_cli/main.rs",
        file_content: r###"//! bestia_dev_cargo_auto_new_cli/src/bin/bestia_dev_cargo_auto_new_cli/main.rs

// This `main.rs` is the code for the CLI application.
// The build of this project will create the CLI application.
// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`
// This `lib.rs` can be used as dependency crate for other projects.

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.

// Linux terminal colors
use bestia_dev_cargo_auto_new_cli::{RED, RESET, YELLOW};

/// entry point into the bin-executable
fn main() {
    // logging is essential for every project
    pretty_env_logger::init();

    // super simple argument parsing. There are crates that can parse more complex arguments.
    match std::env::args().nth(1).as_deref() {
        None | Some("--help") | Some("-h") => print_help(),
        Some("print") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(greet_name) => {
                print_greet_name(greet_name);
            }
            None => println!("{RED}Error: Missing arguments `greet_name`.{RESET}"),
        },
        Some("upper") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(greet_name) => {
                // this can return an error. Here is the last place I can deal with the error.
                match upper_greet_name(greet_name) {
                    // do nothing
                    Ok(()) => (),
                    // log error from anyhow
                    Err(err) => println!("{RED}Error: {err}{RESET}"),
                }
            }
            None => println!("{RED}Error: Missing arguments `greet_name`.{RESET}"),
        },
        _ => println!("{RED}Error: Unrecognized arguments. Try `bestia_dev_cargo_auto_new_cli --help`{RESET}"),
    }
}

/// print help
fn print_help() {
    println!(
        r#"
    {YELLOW}Welcome to bestia_dev_cargo_auto_new_cli !
    This is a simple yet complete template for a CLI program written in Rust.{RESET}

bestia_dev_cargo_auto_new_cli --help
bestia_dev_cargo_auto_new_cli print world
bestia_dev_cargo_auto_new_cli upper world

    This command should return an error:
bestia_dev_cargo_auto_new_cli upper WORLD
  
"#
    );
}

/// print my name
fn print_greet_name(greet_name: &str) {
    // call the function from the `lib.rs`
    println!("{}", bestia_dev_cargo_auto_new_cli::format_hello_phrase(greet_name));
}

/// print my name upper, can return error
fn upper_greet_name(greet_name: &str) -> anyhow::Result<()> {
    // the function from `lib.rs`, can return error
    // use the ? syntax to bubble the error up one level or continue (early return)
    let upper = bestia_dev_cargo_auto_new_cli::format_upper_hello_phrase(greet_name)?;
    println!("{}", upper);
    // return
    Ok(())
}
"###,
    });
    vec_file.push(crate::FileItem{
            file_name :"src/lib.rs",
            file_content : r###"// bestia_dev_cargo_auto_new_cli/src/lib.rs

// You can collapse the long region below using VSCode. It is only the copy of the README.md file, because it gets compiled into docs.

// region: auto_md_to_doc_comments include README.md A //!
//! # cargo-auto  
//!
//! **cargo-auto - automation tasks written in Rust language for the build process of Rust projects**  
//! ***version: 2022.618.1552 date: 2022-06-18 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/cargo-auto)***  
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

// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`

// The `lib.rs` does not have any real code. All the code is in modules in separate files.
// The `lib.rs` has just the list of modules, it publishes module's functions or class for the caller
// and it has some global stuff like the Error enum.

// access to modules
mod hello_mod;

// `pub use` allows the caller of the lib to access modules functions, structs or all(*)
pub use hello_mod::format_hello_phrase;
pub use hello_mod::format_upper_hello_phrase;

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.
use thiserror::Error;

/// all possible library errors for `thiserror`
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("Name `{0}` is already uppercase.")]
    Uppercase(String),
    #[error("Unknown error.")]
    Unknown,
}

// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";
"###,
});
    vec_file.push(crate::FileItem{
            file_name :"README.md",
            file_content : r###"[comment]: # (auto_md_to_doc_comments segment start A)

# bestia_dev_cargo_auto_new_cli

[comment]: # (auto_cargo_toml_to_md start)

**Basic Rust project template for CLI and library, more than just `cargo new hello`**  
***version: 1.0.4 date: 2022-04-21 author: [bestia.dev](bestia.dev) repository: [Github](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli)***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-89-green.svg)](https://github.com/bestia-dev/cargo-auto/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-13-blue.svg)](https://github.com/bestia-dev/cargo-auto/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-36-purple.svg)](https://github.com/bestia-dev/cargo-auto/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-19-yellow.svg)](https://github.com/bestia-dev/cargo-auto/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-30-orange.svg)](https://github.com/bestia-dev/cargo-auto/)

[comment]: # (auto_lines_of_code end)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/blob/main/LICENSE) [![Rust](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli/)

## Edit this README.md file

Edit this README file with your data. But leave the markers: auto_md_to_doc_comments, auto_lines_of_code, auto_cargo_toml_to_md and similar, because the automation tasks need them.  
Modify the title and description only in Cargo.toml. Automation tasks will copy that into README.md.  
Lines of code are filled automatically from automation tasks.  
Find `bestia.dev` everywhere and change it with your username.

## Motivation

My first line I typed when I learned the Rust language was `cargo new hello`. It is extraordinary for learning Rust, but it is a rudimentary example, not really useful in practical life.

I created this project template `bestia_dev_cargo_auto_new_cli` for a simple CLI application that has all the moving parts for a real life project.

## Separate main.rs and lib.rs

It is always good to split the project between a `main.rs` (executable) and a `lib.rs` (library crate).

Even for the smallest project. Maybe some other program will use the library eventually.

All the input/output is coded in the `main.rs`: keyboard and monitor (stdin and stdout), access to files and some access to network.  
The library must not operate directly with the stdin/stdout, because some other caller of the library can have other ideas around input-output options. Maybe it is a Graphical user interface that does thing completely different than CLI applications.

A separate `lib.rs` enables to make good tests and examples without worrying about input-output.

## super simple argument parsing

I use a super simple code to parse CLI arguments inside the `src/bin/bestia_dev_cargo_auto_new_cli/main.rs`. There are crate libraries that enables very complex argument parsing if needed.

## automation_tasks_rs

Building a project is always more complex then just `cargo build` and `cargo run`. There are always some files to copy or some content to copy from file to file. For this I use `cargo-auto` - automation tasks written in Rust language for the build process of Rust projects.

All the source is inside the folder `automation_tasks_rs`. It is pure Rust, it is easy to understand and modify to your needs.

To start using it just type in `VSCode terminal`:

```bash
cargo auto
```

```bash
User defined tasks in automation_tasks_rs:
cargo auto build - builds the crate in debug mode, fmt
cargo auto release - builds the crate in release mode, version from date, fmt, strip
cargo auto doc - builds the docs, copy to docs directory
cargo auto commit_and_push - commits with message and push with mandatory message
 if you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git
cargo auto publish_to_crates_io - publish to crates.io, git tag
```

The `bash auto-completion` should work. If you type `cargo auto b` and press `tab` it should auto-complete to `build`. Look at the project <https://github.com/bestia-dev/dev_bestia_cargo_completion>.

```bash
cargo auto build
```

```bash
Running automation task: build
old version: "0.1.18"
new version: '0.1.19'
$ cargo fmt
$ cargo build
Compiling bestia_dev_cargo_auto_new_cli v0.1.19 (/home/rustdevuser/rustprojects/bestia_dev_cargo_auto_new_cli)
Finished dev [unoptimized + debuginfo] target(s) in 2.72s

After `cargo auto build`, run the compiled binary
run `./target/debug/bestia_dev_cargo_auto_new_cli print world`
later
run `cargo auto release`
```

After the task there is a recommendation what to do next.

```bash
cargo auto release
```

```bash
Running automation task: release
old version: "0.1.20"
new version: '0.1.21'
new text: '
**Basic Rust project template for CLI, more than just `cargo new hello`**
***version: 0.1.21 date: 2022-04-01 author: [bestia.dev](bestia.dev) repository: [Github](https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli)***'

include_into_readme_md write file: README.md
$ cargo fmt
$ cargo build --release
Compiling bestia_dev_cargo_auto_new_cli v0.1.21 (/home/rustdevuser/rustprojects/bestia_dev_cargo_auto_new_cli)
Finished release [optimized] target(s) in 1.05s

After `cargo auto release`, , run the compiled binary
run `./target/release/bestia_dev_cargo_auto_new_cli print world`
later
run `cargo auto doc`

```

Release is incrementing the version number and date, copying the title and description from Cargo.toml to README.md. Calculates the lines of code in the project and makes badges from it in README.md. Copying the README into doc comments, so the documentation can be compiled later.

```bash
cargo auto doc
```

```bash
Running automation task: doc
$ cargo doc --no-deps --document-private-items
 Documenting bestia_dev_cargo_auto_new_cli v0.1.21 (/home/rustdevuser/rustprojects/bestia_dev_cargo_auto_new_cli)
Finished dev [unoptimized + debuginfo] target(s) in 0.54s
$ rsync -a --info=progress2 --delete-after target/doc/ docs/
2,787,371 100% 46.60MB/s 0:00:00 (xfr#56, to-chk=0/61) 

After `cargo auto doc`, check `docs/index.html`. If ok, then 
run `cargo auto commit_and_push` with mandatory commit message
```

If you Ctrl+Click on the link `docs/index.html` it will open the file in VSCode editor. In the right corner you can click to see the Live Preview. It will open the preview for the html file in an integrated browser in VSCode. Very useful.
Now is a good time to run all the test before committing.

```bash
cargo test
```

If we are happy with the changes, we commit and push:

```bash
cargo auto commit_and_push "my message for commit"
```

```bash
Running automation task: commit_and_push
$ git add -A && git commit -m "readme"
[main 3bdcc91] readme
 9 files changed, 443 insertions(+), 89 deletions(-)
$ git push
Enumerating objects: 36, done.
Counting objects: 100% (36/36), done.
Delta compression using up to 6 threads
Compressing objects: 100% (16/16), done.
Writing objects: 100% (19/19), 6.27 KiB | 1.25 MiB/s, done.
Total 19 (delta 11), reused 0 (delta 0), pack-reused 0
remote: Resolving deltas: 100% (11/11), completed with 10 local objects.
To https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli.git
 d0f31d3..3bdcc91 main -> main

After `cargo auto commit and push`
run `cargo auto publish_to_crates_io`
```

And finally if you want to publish it on crates.io. First you need the `access token` you get from crates.io.

```bash
cargo login
# type the access token
cargo auto publish_to_crates_io
```

## lib.rs doc-comments

The entire README.md is copied into lib.rs. This can be annoying to watch. You can collapse the entire section clicking on `// region: auto_md_to_doc_comments include README.md`.

You can use `// region:` and `// endregion:` to mark sections you want to collapse in the editor.

From this doc-comments the `docs` will be created. Take a look and try to write what other users would want to read in the `docs`.

## Modules

I added one module `hello_mod.rs` just to showcase how modules are used in separate files.

## Markdown

README.md and all the doc-comments are in markdown. To separate paragraphs in markdown use an empty line between them.
I tried other variants like double-space or backslash, but an empty line is the most used in the wild.

## tests

I added a unit-test, just to show how it looks. And an integration-test. So it is "ready-to-go".
Run them with `cargo test`.

## examples

In the directory `examples` every rs file is a bin-executable.
Run it with:

```bash
cargo run --example example_1
```

## Error handling thiserror and anyhow

The rule number one is never use `.unwrap()` in your real Rust code. It is a sign, you are not Error handling properly.
Maybe `unwrap()` can be fine for some fast learning examples, but for any real-life Rust code you must use some `Error handling`. There are many different ways to do that in Rust. I choose the pair of libraries `thiserror` and `anyhow`. The first is made for libraries, the second is made for bin-executables.  
The library needs an Enum with all the possible errors that this library can return. With `#[derive(Error)]` this enum get everything needed to be a true Rust error struct. Every error can have a formatting string and a struct of data.  
The bin-executable does not want to be involved in every possible error separately. It needs an umbrella for all possible errors with `anyhow::Result`.  
Inside the code, mostly propagate the errors with the `?` Operator after the `Result` value instead of unwrap() or the match expression.
In the tests we don't want to work with Error handling. There, instead of `.unwrap()`, use the similar function `.expect(&str)` that has an additional description string.

## cargo crev reviews and advisory

We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev) to verify the trustworthiness of each of your dependencies.

Please, spread this info.

You can also read crev reviews quickly on the web:

<https://web.crev.dev/rust-reviews/crates/>

## open-source and free as a beer

My open-source projects are free as a beer (MIT license).

I just love programming.

But I need also to drink. If you find my projects and tutorials helpful,please buy me a beer donating on my [paypal](https://paypal.me/LucianoBestia).

You know the price of a beer in your local bar ;-) So I can drink a free beer for your health :-)

[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[comment]: # (auto_md_to_doc_comments segment end A)
"###,
});
    vec_file.push(crate::FileItem {
        file_name: "Cargo.lock",
        file_content: r###"# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 3

[[package]]
name = "aho-corasick"
version = "0.7.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1e37cfd5e7657ada45f742d6e99ca5788580b5c529dc78faf11ece6dc702656f"
dependencies = [
 "memchr",
]

[[package]]
name = "anyhow"
version = "1.0.56"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4361135be9122e0870de935d7c439aef945b9f9ddd4199a553b5270b49c82a27"

[[package]]
name = "atty"
version = "0.2.14"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d9b39be18770d11421cdb1b9947a45dd3f37e93092cbf377614828a319d5fee8"
dependencies = [
 "hermit-abi",
 "libc",
 "winapi",
]

[[package]]
name = "bestia_dev_cargo_auto_new_cli"
version = "1.0.12"
dependencies = [
 "anyhow",
 "log",
 "pretty_env_logger",
 "thiserror",
]

[[package]]
name = "cfg-if"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "baf1de4339761588bc0619e3cbc0120ee582ebb74b53b4efbf79117bd2da40fd"

[[package]]
name = "env_logger"
version = "0.7.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "44533bbbb3bb3c1fa17d9f2e4e38bbbaf8396ba82193c4cb1b6445d711445d36"
dependencies = [
 "atty",
 "humantime",
 "log",
 "regex",
 "termcolor",
]

[[package]]
name = "hermit-abi"
version = "0.1.19"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "62b467343b94ba476dcb2500d242dadbb39557df889310ac77c5d99100aaac33"
dependencies = [
 "libc",
]

[[package]]
name = "humantime"
version = "1.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "df004cfca50ef23c36850aaaa59ad52cc70d0e90243c3c7737a4dd32dc7a3c4f"
dependencies = [
 "quick-error",
]

[[package]]
name = "libc"
version = "0.2.124"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "21a41fed9d98f27ab1c6d161da622a4fa35e8a54a8adc24bbf3ddd0ef70b0e50"

[[package]]
name = "log"
version = "0.4.16"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6389c490849ff5bc16be905ae24bc913a9c8892e19b2341dbc175e14c341c2b8"
dependencies = [
 "cfg-if",
]

[[package]]
name = "memchr"
version = "2.4.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "308cc39be01b73d0d18f82a0e7b2a3df85245f84af96fdddc5d202d27e47b86a"

[[package]]
name = "pretty_env_logger"
version = "0.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "926d36b9553851b8b0005f1275891b392ee4d2d833852c417ed025477350fb9d"
dependencies = [
 "env_logger",
 "log",
]

[[package]]
name = "proc-macro2"
version = "1.0.37"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ec757218438d5fda206afc041538b2f6d889286160d649a86a24d37e1235afd1"
dependencies = [
 "unicode-xid",
]

[[package]]
name = "quick-error"
version = "1.2.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a1d01941d82fa2ab50be1e79e6714289dd7cde78eba4c074bc5a4374f650dfe0"

[[package]]
name = "quote"
version = "1.0.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a1feb54ed693b93a84e14094943b84b7c4eae204c512b7ccb95ab0c66d278ad1"
dependencies = [
 "proc-macro2",
]

[[package]]
name = "regex"
version = "1.5.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1a11647b6b25ff05a515cb92c365cec08801e83423a235b51e231e1808747286"
dependencies = [
 "aho-corasick",
 "memchr",
 "regex-syntax",
]

[[package]]
name = "regex-syntax"
version = "0.6.25"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f497285884f3fcff424ffc933e56d7cbca511def0c9831a7f9b5f6153e3cc89b"

[[package]]
name = "syn"
version = "1.0.91"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b683b2b825c8eef438b77c36a06dc262294da3d5a5813fac20da149241dcd44d"
dependencies = [
 "proc-macro2",
 "quote",
 "unicode-xid",
]

[[package]]
name = "termcolor"
version = "1.1.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bab24d30b911b2376f3a13cc2cd443142f0c81dda04c118693e35b3835757755"
dependencies = [
 "winapi-util",
]

[[package]]
name = "thiserror"
version = "1.0.30"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "854babe52e4df1653706b98fcfc05843010039b406875930a70e4d9644e5c417"
dependencies = [
 "thiserror-impl",
]

[[package]]
name = "thiserror-impl"
version = "1.0.30"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "aa32fd3f627f367fe16f893e2597ae3c05020f8bba2666a4e6ea73d377e5714b"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "unicode-xid"
version = "0.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8ccb82d61f80a663efe1f787a51b16b5a51e3314d6ac365b08639f52387b33f3"

[[package]]
name = "winapi"
version = "0.3.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419"
dependencies = [
 "winapi-i686-pc-windows-gnu",
 "winapi-x86_64-pc-windows-gnu",
]

[[package]]
name = "winapi-i686-pc-windows-gnu"
version = "0.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6"

[[package]]
name = "winapi-util"
version = "0.1.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "70ec6ce85bb158151cae5e5c87f95a8e97d2c0c4b001223f33a334e3ce5de178"
dependencies = [
 "winapi",
]

[[package]]
name = "winapi-x86_64-pc-windows-gnu"
version = "0.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f"
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "LICENSE",
        file_content: r###"MIT License

Copyright (c) 2022 bestia.dev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "tests/integration_test.rs",
        file_content: r###"// tests/integration_test.rs

use bestia_dev_cargo_auto_new_cli::*;

#[test]
fn integration_test_01() {
    assert_eq!(format_hello_phrase("abcd"), "Hello abcd!");
    assert_eq!(format_upper_hello_phrase("abcd").expect("error"), "Hello ABCD!");
}

#[test]
fn integration_test_02_error_check() {
    assert!(format_upper_hello_phrase("ABCD").is_err());
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "examples/example_1.rs",
        file_content: r###"// examples/example_1.rs

//! A simple example how to use the `lib.rs`
//! You can run it with `cargo run --example example_1`

use bestia_dev_cargo_auto_new_cli::*;

/// example how to use format_hello_phrase() and format_upper_hello_phrase()
fn main() {
    let greet_name = "world";
    let phrase = format_hello_phrase(greet_name);
    println!("{}", phrase);

    // possible error must be processed
    match format_upper_hello_phrase(greet_name) {
        Ok(phrase) => println!("{}", phrase),
        Err(err) => log::error!("Error: {}", err),
    }
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/Cargo.toml",
        file_content: r###"
[package]
name = "automation_tasks_rs"
version = "0.1.1"
authors = ["bestia.dev"]
homepage = "https://bestia.dev"
edition = "2018"
description = "cargo auto - automation tasks written in Rust language"
publish = false

[dependencies]
cargo_auto_lib = "0.7.42""###,
    });
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/.gitignore",
        file_content: r###"/target
    "###,
    });
    vec_file.push(crate::FileItem{
            file_name :"automation_tasks_rs/src/main.rs",
            file_content : r###"//! automation_tasks_rs for bestia_dev_cargo_auto_new_cli

use cargo_auto_lib::*;

// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";


fn main() {
    exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("{YELLOW}Running automation task: {task}{RESET}");
                if &task == "build" {
                    task_build();
                } else if &task == "release" {
                    task_release();
                } else if &task == "doc" {
                    task_doc();
                } else if &task == "test" {
                    task_test();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                /*
                } else if &task == "publish_to_crates_io" {
                    task_publish_to_crates_io();
                */
                } else {
                    println!("{RED}Error: Task {task} is unknown.{RESET}");
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
    {YELLOW}Welcome to cargo-auto !
    This program automates your custom tasks when developing a Rust project.{RESET}

    User defined tasks in automation_tasks_rs:
cargo auto build - builds the crate in debug mode, fmt, increment version
cargo auto release - builds the crate in release mode, fmt, increment version
cargo auto doc - builds the docs, copy to docs directory
cargo auto test - runs all the tests
cargo auto commit_and_push "message" - commits with message and push with mandatory message
    (If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git.)
"#
/*
cargo auto publish_to_crates_io - publish to crates.io, git tag
    (You need credentials for publishing. On crates.io get the 'access token'. Then save it locally once and forever with the command 
    ` cargo login TOKEN` use a space before the command to avoid saving the secret token in bash history.)
*/
    );
    print_examples_cmd();
}

/// all example commands in one place
fn print_examples_cmd(){
/*
    println!(r#"run examples:
cargo run --example example1
"#);
*/
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "release", "doc", "test", "commit_and_push",];
        // , "publish_to_crates_io"
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// cargo build
fn task_build() {
    let cargo_toml = CargoToml::read();
    auto_version_increment_semver_or_date();
    run_shell_command("cargo fmt");
    run_shell_command("cargo build");
    println!(
        r#"{YELLOW}
    After `cargo auto build`, run the compiled binary, examples and/or tests
./target/debug/{package_name} print world
    if ok, then
./target/debug/{package_name} upper world
    if ok, then
./target/debug/{package_name} upper WORLD
    if ok, then
cargo auto test
    if ok, then,
cargo auto release
{RESET}"#,
package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
}

/// cargo build --release
fn task_release() {
    let cargo_toml = CargoToml::read();
    auto_version_increment_semver_or_date();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");

    run_shell_command("cargo fmt");
    run_shell_command("cargo build --release");
    println!(
        r#"{YELLOW}
    After `cargo auto release`, run the compiled binary, examples and/or tests
./target/release/{package_name} print world
    if ok, then
./target/release/{package_name} upper world
    if ok, then
./target/release/{package_name} upper WORLD
    if ok, then
cargo auto test
    if ok, then,
cargo auto doc
{RESET}"#,
package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
}

/// cargo doc, then copies to /docs/ folder, because this is a github standard folder
fn task_doc() {
    let cargo_toml = CargoToml::read();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");
    auto_plantuml(&cargo_toml.package_repository().unwrap());
    auto_md_to_doc_comments();

    run_shell_command("cargo doc --no-deps --document-private-items");
    // copy target/doc into docs/ because it is github standard
    run_shell_command("rsync -a --info=progress2 --delete-after target/doc/ docs/");
    // Create simple index.html file in docs directory
    run_shell_command(&format!(
        "echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",
        cargo_toml.package_name().replace("-","_")
    ));
    run_shell_command("cargo fmt");
    // message to help user with next move
    println!(
        r#"{YELLOW}
    After `cargo auto doc`, check `docs/index.html`. If ok, then test the documentation code examples
cargo auto test
{RESET}"#
    );
}

/// cargo test
fn task_test() {
    run_shell_command("cargo test");
    println!(
        r#"{YELLOW}
    After `cargo auto test`. If ok, then 
cargo auto commit_and_push "message"
    with mandatory commit message
{RESET}"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    match arg_2 {
        None => println!("{RED}Error: Message for commit is mandatory.{RESET}"),
        Some(message) => {
            run_shell_command(&format!(r#"git add -A && git commit --allow-empty -m "{}""#, message));
            run_shell_command("git push");
            println!(
                r#"{YELLOW}
    After `cargo auto commit_and_push "message"`
cargo auto publish_to_crates_io
{RESET}"#
            );
        }
    }
}

/*
/// publish to crates.io and git tag
fn task_publish_to_crates_io() {
    println!(r#"{YELLOW}The crates.io access token must already be saved locally with `cargo login TOKEN`{RESET}"#);

    let cargo_toml = CargoToml::read();
    // git tag
    let shell_command = format!(
        "git tag -f -a v{version} -m version_{version}",
        version = cargo_toml.package_version()
    );
    run_shell_command(&shell_command);

    // cargo publish
    run_shell_command("cargo publish");
    println!(
        r#"{YELLOW}
    After `cargo auto publish_to_crates_io`, 
    check `https://crates.io/crates/{package_name}`.
    Install the crate with `cargo install {package_name}` and check how it works.
    Add the dependency `{package_name} = "{package_version}"` to your Rust project and check how it works.
{RESET}"#,
        package_name = cargo_toml.package_name(),
        package_version = cargo_toml.package_version()
    );
}
*/

// endregion: tasks
"###,
});
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/Cargo.lock",
        file_content: r###"# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 3

[[package]]
name = "adler32"
version = "1.2.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "aae1277d39aeec15cb388266ecc24b11c80469deae6067e17a1a7aa9e5c1f234"

[[package]]
name = "aho-corasick"
version = "0.7.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1e37cfd5e7657ada45f742d6e99ca5788580b5c529dc78faf11ece6dc702656f"
dependencies = [
 "memchr",
]

[[package]]
name = "anyhow"
version = "1.0.56"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4361135be9122e0870de935d7c439aef945b9f9ddd4199a553b5270b49c82a27"

[[package]]
name = "arrayref"
version = "0.3.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a4c527152e37cf757a3f78aae5a06fbeefdb07ccc535c980a3208ee3060dd544"

[[package]]
name = "autocfg"
version = "1.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d468802bab17cbc0cc575e9b053f41e72aa36bfa6b7f55e3529ffa43161b97fa"

[[package]]
name = "automation_tasks_rs"
version = "0.1.1"
dependencies = [
 "cargo_auto_lib",
]

[[package]]
name = "base64"
version = "0.13.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "904dfeac50f3cdaba28fc6f57fdcddb75f49ed61346676a78c4ffe55877802fd"

[[package]]
name = "base64ct"
version = "1.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "dea908e7347a8c64e378c17e30ef880ad73e3b4498346b055c2c00ea342f3179"

[[package]]
name = "bitflags"
version = "1.3.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bef38d45163c2f1dde094a7dfd33ccf595c92905c8f8f4fdc18d06fb1037718a"

[[package]]
name = "block-buffer"
version = "0.10.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0bf7fe51849ea569fd452f37822f606a5cabb684dc918707a0193fd4664ff324"
dependencies = [
 "generic-array",
]

[[package]]
name = "bumpalo"
version = "3.10.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "37ccbd214614c6783386c1af30caf03192f17891059cecc394b4fb119e363de3"

[[package]]
name = "bytes"
version = "1.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c4872d67bab6358e59559027aa3b9157c53d9358c51423c17554809a8858e0f8"

[[package]]
name = "cargo_auto_lib"
version = "0.7.42"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f87245487d1ff625f244d18aca69ce49ffeaedacef47dfaa94034a7837cad544"
dependencies = [
 "anyhow",
 "base64ct",
 "cargo_toml",
 "chrono",
 "deflate",
 "filetime",
 "glob",
 "lazy_static",
 "radix64",
 "reader_for_microxml",
 "regex",
 "reqwest",
 "semver",
 "serde",
 "serde_derive",
 "serde_json",
 "sha2",
 "termion",
 "toml",
 "unwrap",
]

[[package]]
name = "cargo_toml"
version = "0.9.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9c3596addfb02dcdc06f5252ddda9f3785f9230f5827fb4284645240fa05ad92"
dependencies = [
 "serde",
 "serde_derive",
 "toml",
]

[[package]]
name = "cc"
version = "1.0.73"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2fff2a6927b3bb87f9595d67196a70493f627687a71d87a0d692242c33f58c11"

[[package]]
name = "cfg-if"
version = "0.1.10"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4785bdd1c96b2a846b2bd7cc02e86b6b3dbf14e7e53446c4f54c92a361040822"

[[package]]
name = "cfg-if"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "baf1de4339761588bc0619e3cbc0120ee582ebb74b53b4efbf79117bd2da40fd"

[[package]]
name = "chrono"
version = "0.4.19"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "670ad68c9088c2a963aaa298cb369688cf3f9465ce5e2d4ca10e6e0098a1ce73"
dependencies = [
 "libc",
 "num-integer",
 "num-traits",
 "time",
 "winapi",
]

[[package]]
name = "core-foundation"
version = "0.9.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "194a7a9e6de53fa55116934067c844d9d749312f75c6f6d0980e8c252f8c2146"
dependencies = [
 "core-foundation-sys",
 "libc",
]

[[package]]
name = "core-foundation-sys"
version = "0.8.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5827cebf4670468b8772dd191856768aedcb1b0278a04f989f7766351917b9dc"

[[package]]
name = "cpufeatures"
version = "0.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "59a6001667ab124aebae2a495118e11d30984c3a653e99d86d58971708cf5e4b"
dependencies = [
 "libc",
]

[[package]]
name = "crypto-common"
version = "0.1.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "57952ca27b5e3606ff4dd79b0020231aaf9d6aa76dc05fd30137538c50bd3ce8"
dependencies = [
 "generic-array",
 "typenum",
]

[[package]]
name = "deflate"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c86f7e25f518f4b81808a2cf1c50996a61f5c2eb394b2393bd87f2a4780a432f"
dependencies = [
 "adler32",
]

[[package]]
name = "digest"
version = "0.10.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f2fb860ca6fafa5552fb6d0e816a69c8e49f0908bf524e30a90d97c85892d506"
dependencies = [
 "block-buffer",
 "crypto-common",
]

[[package]]
name = "encoding_rs"
version = "0.8.31"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9852635589dc9f9ea1b6fe9f05b50ef208c85c834a562f0c6abb1c475736ec2b"
dependencies = [
 "cfg-if 1.0.0",
]

[[package]]
name = "fastrand"
version = "1.7.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c3fcf0cee53519c866c09b5de1f6c56ff9d647101f81c1964fa632e148896cdf"
dependencies = [
 "instant",
]

[[package]]
name = "filetime"
version = "0.2.16"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c0408e2626025178a6a7f7ffc05a25bc47103229f19c113755de7bf63816290c"
dependencies = [
 "cfg-if 1.0.0",
 "libc",
 "redox_syscall",
 "winapi",
]

[[package]]
name = "fnv"
version = "1.0.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3f9eec918d3f24069decb9af1554cad7c880e2da24a9afd88aca000531ab82c1"

[[package]]
name = "foreign-types"
version = "0.3.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f6f339eb8adc052cd2ca78910fda869aefa38d22d5cb648e6485e4d3fc06f3b1"
dependencies = [
 "foreign-types-shared",
]

[[package]]
name = "foreign-types-shared"
version = "0.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "00b0228411908ca8685dba7fc2cdd70ec9990a6e753e89b6ac91a84c40fbaf4b"

[[package]]
name = "form_urlencoded"
version = "1.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5fc25a87fa4fd2094bffb06925852034d90a17f0d1e05197d4956d3555752191"
dependencies = [
 "matches",
 "percent-encoding",
]

[[package]]
name = "futures-channel"
version = "0.3.21"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c3083ce4b914124575708913bca19bfe887522d6e2e6d0952943f5eac4a74010"
dependencies = [
 "futures-core",
]

[[package]]
name = "futures-core"
version = "0.3.21"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0c09fd04b7e4073ac7156a9539b57a484a8ea920f79c7c675d05d289ab6110d3"

[[package]]
name = "futures-io"
version = "0.3.21"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fc4045962a5a5e935ee2fdedaa4e08284547402885ab326734432bed5d12966b"

[[package]]
name = "futures-sink"
version = "0.3.21"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "21163e139fa306126e6eedaf49ecdb4588f939600f0b1e770f4205ee4b7fa868"

[[package]]
name = "futures-task"
version = "0.3.21"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "57c66a976bf5909d801bbef33416c41372779507e7a6b3a5e25e4749c58f776a"

[[package]]
name = "futures-util"
version = "0.3.21"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d8b7abd5d659d9b90c8cba917f6ec750a74e2dc23902ef9cd4cc8c8b22e6036a"
dependencies = [
 "futures-core",
 "futures-io",
 "futures-task",
 "memchr",
 "pin-project-lite",
 "pin-utils",
 "slab",
]

[[package]]
name = "generic-array"
version = "0.14.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fd48d33ec7f05fbfa152300fdad764757cbded343c1aa1cff2fbaf4134851803"
dependencies = [
 "typenum",
 "version_check",
]

[[package]]
name = "glob"
version = "0.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9b919933a397b79c37e33b77bb2aa3dc8eb6e165ad809e58ff75bc7db2e34574"

[[package]]
name = "h2"
version = "0.3.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "37a82c6d637fc9515a4694bbf1cb2457b79d81ce52b3108bdeea58b07dd34a57"
dependencies = [
 "bytes",
 "fnv",
 "futures-core",
 "futures-sink",
 "futures-util",
 "http",
 "indexmap",
 "slab",
 "tokio",
 "tokio-util",
 "tracing",
]

[[package]]
name = "hashbrown"
version = "0.12.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "db0d4cf898abf0081f964436dc980e96670a0f36863e4b83aaacdb65c9d7ccc3"

[[package]]
name = "hermit-abi"
version = "0.1.19"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "62b467343b94ba476dcb2500d242dadbb39557df889310ac77c5d99100aaac33"
dependencies = [
 "libc",
]

[[package]]
name = "http"
version = "0.2.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "75f43d41e26995c17e71ee126451dd3941010b0514a81a9d11f3b341debc2399"
dependencies = [
 "bytes",
 "fnv",
 "itoa",
]

[[package]]
name = "http-body"
version = "0.4.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d5f38f16d184e36f2408a55281cd658ecbd3ca05cce6d6510a176eca393e26d1"
dependencies = [
 "bytes",
 "http",
 "pin-project-lite",
]

[[package]]
name = "httparse"
version = "1.7.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "496ce29bb5a52785b44e0f7ca2847ae0bb839c9bd28f69acac9b99d461c0c04c"

[[package]]
name = "httpdate"
version = "1.0.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c4a1e36c821dbe04574f602848a19f742f4fb3c98d40449f11bcad18d6b17421"

[[package]]
name = "hyper"
version = "0.14.19"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "42dc3c131584288d375f2d07f822b0cb012d8c6fb899a5b9fdb3cb7eb9b6004f"
dependencies = [
 "bytes",
 "futures-channel",
 "futures-core",
 "futures-util",
 "h2",
 "http",
 "http-body",
 "httparse",
 "httpdate",
 "itoa",
 "pin-project-lite",
 "socket2",
 "tokio",
 "tower-service",
 "tracing",
 "want",
]

[[package]]
name = "hyper-tls"
version = "0.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d6183ddfa99b85da61a140bea0efc93fdf56ceaa041b37d553518030827f9905"
dependencies = [
 "bytes",
 "hyper",
 "native-tls",
 "tokio",
 "tokio-native-tls",
]

[[package]]
name = "idna"
version = "0.2.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "418a0a6fab821475f634efe3ccc45c013f742efe03d853e8d3355d5cb850ecf8"
dependencies = [
 "matches",
 "unicode-bidi",
 "unicode-normalization",
]

[[package]]
name = "indexmap"
version = "1.9.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6c6392766afd7964e2531940894cffe4bd8d7d17dbc3c1c4857040fd4b33bdb3"
dependencies = [
 "autocfg",
 "hashbrown",
]

[[package]]
name = "instant"
version = "0.1.12"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7a5bbe824c507c5da5956355e86a746d82e0e1464f65d862cc5e71da70e94b2c"
dependencies = [
 "cfg-if 1.0.0",
]

[[package]]
name = "ipnet"
version = "2.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "879d54834c8c76457ef4293a689b2a8c59b076067ad77b15efafbb05f92a592b"

[[package]]
name = "itoa"
version = "1.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1aab8fc367588b89dcee83ab0fd66b72b50b72fa1904d7095045ace2b0c81c35"

[[package]]
name = "js-sys"
version = "0.3.58"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c3fac17f7123a73ca62df411b1bf727ccc805daa070338fda671c86dac1bdc27"
dependencies = [
 "wasm-bindgen",
]

[[package]]
name = "lazy_static"
version = "1.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e2abad23fbc42b3700f2f279844dc832adb2b2eb069b2df918f455c4e18cc646"

[[package]]
name = "libc"
version = "0.2.124"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "21a41fed9d98f27ab1c6d161da622a4fa35e8a54a8adc24bbf3ddd0ef70b0e50"

[[package]]
name = "log"
version = "0.4.17"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "abb12e687cfb44aa40f41fc3978ef76448f9b6038cad6aef4259d3c095a2382e"
dependencies = [
 "cfg-if 1.0.0",
]

[[package]]
name = "matches"
version = "0.1.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a3e378b66a060d48947b590737b30a1be76706c8dd7b8ba0f2fe3989c68a853f"

[[package]]
name = "memchr"
version = "2.4.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "308cc39be01b73d0d18f82a0e7b2a3df85245f84af96fdddc5d202d27e47b86a"

[[package]]
name = "mime"
version = "0.3.16"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2a60c7ce501c71e03a9c9c0d35b861413ae925bd979cc7a4e30d060069aaac8d"

[[package]]
name = "mio"
version = "0.8.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "713d550d9b44d89174e066b7a6217ae06234c10cb47819a88290d2b353c31799"
dependencies = [
 "libc",
 "log",
 "wasi 0.11.0+wasi-snapshot-preview1",
 "windows-sys",
]

[[package]]
name = "native-tls"
version = "0.2.10"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fd7e2f3618557f980e0b17e8856252eee3c97fa12c54dff0ca290fb6266ca4a9"
dependencies = [
 "lazy_static",
 "libc",
 "log",
 "openssl",
 "openssl-probe",
 "openssl-sys",
 "schannel",
 "security-framework",
 "security-framework-sys",
 "tempfile",
]

[[package]]
name = "num-integer"
version = "0.1.44"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d2cc698a63b549a70bc047073d2949cce27cd1c7b0a4a862d08a8031bc2801db"
dependencies = [
 "autocfg",
 "num-traits",
]

[[package]]
name = "num-traits"
version = "0.2.14"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9a64b1ec5cda2586e284722486d802acf1f7dbdc623e2bfc57e65ca1cd099290"
dependencies = [
 "autocfg",
]

[[package]]
name = "num_cpus"
version = "1.13.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "19e64526ebdee182341572e50e9ad03965aa510cd94427a4549448f285e957a1"
dependencies = [
 "hermit-abi",
 "libc",
]

[[package]]
name = "numtoa"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b8f8bdf33df195859076e54ab11ee78a1b208382d3a26ec40d142ffc1ecc49ef"

[[package]]
name = "once_cell"
version = "1.12.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7709cef83f0c1f58f666e746a08b21e0085f7440fa6a29cc194d68aac97a4225"

[[package]]
name = "openssl"
version = "0.10.40"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fb81a6430ac911acb25fe5ac8f1d2af1b4ea8a4fdfda0f1ee4292af2e2d8eb0e"
dependencies = [
 "bitflags",
 "cfg-if 1.0.0",
 "foreign-types",
 "libc",
 "once_cell",
 "openssl-macros",
 "openssl-sys",
]

[[package]]
name = "openssl-macros"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b501e44f11665960c7e7fcf062c7d96a14ade4aa98116c004b2e37b5be7d736c"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "openssl-probe"
version = "0.1.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ff011a302c396a5197692431fc1948019154afc178baf7d8e37367442a4601cf"

[[package]]
name = "openssl-sys"
version = "0.9.74"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "835363342df5fba8354c5b453325b110ffd54044e588c539cf2f20a8014e4cb1"
dependencies = [
 "autocfg",
 "cc",
 "libc",
 "pkg-config",
 "vcpkg",
]

[[package]]
name = "percent-encoding"
version = "2.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d4fd5641d01c8f18a23da7b6fe29298ff4b55afcccdf78973b24cf3175fee32e"

[[package]]
name = "pin-project-lite"
version = "0.2.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e0a7ae3ac2f1173085d398531c705756c94a4c56843785df85a60c1a0afac116"

[[package]]
name = "pin-utils"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8b870d8c151b6f2fb93e84a13146138f05d02ed11c7e7c54f8826aaaf7c9f184"

[[package]]
name = "pkg-config"
version = "0.3.25"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1df8c4ec4b0627e53bdf214615ad287367e482558cf84b109250b37464dc03ae"

[[package]]
name = "proc-macro2"
version = "1.0.37"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ec757218438d5fda206afc041538b2f6d889286160d649a86a24d37e1235afd1"
dependencies = [
 "unicode-xid",
]

[[package]]
name = "quote"
version = "1.0.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a1feb54ed693b93a84e14094943b84b7c4eae204c512b7ccb95ab0c66d278ad1"
dependencies = [
 "proc-macro2",
]

[[package]]
name = "radix64"
version = "0.6.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "999718fa65c3be3a74f3f6dae5a98526ff436ea58a82a574f0de89eecd342bee"
dependencies = [
 "arrayref",
 "cfg-if 0.1.10",
]

[[package]]
name = "reader_for_microxml"
version = "2.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0d726a3f4c11def37106edaf44caf861a9012ebcd4eb6f748cc4fd93c2a15de1"

[[package]]
name = "redox_syscall"
version = "0.2.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "62f25bc4c7e55e0b0b7a1d43fb893f4fa1361d0abe38b9ce4f323c2adfe6ef42"
dependencies = [
 "bitflags",
]

[[package]]
name = "redox_termios"
version = "0.1.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8440d8acb4fd3d277125b4bd01a6f38aee8d814b3b5fc09b3f2b825d37d3fe8f"
dependencies = [
 "redox_syscall",
]

[[package]]
name = "regex"
version = "1.5.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1a11647b6b25ff05a515cb92c365cec08801e83423a235b51e231e1808747286"
dependencies = [
 "aho-corasick",
 "memchr",
 "regex-syntax",
]

[[package]]
name = "regex-syntax"
version = "0.6.25"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f497285884f3fcff424ffc933e56d7cbca511def0c9831a7f9b5f6153e3cc89b"

[[package]]
name = "remove_dir_all"
version = "0.5.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3acd125665422973a33ac9d3dd2df85edad0f4ae9b00dafb1a05e43a9f5ef8e7"
dependencies = [
 "winapi",
]

[[package]]
name = "reqwest"
version = "0.11.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b75aa69a3f06bbcc66ede33af2af253c6f7a86b1ca0033f60c580a27074fbf92"
dependencies = [
 "base64",
 "bytes",
 "encoding_rs",
 "futures-core",
 "futures-util",
 "h2",
 "http",
 "http-body",
 "hyper",
 "hyper-tls",
 "ipnet",
 "js-sys",
 "lazy_static",
 "log",
 "mime",
 "native-tls",
 "percent-encoding",
 "pin-project-lite",
 "serde",
 "serde_json",
 "serde_urlencoded",
 "tokio",
 "tokio-native-tls",
 "tower-service",
 "url",
 "wasm-bindgen",
 "wasm-bindgen-futures",
 "web-sys",
 "winreg",
]

[[package]]
name = "ryu"
version = "1.0.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "73b4b750c782965c211b42f022f59af1fbceabdd026623714f104152f1ec149f"

[[package]]
name = "schannel"
version = "0.1.20"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "88d6731146462ea25d9244b2ed5fd1d716d25c52e4d54aa4fb0f3c4e9854dbe2"
dependencies = [
 "lazy_static",
 "windows-sys",
]

[[package]]
name = "security-framework"
version = "2.6.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2dc14f172faf8a0194a3aded622712b0de276821addc574fa54fc0a1167e10dc"
dependencies = [
 "bitflags",
 "core-foundation",
 "core-foundation-sys",
 "libc",
 "security-framework-sys",
]

[[package]]
name = "security-framework-sys"
version = "2.6.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0160a13a177a45bfb43ce71c01580998474f556ad854dcbca936dd2841a5c556"
dependencies = [
 "core-foundation-sys",
 "libc",
]

[[package]]
name = "semver"
version = "1.0.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d65bd28f48be7196d222d95b9243287f48d27aca604e08497513019ff0502cc4"

[[package]]
name = "serde"
version = "1.0.136"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ce31e24b01e1e524df96f1c2fdd054405f8d7376249a5110886fb4b658484789"

[[package]]
name = "serde_derive"
version = "1.0.136"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "08597e7152fcd306f41838ed3e37be9eaeed2b61c42e2117266a554fab4662f9"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "serde_json"
version = "1.0.79"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8e8d9fa5c3b304765ce1fd9c4c8a3de2c8db365a5b91be52f186efc675681d95"
dependencies = [
 "itoa",
 "ryu",
 "serde",
]

[[package]]
name = "serde_urlencoded"
version = "0.7.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d3491c14715ca2294c4d6a88f15e84739788c1d030eed8c110436aafdaa2f3fd"
dependencies = [
 "form_urlencoded",
 "itoa",
 "ryu",
 "serde",
]

[[package]]
name = "sha2"
version = "0.10.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "55deaec60f81eefe3cce0dc50bda92d6d8e88f2a27df7c5033b42afeb1ed2676"
dependencies = [
 "cfg-if 1.0.0",
 "cpufeatures",
 "digest",
]

[[package]]
name = "slab"
version = "0.4.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "eb703cfe953bccee95685111adeedb76fabe4e97549a58d16f03ea7b9367bb32"

[[package]]
name = "socket2"
version = "0.4.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "66d72b759436ae32898a2af0a14218dbf55efde3feeb170eb623637db85ee1e0"
dependencies = [
 "libc",
 "winapi",
]

[[package]]
name = "syn"
version = "1.0.91"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b683b2b825c8eef438b77c36a06dc262294da3d5a5813fac20da149241dcd44d"
dependencies = [
 "proc-macro2",
 "quote",
 "unicode-xid",
]

[[package]]
name = "tempfile"
version = "3.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5cdb1ef4eaeeaddc8fbd371e5017057064af0911902ef36b39801f67cc6d79e4"
dependencies = [
 "cfg-if 1.0.0",
 "fastrand",
 "libc",
 "redox_syscall",
 "remove_dir_all",
 "winapi",
]

[[package]]
name = "termion"
version = "1.5.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "077185e2eac69c3f8379a4298e1e07cd36beb962290d4a51199acf0fdc10607e"
dependencies = [
 "libc",
 "numtoa",
 "redox_syscall",
 "redox_termios",
]

[[package]]
name = "time"
version = "0.1.44"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6db9e6914ab8b1ae1c260a4ae7a49b6c5611b40328a735b21862567685e73255"
dependencies = [
 "libc",
 "wasi 0.10.0+wasi-snapshot-preview1",
 "winapi",
]

[[package]]
name = "tinyvec"
version = "1.6.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "87cc5ceb3875bb20c2890005a4e226a4651264a5c75edb2421b52861a0a0cb50"
dependencies = [
 "tinyvec_macros",
]

[[package]]
name = "tinyvec_macros"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "cda74da7e1a664f795bb1f8a87ec406fb89a02522cf6e50620d016add6dbbf5c"

[[package]]
name = "tokio"
version = "1.19.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c51a52ed6686dd62c320f9b89299e9dfb46f730c7a48e635c19f21d116cb1439"
dependencies = [
 "bytes",
 "libc",
 "memchr",
 "mio",
 "num_cpus",
 "once_cell",
 "pin-project-lite",
 "socket2",
 "winapi",
]

[[package]]
name = "tokio-native-tls"
version = "0.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f7d995660bd2b7f8c1568414c1126076c13fbb725c40112dc0120b78eb9b717b"
dependencies = [
 "native-tls",
 "tokio",
]

[[package]]
name = "tokio-util"
version = "0.7.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "cc463cd8deddc3770d20f9852143d50bf6094e640b485cb2e189a2099085ff45"
dependencies = [
 "bytes",
 "futures-core",
 "futures-sink",
 "pin-project-lite",
 "tokio",
 "tracing",
]

[[package]]
name = "toml"
version = "0.5.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8d82e1a7758622a465f8cee077614c73484dac5b836c02ff6a40d5d1010324d7"
dependencies = [
 "serde",
]

[[package]]
name = "tower-service"
version = "0.3.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b6bc1c9ce2b5135ac7f93c72918fc37feb872bdc6a5533a8b85eb4b86bfdae52"

[[package]]
name = "tracing"
version = "0.1.35"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a400e31aa60b9d44a52a8ee0343b5b18566b03a8321e0d321f695cf56e940160"
dependencies = [
 "cfg-if 1.0.0",
 "pin-project-lite",
 "tracing-core",
]

[[package]]
name = "tracing-core"
version = "0.1.27"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7709595b8878a4965ce5e87ebf880a7d39c9afc6837721b21a5a816a8117d921"
dependencies = [
 "once_cell",
]

[[package]]
name = "try-lock"
version = "0.2.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "59547bce71d9c38b83d9c0e92b6066c4253371f15005def0c30d9657f50c7642"

[[package]]
name = "typenum"
version = "1.15.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "dcf81ac59edc17cc8697ff311e8f5ef2d99fcbd9817b34cec66f90b6c3dfd987"

[[package]]
name = "unicode-bidi"
version = "0.3.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "099b7128301d285f79ddd55b9a83d5e6b9e97c92e0ea0daebee7263e932de992"

[[package]]
name = "unicode-normalization"
version = "0.1.19"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d54590932941a9e9266f0832deed84ebe1bf2e4c9e4a3554d393d18f5e854bf9"
dependencies = [
 "tinyvec",
]

[[package]]
name = "unicode-xid"
version = "0.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8ccb82d61f80a663efe1f787a51b16b5a51e3314d6ac365b08639f52387b33f3"

[[package]]
name = "unwrap"
version = "1.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7e33648dd74328e622c7be51f3b40a303c63f93e6fa5f08778b6203a4c25c20f"

[[package]]
name = "url"
version = "2.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a507c383b2d33b5fc35d1861e77e6b383d158b2da5e14fe51b83dfedf6fd578c"
dependencies = [
 "form_urlencoded",
 "idna",
 "matches",
 "percent-encoding",
]

[[package]]
name = "vcpkg"
version = "0.2.15"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "accd4ea62f7bb7a82fe23066fb0957d48ef677f6eeb8215f372f52e48bb32426"

[[package]]
name = "version_check"
version = "0.9.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "49874b5167b65d7193b8aba1567f5c7d93d001cafc34600cee003eda787e483f"

[[package]]
name = "want"
version = "0.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1ce8a968cb1cd110d136ff8b819a556d6fb6d919363c61534f6860c7eb172ba0"
dependencies = [
 "log",
 "try-lock",
]

[[package]]
name = "wasi"
version = "0.10.0+wasi-snapshot-preview1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1a143597ca7c7793eff794def352d41792a93c481eb1042423ff7ff72ba2c31f"

[[package]]
name = "wasi"
version = "0.11.0+wasi-snapshot-preview1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9c8d87e72b64a3b4db28d11ce29237c246188f4f51057d65a7eab63b7987e423"

[[package]]
name = "wasm-bindgen"
version = "0.2.81"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7c53b543413a17a202f4be280a7e5c62a1c69345f5de525ee64f8cfdbc954994"
dependencies = [
 "cfg-if 1.0.0",
 "wasm-bindgen-macro",
]

[[package]]
name = "wasm-bindgen-backend"
version = "0.2.81"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5491a68ab4500fa6b4d726bd67408630c3dbe9c4fe7bda16d5c82a1fd8c7340a"
dependencies = [
 "bumpalo",
 "lazy_static",
 "log",
 "proc-macro2",
 "quote",
 "syn",
 "wasm-bindgen-shared",
]

[[package]]
name = "wasm-bindgen-futures"
version = "0.4.31"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "de9a9cec1733468a8c657e57fa2413d2ae2c0129b95e87c5b72b8ace4d13f31f"
dependencies = [
 "cfg-if 1.0.0",
 "js-sys",
 "wasm-bindgen",
 "web-sys",
]

[[package]]
name = "wasm-bindgen-macro"
version = "0.2.81"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c441e177922bc58f1e12c022624b6216378e5febc2f0533e41ba443d505b80aa"
dependencies = [
 "quote",
 "wasm-bindgen-macro-support",
]

[[package]]
name = "wasm-bindgen-macro-support"
version = "0.2.81"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7d94ac45fcf608c1f45ef53e748d35660f168490c10b23704c7779ab8f5c3048"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
 "wasm-bindgen-backend",
 "wasm-bindgen-shared",
]

[[package]]
name = "wasm-bindgen-shared"
version = "0.2.81"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6a89911bd99e5f3659ec4acf9c4d93b0a90fe4a2a11f15328472058edc5261be"

[[package]]
name = "web-sys"
version = "0.3.58"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2fed94beee57daf8dd7d51f2b15dc2bcde92d7a72304cdf662a4371008b71b90"
dependencies = [
 "js-sys",
 "wasm-bindgen",
]

[[package]]
name = "winapi"
version = "0.3.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419"
dependencies = [
 "winapi-i686-pc-windows-gnu",
 "winapi-x86_64-pc-windows-gnu",
]

[[package]]
name = "winapi-i686-pc-windows-gnu"
version = "0.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6"

[[package]]
name = "winapi-x86_64-pc-windows-gnu"
version = "0.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f"

[[package]]
name = "windows-sys"
version = "0.36.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ea04155a16a59f9eab786fe12a4a450e75cdb175f9e0d80da1e17db09f55b8d2"
dependencies = [
 "windows_aarch64_msvc",
 "windows_i686_gnu",
 "windows_i686_msvc",
 "windows_x86_64_gnu",
 "windows_x86_64_msvc",
]

[[package]]
name = "windows_aarch64_msvc"
version = "0.36.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9bb8c3fd39ade2d67e9874ac4f3db21f0d710bee00fe7cab16949ec184eeaa47"

[[package]]
name = "windows_i686_gnu"
version = "0.36.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "180e6ccf01daf4c426b846dfc66db1fc518f074baa793aa7d9b9aaeffad6a3b6"

[[package]]
name = "windows_i686_msvc"
version = "0.36.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e2e7917148b2812d1eeafaeb22a97e4813dfa60a3f8f78ebe204bcc88f12f024"

[[package]]
name = "windows_x86_64_gnu"
version = "0.36.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4dcd171b8776c41b97521e5da127a2d86ad280114807d0b2ab1e462bc764d9e1"

[[package]]
name = "windows_x86_64_msvc"
version = "0.36.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c811ca4a8c853ef420abd8592ba53ddbbac90410fab6903b3e79972a631f7680"

[[package]]
name = "winreg"
version = "0.10.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "80d0f4e272c85def139476380b12f9ac60926689dd2e01d4923222f40580869d"
dependencies = [
 "winapi",
]
"###,
    });
    // endregion: files copied into strings by automation tasks

    // return
    vec_file
}
