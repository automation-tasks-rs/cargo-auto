# automation_tasks_rs

In this sub-project `automation_tasks_rs` you can write tasks that you need when compiling or managing your Rust project.  
The simple `cargo build` and `cargo build --release` are sometimes not enough. We need to copy some files, to prepare some environment. It is nice to have `all` the tasks in one place with a sort order that new users can easily follow.  
It is a Rust project, so you don't have to learn another strange language for automation.  
This helper project is used in combination with the program `cargo-auto`. Install it with `cargo install cargo-auto`.
You can use also the cargo bash completion program `cargo install dev_bestia_cargo_completion`.  

Don't put any secrets like passwords, passphrases or tokens here, because this helper-project is pushed to the remote repository together with the main Rust project.  

In the main  project folder (where the Cargo.toml file is) run

```bash
cargo auto
```

You will get the list of possible tasks with descriptions like this:

```bash
User defined tasks in automation_tasks_rs:
cargo auto build - builds the crate in debug mode, fmt, increment version
cargo auto release - builds the crate in release mode, fmt, increment version
cargo auto doc - builds the docs, copy to docs directory
cargo auto test - runs all the tests
cargo auto commit_and_push "message" - commits with message and push with mandatory message
      (If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git.)
cargo auto publish_to_crates_io - publish to crates.io, git tag
      (YOu need to save the credentials before publishing. On crates.io get the 'access token'. Then save it locally with the command `cargo login TOKEN`)
```

## user defined tasks

You can write any task you need. You have all the power of the Rust language under your fingertips.  
You can use or write a library for some specific tasks you need.  
For example there is the crate `cargo_auto_github_lib` if you need to create a Release on Github.  
