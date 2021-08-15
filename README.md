# cargo-auto  

[comment]: # (lmake_readme cargo.toml data start)

[comment]: # (lmake_readme cargo.toml data end)  

[comment]: # (lmake_lines_of_code start)

[comment]: # (lmake_lines_of_code end)

cargo-auto : automation tasks written in Rust language for the build process of rust projects

## Try it

Inside your main rust project directory (the one with Cargo.toml) run:  

```bash
cargo install cargo-auto
cargo auto new
```

This will generate a new directory `automation_tasks_rs` in your main rust project. It is a helper project with complete directory structure for rust projects and completely independent from the main rust project. You can open it in a new editor and view/edit the rust code.  
Then try this common tasks in the main rust project directory:  

```bash
cargo auto build
cargo auto release
cargo auto docs
```

## Motivation

Cargo is a great tool for building rust projects. It has all the basics: `cargo build`, `cargo build --release`, `cargo fmt`, `cargo test`, `cargo doc`,...  
But sometimes we need to do more things like copying some files, publish to ftp or enter long commands. These repetitive tasks must be automated.  
Task automation makes work easier and faster, simplifies the workflow, while improving the consistency and accuracy of workflows.  
This is also sometimes referred to as "workflow automation."  
There are many different build systems and task runners there: `make`, `cmake`, `shell scripts`, `cargo-make`, `cargo-script`, `cargo-run-script`, `runner`, `python scripts`, `powershell scripts`, `cmd prompt scripts`, ...  
Sadly there is no standard in the rust community for now.  
I want something similar to [build.rs](https://doc.rust-lang.org/cargo/reference/build-scripts.html), so I can write my "tasks" in pure rust. I don't want to learn another meta language with weird syntax and difficult to debug. So I will make something really simple, easy, rusty and extensible.  

## cargo auto - automation tasks written in Rust language

Rust is a compiled language. It is not really a scripting or interpreted language. But the compilation of small projects is really fast and can be ignored. Subsequent calls will use the already built binary and so the speed will be even faster.  
This tool is meant for rust projects, so it means that all the rust infrastructure is already in place.  

## automation_tasks_rs helper project

The command `cargo auto new` will create a new directory `automation_tasks_rs` with a template for a helper rust project in the root directory of your `main rust project` . It should not interfere with the main rust project. This directory will be added into git commits and pushed to remote repositories as part of the main project. It has its own `.gitignore` to avoid committing its target directory.  
The `automation_tasks_rs` helper project contains user defined tasks in rust code. This helper project should be opened in a new editor starting from the `automation_tasks_rs` directory. It does not share dependencies with the main project. It is completely separate and independent.  

## cargo auto subcommand

The command `cargo install cargo-auto` will add a new subcommand to cargo:

```bash
cargo auto
```

This binary is super simple. It has only 3 trivial dependencies: `unwrap`, `termion` and `lazy_static`.  
The binary only reads the CLI arguments and runs the `automation_tasks_rs` binary with them. If needed it will compile `automation_tasks_rs` first.  
The code-flow of the source code of `cargo-auto` is simple, fully commented and straightforward to audit. The source code is on [GitHub](https://github.com/LucianoBestia/cargo-auto) with MIT open-source licensing.  

## templates

Inside the cargo-auto project there are 2 directories with rust sub-projects as templates. I can open a new editor for these directories and build this crates independently. So it is easy to debug and develop.  
Sadly, I cannot publish these directories and files to `crates.io`. I can effectively publish only the source code inside my main rust project `cargo-auto`.  
Therefor, before publishing I must copy the text of these files into the modules `template_basic` and `template_with_lib`. It is not difficult now that rust has fantastic [raw strings](https://doc.rust-lang.org/rust-by-example/std/str.html).  

## template_basic

The command  

```rust
cargo auto new
```

will copy the `template_basic` into `automation_tasks_rs` directory.  
This has no dependencies at all, except `std`. It is really simple to understand how it works.  
Open the directory `automation_tasks_rs` in a new editor, explore and add your own tasks in rust code. It is a plain CLI rust project, you can do everything you need with it. Add dependencies and stuff. No limits. This helper project will be added to you commits and stay part of your project.  
Then in the main project run your task (the task name here is `build`):  

```rust
cargo auto build
```

Your code will be compiled (if needed) and executed.  

## template_with_lib

This is a work in progress.  
The goal is to create a library crate [cargo_auto_lib](https://github.com/LucianoBestia/cargo_auto_lib) with many functions that are commonly used when building rust projects.  

```rust
cargo auto new with_lib
```

## development

This crate does not use itself for automation. It is so simple, that cargo alone is enough.  
Usually I compile and run the code with added arguments like this:  

```bash
cargo run -- new
cargo run -- build
cargo run -- release
```

After my first publish to crates.io I discovered that the position of the argument changes if it is used as:  
`cargo-auto new`      - new is 1st arg  
or  
`cargo auto new`      - new is 2nd arg  

## cargo crev reviews and advisory

We leave in times of danger with `supply chain attacks`.  
It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
You can also read reviews quickly on the web. Example for the crate `num-traits`:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  

## open-source free and free as a beer

My open-source projects are free and free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer or two donating on my [paypal](https://www.paypal.com/paypalme/LucianoBestia). You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) !
