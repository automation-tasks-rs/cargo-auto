# Development details

## Workflow with automation_tasks_rs and cargo-auto

First install `cargo-auto` and `dev_bestia_cargo_completion` to enable automation tasks coded in Rust.

```bash
cargo install cargo-auto
cargo install dev_bestia_cargo_completion
```

Automation tasks that are used repetitively are coded in the sub-project `automation_tasks_rs`.
This is a basic workflow:

```bash
cargo auto build
cargo auto release
cargo auto doc
cargo auto test
cargo auto commit_and push
cargo auto publish_to_crates_io
cargo auto github_new_release
```

## Development of cargo-auto

I am using the previous version of `cargo-auto` to develop the next version. I added the `automation_tasks_rs` folder and prepared the automation tasks that are used repetitively.

## Templates

Inside the cargo-auto project, there are some Rust sub-projects that are templates. I can open a new editor for these directories and build these crates independently. So it is easy to debug and develop.  
Sadly, I cannot publish these directories and files to `crates.io`. I can effectively publish only the source code inside my main Rust project `cargo-auto`.  
Therefore, before publishing I copy the content of these files into the modules `template_new_auto_mod.rs` on every build. It is not difficult now that Rust has fantastic [raw strings](https://doc.rust-lang.org/rust-by-example/std/str.html). For this repetitive task as always, I prepared an automation task in `automation_tasks_rs`.
