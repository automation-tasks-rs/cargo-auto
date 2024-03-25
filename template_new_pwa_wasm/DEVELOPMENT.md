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

## HTML, CSS

The simple static HTML and CSS files are in `web_server_folder/cargo_auto_template_new_pwa_wasm`.  
Then the Rust code injects html elements into the DOM.  

## Web server and wasm

The browser security does not allow the loading of WASM modules from local files. It needs to be loaded from a web server. The CRUSTDE container has the [basic-http-server](https://github.com/brson/basic-http-server) already installed.  

Run the server in a second VSCode terminal, so it can keep running all the time.  

```bash
basic-http-server -a 0.0.0.0:4000 ./web_server_folder
```

In the first VSCode terminal, we can build the project.  
Then in the browser, we can refresh the page <http://localhost:4000/cargo_auto_template_new_pwa_wasm> with F5 to see the changes.  

## Rust and wasm

In the `Cargo.toml` it is important to define the output as wasm library and the required dependencies to web-sys, js-sys, and wasm-bindgen.  
Wasm starts from the `src/lib.rs`. On purpose, I added the `main_mod.rs` and `lib_mod.rs` to make the project structure similar to a Rust CLI project.  
The User Interface UI is completely different in-browser than in a CLI, but we can reuse the libraries if they are UI agnostic.  
It is smart to split a project so that the logic does not contain anything about the UI.

## GitHub

This template contains GitHub actions to build the project on commit and publish the documentation on GutHub pages.  
