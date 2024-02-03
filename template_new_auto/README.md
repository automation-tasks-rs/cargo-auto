# automation_tasks_rs

In this sub-project `automation_tasks_rs`, you can write tasks that you need when compiling or managing your Rust project.  
The simple `cargo build` and `cargo build --release` are sometimes not enough. We need to copy some files, to prepare some environment. It is nice to have `all` the tasks in one place with a sort order that new users can easily follow.  
It is a Rust project, so you don't have to learn another strange language for automation.  
This helper project is used in combination with the program `cargo-auto`. Install it with `cargo install cargo-auto`.
You can use also the cargo bash completion program `cargo install dev_bestia_cargo_completion`.  

Don't put any secrets like passwords, passphrases, or tokens here, because this helper project is pushed to the remote repository together with the main Rust project.  

In the main  project folder (where the Cargo.toml or Cargo-auto.toml file is) run

```bash
cargo auto
```

You will get the list of possible tasks with descriptions like this:
user-defined tasks

You can write any task you need. You have all the power of the Rust language under your fingertips.  
You can use or write a library for some specific tasks you need.  
For example, there is the crate `cargo_auto_github_lib` if you need to create a Release on GitHub.  
