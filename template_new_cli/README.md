[//]: # (auto_md_to_doc_comments segment start A)

# cargo_auto_template_new_cli

[//]: # (auto_cargo_toml_to_md start)

**Basic Rust project template for CLI and library, more than just `cargo new hello`**  
***version: 1.0.4 date: 2022-04-21 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/cargo_auto_template_new_cli)***  

[//]: # (auto_cargo_toml_to_md end)

 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo_auto_template_new_cli/blob/main/LICENSE)
 [![Rust](https://github.com/bestia-dev/cargo_auto_template_new_cli/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/cargo_auto_template_new_cli/)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-89-green.svg)](https://github.com/bestia-dev/cargo-auto/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-13-blue.svg)](https://github.com/bestia-dev/cargo-auto/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-36-purple.svg)](https://github.com/bestia-dev/cargo-auto/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-19-yellow.svg)](https://github.com/bestia-dev/cargo-auto/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-30-orange.svg)](https://github.com/bestia-dev/cargo-auto/)

[//]: # (auto_lines_of_code end)

## Edit this README.md file

Edit this README file with your data. But leave the markers: auto_md_to_doc_comments, auto_lines_of_code, auto_cargo_toml_to_md and similar, because the automation tasks need them.  
Modify the title and description only in Cargo.toml. Automation tasks will copy that into README.md.  
Lines of code are filled automatically from automation tasks.  
Find `bestia.dev` everywhere and change it with your username.

## Motivation

The first line I typed when I learned the Rust language was `cargo new hello`. It is extraordinary for learning Rust, but it is a rudimentary example, not really useful in practical life.

I created this project template `cargo_auto_template_new_cli` for a simple CLI application that has all the moving parts for a real-life project.

## Separate main.rs and lib.rs

It is always good to split the project between a `main.rs` (executable) and a `lib.rs` (library crate).

Even for the smallest project. Maybe some other program will use the library eventually.

All the input/output is coded in the `main.rs`: keyboard and monitor (stdin and stdout), access to files, and some access to the network.  
The library must not operate directly with the stdin/stdout, because some other caller of the library can have other ideas around input-output options. Maybe it is a Graphical user interface that does things completely different than CLI applications.

A separate `lib.rs` enables one to make good tests and examples without worrying about input-output.

## super simple argument parsing

I use a super simple code to parse CLI arguments inside the `src/bin/cargo_auto_template_new_cli/main.rs`. There are crate libraries that enable very complex argument parsing if needed.

## automation_tasks_rs

Building a project is always more complex than just `cargo build` and `cargo run`. There are always some files to copy or some content to copy from file to file. For this, I use `cargo-auto` - automation tasks written in Rust language for the build process of Rust projects.

All the sources are inside the folder `automation_tasks_rs`. It is pure Rust, it is easy to understand and modify to your needs.

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

After the task, there is a recommendation on what to do next.

```bash
cargo auto release
```

```bash
Running automation task: release
old version: "0.1.20"
new version: '0.1.21'
new text: '
**Basic Rust project template for CLI, more than just `cargo new hello`**
***version: 0.1.21 date: 2022-04-01 author: [bestia.dev](bestia.dev) repository: [GitHub](https://github.com/bestia-dev/cargo_auto_template_new_cli)***'

include_into_readme_md write file: README.md
$ cargo fmt
$ cargo build --release
Compiling cargo_auto_template_new_cli v0.1.21 (/home/rustdevuser/rustprojects/cargo_auto_template_new_cli)
Finished release [optimized] target(s) in 1.05s

After `cargo auto release`, , run the compiled binary
run `./target/release/cargo_auto_template_new_cli print world`
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
 Documenting cargo_auto_template_new_cli v0.1.21 (/home/rustdevuser/rustprojects/cargo_auto_template_new_cli)
Finished dev [unoptimized + debuginfo] target(s) in 0.54s
$ rsync -a --info=progress2 --delete-after target/doc/ docs/
2,787,371 100% 46.60MB/s 0:00:00 (xfr#56, to-chk=0/61) 

After `cargo auto doc`, check `docs/index.html`. If ok, then 
run `cargo auto commit_and_push` with mandatory commit message
```

If you Ctrl+Click on the link `docs/index.html` it will open the file in VSCode editor. In the right corner you can click to see the Live Preview. It will open the preview for the html file in an integrated browser in VSCode. Very useful.
Now is a good time to run all the tests before committing.

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
To https://github.com/bestia-dev/cargo_auto_template_new_cli.git
 d0f31d3..3bdcc91 main -> main

After `cargo auto commit and push`
run `cargo auto publish_to_crates_io`
```

And finally, if you want to publish it on crates.io. First, you need the `access token` you get from crates.io.

```bash
cargo login
# type the access token
cargo auto publish_to_crates_io
```

## lib.rs doc-comments

The entire README.md is copied into lib.rs. This can be annoying to watch. You can collapse the entire section by clicking on `// region: auto_md_to_doc_comments include README.md`.

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

Rule number one is never to use `.unwrap()` in your real Rust code. It is a sign, you are not Error handling properly.
Maybe `unwrap()` can be fine for some fast learning examples, but for any real-life Rust code, you must use some `Error handling`. There are many different ways to do that in Rust. I choose the pair of libraries `thiserror` and `anyhow`. The first is made for libraries, the second is made for bin-executables.  
The library needs an Enum with all the possible errors that this library can return. With `#[derive(Error)]` this enum gets everything needed to be a true Rust error struct. Every error can have a formatting string and a struct of data.  
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

But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer donating on my [paypal](https://paypal.me/LucianoBestia).

You know the price of a beer in your local bar ;-) So I can drink a free beer for your health :-)

[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//]: # (auto_md_to_doc_comments segment end A)
