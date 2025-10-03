# automation_tasks_rs

**Automation tasks coded in Rust language for the workflow of Rust projects**  
***version: 1.0.0 date: 2024-05-04 author: [Bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/automation-tasks-rs/cargo_auto_template_new_auto_for_cli)***

 ![maintained](https://img.shields.io/badge/maintained-green)
 ![ready-for-use](https://img.shields.io/badge/ready_for_use-green)
 ![rustlang](https://img.shields.io/badge/rustlang-orange)
 ![automation](https://img.shields.io/badge/automation-orange)
 ![workflow](https://img.shields.io/badge/workflow-orange)
 ![License](https://img.shields.io/badge/license-MIT-blue.svg)

 ![logo](https://raw.githubusercontent.com/automation-tasks-rs/cargo-auto/main/images/logo/logo_cargo_auto.svg)  
This is part of the [automation_tasks_rs](https://github.com/automation-tasks-rs) project

## The sub-project automation_tasks_rs

In this sub-project `automation_tasks_rs`, you can write tasks that you need when compiling or managing your Rust project.  
The simple `cargo build` and `cargo build --release` are sometimes not enough. We need to copy some files, to prepare some environment. It is nice to have `all` the tasks in one place with a sort order that new users can easily follow.  
It is a Rust project, so you don't have to learn another strange language for automation. You can develop this sub-project in VSCode:

```bash
code automation-tasks-rs
```

Don't put any secrets like passwords, passphrases, or tokens here, because this helper project is pushed to the remote repository together with the main Rust project.  

## cargo-auto

This helper project is used in combination with the program `cargo-auto`. Install it with:  

```bash
cargo install cargo-auto
```

You can use also the cargo bash completion program:

```bash
cargo install dev_bestia_cargo_completion
```

In the main  project folder (where the Cargo.toml file is) run

```bash
cargo auto
```

You will get a list of possible tasks with descriptions like `cargo auto build`, `cargo auto release`, `cargo auto doc`,...

## Already prepared tasks
  
I prepared some tasks that I often use. I use my own library crate for often-used functions `cargo_auto_lib`. You can use this tasks or you can modify them or you can completely delete them.

You can write any task you need. You have all the power of the Rust language under your fingertips. You can use or write a library crate for some specific tasks you need.  

## Secrets

Sometimes we need secrets in the development workflow. For example the secret access token for GitHub or crates.io. We must be careful with secrets.

I use my knowledge of SSH private keys to store secrets in my ~/.ssh folder. They are protected by my private key, that is protected by a passphrase that is easy to remember. This secrets never leave the source code inside automation-tasks-rs. They never go to an obscure library crate that can turn malevolent from one update to the other.

In the file `cargo_auto_config.json` there are the names of the private keys you can use in automation-tasks-rs.

## ssh-agent

Typing the passphrase every time you user the private key is very secure, but it has to be repeated too much times. I use the ssh-agent to store my private keys for one hour. This is less secure, but much more comfortable. ssh-agent works well in Linux and in Windows git-bash that is installed with git-for-windows.

## GitHub api

Many things in GitHub can be achieve only through GitHub api. This needs the secret token. GitHub uses OAuth2 to get the token. This is all prepared and working.

The prepared automation-tasks-rs can create a new git repository and a remote GitHub repository, split docs to a separate commit for clarity, change GitHub topics from Cargo.toml keywords, create a release and upload the executables.

## crates.io

To publish to crates.io we need the secret access token. The prepared tasks will store it in ~/.ssh protected by your private key.

## Error handling thiserror and anyhow

Rule number one is never to use `.unwrap()` in your real Rust code. It is a sign, you are not Error handling properly.
Maybe `unwrap()` can be fine for some fast learning examples, but for any real-life Rust code, you must use some `Error handling`. There are many different ways to do that in Rust. I choose the pair of libraries `thiserror` and `anyhow`. The first is made for libraries, the second is made for bin-executables.  
The library needs an Enum with all the possible errors that this library can return. With `#[derive(Error)]` this enum gets everything needed to be a true Rust error struct. Every error can have a formatting string and a struct of data.  
The bin-executable does not want to be involved in every possible error separately. It needs an umbrella for all possible errors with `anyhow::Result`.  
Inside the code, mostly propagate the errors with the `?` Operator after the `Result` value instead of unwrap() or the match expression.
In the tests we don't want to work with Error handling. There, instead of `.unwrap()`, use the similar function `.expect(&str)` that has an additional description string. I use expect() when I am 100% sure the panic cannot happen because I checked some conditions before it.  

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
