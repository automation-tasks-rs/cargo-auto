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
