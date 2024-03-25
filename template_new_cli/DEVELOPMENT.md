# Development details

## CRUSTDE - Containerized Rust Development Environment

I recommend using the CRUSTDE - Containerized Rust Development Environment to write Rust projects. Follow the instructions here <https://github.com/CRUSTDE-Containerized-Rust-DevEnv/crustde_cnt_img_pod>.  

It is an isolated development environment that will not mess with you system.
It will work on Linux (tested on Debian) and inside WSL (Windows Subsystem for Linux).

You just need to install the newer alternative to Docker: [podman](https://podman.io/). Then you download the prepared container image from DockerHub (3GB). And then a little juggling with ssh keys. All this is simplified by running a few bash scripts. Just follow the easy instructions.  

The container image contains cargo, rustc, wasm-pack, basic-http-server, cargo-auto and other utils that a Rust project needs.  

## Workflow with automation_tasks_rs and cargo-auto

For easy workflow, use the automation tasks that are already coded in the sub-project `automation_tasks_rs`. This is a basic workflow:

```bash
cargo auto build
cargo auto release
cargo auto doc
cargo auto test
cargo auto commit_and push
cargo auto publish_to_crates_io
cargo auto github_new_release
```

Every task finishes with instructions how to proceed.  
The [cargo-auto](https://github.com/automation-tasks-rs/cargo-auto) and [dev_bestia_cargo_completion](https://github.com/automation-tasks-rs/dev_bestia_cargo_completion) are already installed inside the CRUSTDE container.

You can open the automation sub-project in VSCode and then code your own tasks in Rust.

```bash
code automation_tasks_rs
```

## Separate main.rs and lib.rs

It is always good to split the project between a `main.rs` (executable) and a `lib.rs` (library crate).

Even for the smallest project. Maybe some other program will use the library eventually.

All the input/output is coded in the `main.rs`: keyboard and monitor (stdin and stdout), access to files, and some access to the network.  
The library must not operate directly with the stdin/stdout, because some other caller of the library can have other ideas around input-output options. Maybe it is a Graphical user interface that does things completely different than CLI applications.

A separate `lib.rs` enables one to make good tests and examples without worrying about input-output.

## super simple argument parsing

I use a super simple code to parse CLI arguments inside the `src/bin/cargo_auto_template_new_cli/main.rs`. There are crate libraries that enable very complex argument parsing if needed.

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

Rule number one is never to use `.unwrap()` in your real Rust code. It is a sign, you are not Error handling properly.
Maybe `unwrap()` can be fine for some fast learning examples, but for any real-life Rust code, you must use some `Error handling`. There are many different ways to do that in Rust. I choose the pair of libraries `thiserror` and `anyhow`. The first is made for libraries, the second is made for bin-executables.  
The library needs an Enum with all the possible errors that this library can return. With `#[derive(Error)]` this enum gets everything needed to be a true Rust error struct. Every error can have a formatting string and a struct of data.  
The bin-executable does not want to be involved in every possible error separately. It needs an umbrella for all possible errors with `anyhow::Result`.  
Inside the code, mostly propagate the errors with the `?` Operator after the `Result` value instead of unwrap() or the match expression.
In the tests we don't want to work with Error handling. There, instead of `.unwrap()`, use the similar function `.expect(&str)` that has an additional description string. I use expect() when I am 100% sure the panic cannot happen because I checked some conditions before it.  
