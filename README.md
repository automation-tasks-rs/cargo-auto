[//]: # (auto_md_to_doc_comments segment start A)

# cargo-auto  

[//]: # (auto_cargo_toml_to_md start)

**Automation tasks coded in Rust language for the workflow of Rust projects**  
***version: 2024.307.1516 date: 2024-03-07 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/automation-tasks-rs/cargo-auto)***

 ![maintained](https://img.shields.io/badge/maintained-green)
 ![ready-for-use](https://img.shields.io/badge/ready_for_use-green)
 ![rustlang](https://img.shields.io/badge/rustlang-orange)
 ![automation](https://img.shields.io/badge/automation-orange)
 ![workflow](https://img.shields.io/badge/workflow-orange)

[//]: # (auto_cargo_toml_to_md end)

 ![logo](https://raw.githubusercontent.com/automation-tasks-rs/cargo-auto/main/images/logo/logo_cargo_auto.svg)
 cargo-auto is part of [automation_tasks_rs](https://github.com/automation-tasks-rs) project

 [![crates.io](https://img.shields.io/crates/v/cargo-auto.svg)](https://crates.io/crates/cargo-auto)
 [![Documentation](https://docs.rs/cargo-auto/badge.svg)](https://docs.rs/cargo-auto/)
 [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/cargo-auto.svg)](https://web.crev.dev/rust-reviews/crate/cargo-auto/)
 [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/cargo-auto/)  
 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/automation-tasks-rs/cargo-auto/blob/master/LICENSE)
 [![Rust](https://github.com/automation-tasks-rs/cargo-auto/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/automation-tasks-rs/cargo-auto/)
 [![Newest docs](https://img.shields.io/badge/newest_docs-purple.svg)](https://automation-tasks-rs.github.io/cargo-auto/cargo_auto/index.html)
 ![cargo-auto](https://bestia.dev/webpage_hit_counter/get_svg_image/959103982.svg)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-9026-green.svg)](https://github.com/automation-tasks-rs/cargo-auto/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-735-blue.svg)](https://github.com/automation-tasks-rs/cargo-auto/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-427-purple.svg)](https://github.com/automation-tasks-rs/cargo-auto/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/automation-tasks-rs/cargo-auto/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-7840-orange.svg)](https://github.com/automation-tasks-rs/cargo-auto/)

[//]: # (auto_lines_of_code end)

Hashtags: #maintained #ready-for-use #rustlang #automation #workflow  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  
I recommend using the [CRDE - Containerized Rust Development Environment](https://github.com/automation-tasks-rs/docker_rust_development) to write Rust projects on Linux, isolated from your system.  

## Try it

First, we will use `cargo-auto` to create a new empty CLI Rust project similar to `cargo new`, but with a more complete project structure.  

 ```bash
cargo install cargo-auto
cargo auto new_cli my_hello_project
cd my_hello_project
cargo auto
# it lists all the prepared automation tasks
# try a few
cargo auto build
cargo auto release
cargo auto doc
cargo auto test
```

We can also add `automation tasks` to an existing Rust project.
Inside your Rust project directory (the one with Cargo.toml) run:  

```bash
cargo auto new_auto
cargo auto
# it lists all the prepared automation tasks
# try to build
cargo auto build
```

Congratulations! You are already using `cargo-auto`. Simple as that.  
Now you can modify the tasks to your needs. It is all Rust language.  

## Motivation

Cargo is a great tool for building Rust projects. It has all the basics: `cargo build`, `cargo build --release`, `cargo fmt`, `cargo test`, `cargo doc`,...  
But sometimes we need to do more things like copying some files, publishing to FTP, or entering long commands. These repetitive tasks must be automated.  
Task automation makes work easier and faster, and simplifies the workflow while improving the consistency and accuracy of workflows.  
This is also sometimes referred to as "workflow automation."  
There are many different build systems and task runners there: `make`, `cmake`, `shell scripts`, `cargo-xtask`, `cargo-make`, `cargo-task`, `cargo-script`, `cargo-run-script`, `runner`, `python scripts`, `powershell scripts`, `cmd prompt scripts`, ...  
Sadly there is no standard in the Rust community for now.  
I want something similar to [build.rs](https://doc.rust-lang.org/cargo/reference/build-scripts.html), so I can write my "tasks" in pure Rust I don't want to learn another meta language with weird syntax and difficulty to debug. So I will make something really simple, easy, rusty, and extensible.  

## cargo auto subcommand

The command `cargo install cargo-auto` will add a new subcommand to cargo:

```bash
cargo auto
```

This binary is super simple. It has only 1 trivial dependency: `lazy_static`.  
The binary only reads the CLI arguments and runs the `automation_tasks_rs` binary with them. If needed it will compile `automation_tasks_rs` first.  
The code-flow of the source code of `cargo-auto` is simple, fully commented, and straightforward to audit.  
The source code is on [GitHub](https://github.com/automation-tasks-rs/cargo-auto) with MIT open-source licensing.  

## bash auto-completion

With the help of the crate [dev_bestia_cargo_completion](https://crates.io/crates/dev_bestia_cargo_completion), the commands `cargo` and `cargo auto` get bash auto-completion. Try it!  

## cargo auto new_cli

I like very much that Rust has the command `cargo new project_name`. It creates a super simple Rust Hello project that can be built and run immediately. But this example is too simple. It lacks the basic file structures of a serious CLI program.  
I composed an opinionated template for a Rust CLI project. It is easy to run:

```bash
cargo auto new_cli project_name
# then
cd project_name
cargo auto build
# then follow detailed instructions
```

## cargo auto new_wasm

I composed an opinionated template for a simple Rust WASM project for a browser. It is very similar to the new_cli template but for WASM.  
It is easy to run:

```bash
cargo auto new_wasm project_name
# then
cd project_name
cargo auto build
# then follow detailed instructions
```

## cargo auto new_pwa_wasm

I composed an opinionated template for a simple Rust PWA-WASM project for a browser. It is very similar to the new_cli template but for WASM. It adds the PWA standard functionality to work as an offline app.  
The template needs the title, name, long name, and description inside a `pwa.json5` file and the `icon512x512.png` file for the icons.  
It is easy to run:

```bash
cargo auto new_pwa_wasm
# on first run it will just create the `pwa.json5` and `icon512x512.png` files
# modify these files with data for your new app and then repeat
cargo auto new_pwa_wasm
# then
cd project_name
cargo auto build
# then follow detailed instructions
```

## scripting with rust

Rust is a compiled language. It is not really a scripting or interpreted language. But the compilation of small projects is really fast and can be ignored. Subsequent calls will use the already-built binary so the speed will be even faster.  
This tool `cargo-auto` is meant for Rust projects, so it means that all the Rust infrastructure is already in place.  

## automation_tasks_rs Rust sub-project

The command `cargo auto new_auto` will create a new Rust sub-project`automation_tasks_rs` inside your `Rust project`. It should not interfere with the main Rust project. This directory will be added to git commits and pushed to remote repositories as part of the main project. It has its own `.gitignore` to avoid committing to its target directory.  
The `automation_tasks_rs` helper project contains user-defined tasks in Rust code. Your tasks. This helper project should be opened in a new editor starting from the `automation_tasks_rs` directory. It does not share dependencies with the main project. It is completely separate and independent.  
You can edit it and add your dependencies and Rust code. No limits. Freedom of expression.  
This is now your code, your tasks, and your helper Rust project!  
Because only you know what you want to automate and how to do it.  
Never write secrets, passwords, passcodes, or tokens inside your Rust code. Because then it is pushed to GitHub and the whole world can read it in the next second!
Basic example (most of the useful functions are already there):  

```rust ignore
/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args){
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {            
            println!("Running auto task: {}", &task);
            if &task == "build"{
                task_build();
            } else if &task == "release" {
                task_release();
            } else if &task == "doc" {
                task_doc();
            } else {
                println!("Task {} is unknown.", &task);
                print_help();
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(r#"
    User defined tasks in automation_tasks_rs:
cargo auto build - builds the crate in debug mode
cargo auto release - builds the crate in release mode
cargo auto docs - builds the docs
"#);
}

// region: tasks

/// cargo build
fn task_build() {
    run_shell_command("cargo fmt");
    run_shell_command("cargo build");
}

/// cargo build --release
fn task_release() {
    run_shell_command("cargo fmt");
    run_shell_command("cargo build --release");
}

/// cargo doc, then copies to /docs/ folder, because this is a github standard folder
fn task_doc() {
    run_shell_command("cargo doc --no-deps --document-private-items");
    // copy target/doc into docs/ because it is github standard
    run_shell_command("rsync -a --info=progress2 --delete-after target/doc/ docs/");
    // Create simple index.html file in docs directory
    run_shell_command(&format!(
        "echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",
        cargo_toml.package_name().replace("-","_")
    ));
    run_shell_command("cargo fmt");
}

// endregion: tasks

```

## more complex tasks

You can write more complex tasks in Rust language.  
For example in this project I use automation to create GitHub Releases: <https://github.com/automation-tasks-rs/dropbox_backup_to_external_disk>  
Here is a pretty complex workspace with more sub-projects:  
<https://github.com/automation-tasks-rs/cargo_crev_reviews_workspace>  
There is no end to your imagination. If you write something that looks like it can help other developers, please share it with me and I will add it here.

## Development details

Read the development details in a separate md file:  
[DEVELOPMENT.md](https://github.com/automation-tasks-rs/cargo-auto/blob/main/DEVELOPMENT.md)

## Releases changelog

Read the changelog in a separate md file:  
[RELEASES.md](https://github.com/automation-tasks-rs/cargo-auto/blob/main/RELEASES.md)

## TODO

Nothing big in the near future.

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/automation-tasks-rs](https://github.com/automation-tasks-rs)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
