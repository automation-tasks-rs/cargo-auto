# Development details

## CRUSTDE - Containerized Rust Development Environment

I recommend using the CRUSTDE - Containerized Rust Development Environment to write Rust projects.  
Follow the instructions here  
<https://github.com/CRUSTDE-ContainerizedRustDevEnvrustde_cnt_img_pod>.  

It is an isolated development environment that will not mess with you system.
It will work on Linux (tested on Debian) and inside WSL (Windows Subsystem for Linux).

You just need to install the Docker newer alternative: [Podman](https://podman.io/).  
Then you download the prepared container image from DockerHub (3GB).  
And then a little juggling with ssh keys.  
All this is simplified by running a few bash scripts.  
Just follow the easy instructions.  

The container image contains cargo, rustc, wasm-pack, basic-http-server, cargo-auto and other utils that a Rust project needs.  

## Workflow with automation_tasks_rs

Automation tasks that are already coded in the sub-project `automation_tasks_rs`. This is a basic workflow:

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

## Development of cargo-auto

I am using the previous version of `cargo-auto` to develop the next version. I added the `automation_tasks_rs` folder and prepared the automation tasks that are used repetitively.

## Templates

Inside the cargo-auto project, there are some Rust sub-projects that are templates. I can open a new editor for these directories and build these crates independently. So it is easy to debug and develop.  
Sadly, I cannot publish these directories and files to `crates.io`. I can effectively publish only the source code inside my main Rust project `cargo-auto`.  
Therefore, before publishing I copy the content of these files into the modules `template_new_auto_for_cli_mod.rs` on every build. It is not difficult now that Rust has fantastic [raw strings](https://doc.rust-lang.org/rust-by-example/std/str.html). For this repetitive task as always, I prepared an automation task in `automation_tasks_rs`.

## Error handling thiserror and anyhow

Rule number one is never to use `.unwrap()` and `panic!()` in your real Rust code. It is a sign, you are not Error handling properly.
When using panic or even worse process.exit() the program will not finish execution in a nice way. Avoid that.  
Maybe `unwrap()` can be fine for some fast learning examples, but for any real-life Rust code, you must use some `Error handling`. There are many different ways to do that in Rust. I choose the pair of libraries `thiserror` and `anyhow`. The first is made for libraries, the second is made for bin-executables.  
The library needs an Enum with all the possible errors that this library can return. With `#[derive(Error)]` this enum gets everything needed to be a true Rust error struct. Every error can have a formatting string and a struct of data. Internal errors can be propagated without change using the `transparent` cfg.  
To transform `Option<>` into `Result<>` when using `thiserror` use `ok_or_else(||)`.  
The bin-executable does not want to be involved in every possible error separately. It needs an umbrella for all possible errors with `anyhow::Result`.  
Inside the code, mostly propagate the errors with the `?` Operator after the `Result` value instead of unwrap() or the match expression.
To transform `Option<>` into `Result<>` `use anyhow::Context` trait and `context()` method.  
In the tests we don't want to work with Error handling. There, instead of `.unwrap()`, use the similar function `.expect(&str)` that has an additional description string. I use `expect()` when I am 100% sure the panic cannot happen because I checked some conditions before it.  
