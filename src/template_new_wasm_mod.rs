//! this strings are copied from the template_new_wasm folder
//! because when publishing to crates.io, only the main bin-executable is transferred

use crate::{GREEN, RED, RESET, YELLOW};

pub fn new_wasm(arg_2: Option<String>) {
    match arg_2 {
        None => println!("{RED}Error: Project name argument is missing: `cargo auto new_wasm project_name`{RESET}"),
        Some(project_name) => {
            copy_to_files(&project_name);
            println!("");
            println!("    {YELLOW}The command `crate auto new_wasm` generated the directory `{project_name}`{RESET}");
            println!("    {YELLOW}You can open this new Rust project `{project_name}` in a new Rust editor.{RESET}",);
            println!("    {YELLOW}For example VSCode:{RESET}");
            println!("{GREEN}code {project_name}{RESET}");
            println!("    {YELLOW}Then build with:{RESET}");
            println!("{GREEN}cargo auto build{RESET}");
            println!("    {YELLOW}and follow the detailed instructions.{RESET}");
        }
    }
}

pub fn copy_to_files(project_name: &str) {
    let folder_path = std::path::Path::new(project_name);
    std::fs::create_dir_all(folder_path).unwrap();
    for file_item in get_vec_file() {
        // rename/replace the project_name
        let file_name = file_item
            .file_name
            .replace("cargo_auto_template_new_wasm", project_name);
        let file_content = file_item
            .file_content
            .replace("cargo_auto_template_new_wasm", project_name);

        // create directory if needed
        std::fs::create_dir_all(folder_path.join(&file_name).parent().unwrap()).unwrap();
        std::fs::write(folder_path.join(&file_name), file_content.as_bytes()).unwrap();
    }
}

pub fn get_vec_file() -> Vec<crate::FileItem> {
    let mut vec_file = vec![];

    // region: files copied into strings by automation tasks
    vec_file.push(crate::FileItem{
            file_name :"README.md",
            file_content : r###"[//]: # (auto_md_to_doc_comments segment start A)

# cargo_auto_template_new_wasm

[//]: # (auto_cargo_toml_to_md start)

**template for a minimal WASM project for browser**  
***version: 2023.519.1012 date: 2023-05-19 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/cargo_auto_template_new_wasm)***  

[//]: # (auto_cargo_toml_to_md end)

 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo_auto_template_new_wasm/blob/master/LICENSE)
 [![Rust](https://github.com/bestia-dev/cargo_auto_template_new_wasm/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/cargo_auto_template_new_wasm/)
 ![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/638168303.svg)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-262-green.svg)](https://github.com/bestia-dev/rust_wasm_pwa_minimal_clock/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-30-blue.svg)](https://github.com/bestia-dev/rust_wasm_pwa_minimal_clock/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-67-purple.svg)](https://github.com/bestia-dev/rust_wasm_pwa_minimal_clock/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/rust_wasm_pwa_minimal_clock/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-19-orange.svg)](https://github.com/bestia-dev/rust_wasm_pwa_minimal_clock/)

[//]: # (auto_lines_of_code end)

Hashtags: #rustlang #tutorial #pwa #wasm #webassembly  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## template

Just like `cargo new` makes a soft and gentle introduction to Rust projects and development, I want to make the same for an in-browser WASM project with `cargo auto new_wasm`.  
Extremely simple, just the basic moving parts and use-cases.  
This simplest template does not have a PWA implementation or dedicated web server.

## Containerized Rust Development Environment (CRDE)

I recommend using the Containerized Rust Development Environment (CRDE) to write Rust projects. It contains wasm-pack and basic-http-server that this project needs.  
<https://github.com/bestia-dev/docker_rust_development>  

## HTML, CSS

The simple static HTML and CSS files are in `web_server_folder/cargo_auto_template_new_wasm`.  
Then the Rust code injects html elements into the DOM.  

## Web server and wasm

We will need the `basic-http-server` because browser security does not allow the loading of WASM modules from local files.  
Run the server in a separate VSCode terminal, so it can keep running all the time. In the first VSCode terminal, we can build the project and in the browser, we can refresh the page with F5.  

## Rust and wasm

Cargo.toml is very important to define the output as wasm library and the required dependencies to web-sys, js-sys, and wasm-bindgen.
Wasm starts from the src/lib.rs. On purpose, I added the main_mod.rs and lib_mod.rs to make the project structure similar to a Rust CLI project. The User Interface UI is completely different in-browser or CLI, but we can reuse the libraries that are UI agnostic.  It is smart to split a project that logic does not contain UI.

We use cargo auto for automation tasks. Run:
`cargo auto build`  
and follow the detailed instructions.

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

[//]: # (auto_md_to_doc_comments segment end A)
"###,
});
    vec_file.push(crate::FileItem {
        file_name: "web_server_folder/cargo_auto_template_new_wasm/index.html",
        file_content: r###"<!DOCTYPE html>
<html lang="en">

<head>
      <!-- classic header for a web page -->
      <title>cargo_auto_template_new_wasm</title>
      <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
      <meta name="Description" content="template for a minimal wasm project for browser">
      <meta name="author" content="https://github.com/bestia-dev/cargo_auto_template_new_wasm">    
      <link rel="stylesheet" href="css/basic_style.css">
      
</head>

<body>
      <!-- warning if javascript iis not enabled -->
      <noscript>
            <h2>
                  !!!???!!!<br>
                  This web app <br>
                  cannot work <br>
                  without javascript<br>
                  enabled<br>
                  !!!???!!!</h2>
      </noscript>
      <!-- display a text while waiting for wasm download. 
      This content will change from the wasm code.-->
      <div id="div_for_wasm_html_injecting">
            <h2>
                  Waiting to<br>
                  download <br>
                  the web app...<br>
                  This is <br>
                  very quick on fast<br>
                  networks...<br>
            </h2>
      </div>
      <p class="fc_red" id="div_for_errors"></p>
      <!-- import and init the wasm code -->
      <script type="module">
            import init from "./pkg/cargo_auto_template_new_wasm.js";
            init("./pkg/cargo_auto_template_new_wasm_bg.wasm");
      </script>
</body>

</html>"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "web_server_folder/cargo_auto_template_new_wasm/css/basic_style.css",
        file_content: r###"html { 
    font-family: sans-serif;
    background-color: #000000;
    color: #FBF462;
}

h1{
    margin-left: auto;
    margin-right: auto;
    text-align: center;
}
p{
    margin-left: auto;
    margin-right: auto;
    text-align: center;
}
.small{
    font-size: 10px;
}

.input-wrap {
    position: relative;
	text-align: center;
  }

.button {
    display: inline-block;
    padding: 12px 18px;
    cursor: pointer;
    border-radius: 5px;
    background-color: #8ebf42;
    font-size: 16px;
    font-weight: bold;
    color: #fff;
  }

  .fc_red{
    color: red;
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: ".vscode/settings.json",
        file_content: r###"{
    "cSpell.words": [
        "apos",
        "bestia",
        "bindgen",
        "cdylib",
        "CRDE",
        "endregion",
        "onchange",
        "onclick",
        "plantuml",
        "rustlang",
        "thiserror",
        "webassembly"
    ]
}"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "Cargo.toml",
        file_content: r###"[package]
name = "cargo_auto_template_new_wasm"
version = "2023.519.1012"
authors = ["bestia.dev"]
homepage = "https://bestia.dev"
edition = "2021"
description = "template for a minimal wasm project for browser"
repository = "https://github.com/bestia-dev/cargo_auto_template_new_wasm"
readme = "README.md"
license = "MIT"
keywords = ["wasm"]
categories = ["learning"]
publish = false

[lib]
# cdylib is for the wasm module library
crate-type = ["cdylib"]

[dependencies]
unwrap = "1.2.1"
wasm-bindgen = "0.2.86"
console_error_panic_hook = "0.1.7"
js-sys = "0.3.63"
thiserror="1.0.40"
anyhow="1.0.71"
log = "0.4.17"
wasm-logger = "0.2.0"
wasm-rs-dbg = {version="0.1.2", default-features = false, features = ["console-log"]}

[dependencies.web-sys]
version = "0.3.63"
features = [
  "console",
  "Document",
  "Element",
  "HtmlElement",
  "HtmlInputElement",
  "Location",
  "Window",
]

[dev-dependencies]
wasm-bindgen-test = "0.3.36"

[profile.release]
panic = "abort"
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/main_mod/lib_mod.rs",
        file_content: r###"// src/lib_mod.rs
// This module is like a lib.rs module for a binary CLI executable.
// The `lib_mod.rs` must not contains any input/output interface stuff.
// So the program logic can be separate from the interface.

// The `main_mod.rs` contains all input/output interface stuff.
// This `lib_mod.rs` can then be used as dependency crate for other projects.

// The `lib_mod.rs` does not have any real code. All the code is in modules in separate files.
// The `lib_mod.rs` has just the list of modules, it publishes module's functions or class for the caller
// and it has some global stuff like the Error enum.

// The `main_mod.rs` uses the `anyhow` error library.
// The `lib_mod.rs` uses the `thiserror` library.

use thiserror::Error;

// Instead of a hello_mod local module, we could use a UI agnostic crate library dependency.
// So the same library could be used for CLI and for WASM, that have vastly different UI.
mod hello_mod;

pub mod web_sys_mod;
pub use web_sys_mod as wsm;

pub use hello_mod::format_hello_phrase;
pub use hello_mod::format_upper_hello_phrase;

/// all possible library errors for `thiserror`
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("Name `{0}` is already uppercase.")]
    Uppercase(String),
    #[error("Unknown error.")]
    Unknown,
}

// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/main_mod/lib_mod/web_sys_mod.rs",
        file_content: r###"//! src/web_sys_mod.rs
//! helper functions for web_sys, window, document, dom, console,
//! Trying to isolate/hide all javascript code and conversion here.

// region: use
// the macro unwrap! shows the TRUE location where the error has ocurred.
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
// use wasm_bindgen_futures::JsFuture;
use web_sys::console;
// use web_sys::{Request, RequestInit, Response};
// endregion: use

/// return the global window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// get element by id
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(window().document());
    unwrap!(document.get_element_by_id(element_id))
}

/// debug write into session_storage
pub fn debug_write(text: &str) {
    // writing to the console
    console::log_1(&JsValue::from_str(text));
}

/// get html element by id
pub fn get_html_element_by_id(element_id: &str) -> web_sys::HtmlElement {
    let element = get_element_by_id(element_id);
    let html_element: web_sys::HtmlElement = unwrap!(element.dyn_into::<web_sys::HtmlElement>());
    //return
    html_element
}

/// get input element value string by id
pub fn get_input_element_value_string_by_id(element_id: &str) -> String {
    // debug_write("before get_element_by_id");
    let input_element = get_element_by_id(element_id);
    // debug_write("before dyn_into");
    let input_html_element = unwrap!(input_element.dyn_into::<web_sys::HtmlInputElement>());
    // debug_write("before value()");
    input_html_element.value()
}

/// add event listener for button
pub fn add_listener_to_button(element_id: &str, fn_on_click_button: &'static (dyn Fn() + 'static)) {
    let handler_1 = Box::new(move || {
        fn_on_click_button();
    }) as Box<dyn FnMut()>;
    let closure = Closure::wrap(handler_1);

    let html_element = get_html_element_by_id(element_id);
    html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

/// set inner text
pub fn set_html_element_inner_text(element_id: &str, inner_text: &str) {
    let html_element = get_html_element_by_id(element_id);
    html_element.set_inner_text(inner_text);
}

/// WARNING for HTML INJECTION! Never put user provided strings in set_html_element_inner_html.
/// Only correctly html encoded strings can use this function.
/// set inner html into dom
pub fn set_html_element_inner_html(element_id: &str, inner_html: &str) {
    let html_element = get_element_by_id(element_id);
    html_element.set_inner_html(inner_html);
}

// open URL in new tab
pub fn open_url_in_new_tab(url: &str) {
    window().open_with_url_and_target(url, "_blank").unwrap();
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/main_mod/lib_mod/hello_mod.rs",
        file_content: r###"// src/hello_mod.rs

//! All the real code is inside modules in separate files (program logic).
//!
//! This doc-comments will be compiled into the `docs`.

/// format the hello phrase
pub fn format_hello_phrase(greet_name: &str) -> String {
    log::info!("start format_hello_phrase()");
    // return
    format!("Hello {}!", greet_name)
}

/// format the hello phrase with uppercase name
/// if it is already uppercase, return error with thiserror
pub fn format_upper_hello_phrase(greet_name: &str) -> Result<String, crate::LibraryError> {
    log::info!("start format_upper_hello_phrase()");
    // shadowing the same variable name:
    let upper_greet_name = make_uppercase(greet_name);
    if upper_greet_name == greet_name {
        return Err(crate::LibraryError::Uppercase(greet_name.to_string()));
    }

    // return
    Ok(format!("Hello {}!", &upper_greet_name))
}

/// return uppercase
pub fn make_uppercase(greet_name: &str) -> String {
    // return
    greet_name.to_uppercase()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_format_upper_hello_phrase() {
        assert_eq!(
            format_upper_hello_phrase("abcd").expect("error"),
            "Hello ABCD!"
        );
        assert!(format_upper_hello_phrase("ABCD").is_err());
    }

    #[test]
    pub fn test_make_uppercase() {
        assert_eq!(make_uppercase("abcd"), "ABCD");
        assert_eq!(make_uppercase("1234abcd"), "1234ABCD");
        assert_eq!(make_uppercase("čšž"), "ČŠŽ");
    }
}
"###,
    });
    vec_file.push(crate::FileItem{
            file_name :"src/main_mod.rs",
            file_content : r###"// src/main_mod.rs
// This module is like a main.rs module for a binary CLI executable.
// The `main_mod.rs` contains all input/output interface stuff.
// So the program logic can be separate from the interface.

// The `lib_mod.rs` must not contains any input/output interface stuff.
// This `lib_mod.rs` can then be used as dependency crate for other projects.

// The `main_mod.rs` uses the `anyhow` error library.
// The `lib_mod.rs` uses the `thiserror` library.

use unwrap::unwrap;
use wasm_rs_dbg::dbg;

mod lib_mod;
pub use lib_mod::wsm;
pub use lib_mod::LibraryError;

/// entry point just like for cli-bin-executable
pub fn main() {
    // logging is essential for every project
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("main() started");

    // super simple argument parsing.
    // In browser we can use 'local routing' on url path with # fragment
    // http://localhost:4000/cargo_auto_template_new_wasm#arg_1/arg_2
    let location = wsm::window().location();
    let mut location_hash_fragment = unwrap!(location.hash());
    // dbg! is now writing to the console, crate wasm-rs-dbg
    dbg!(&location_hash_fragment);

    // in std::env::args() the nth(0) is the exe name. Let's make it similar.
    if !location_hash_fragment.is_empty() {
        // replace # with delimiter /
        location_hash_fragment.replace_range(..1, "/");
    }
    let location_hash_fragment = format!("cargo_auto_template_new_wasm{}", location_hash_fragment);
    dbg!(&location_hash_fragment);
    let args = location_hash_fragment.split("/");
    let args: Vec<&str> = args.collect();
    dbg!(&args);

    remove_downloading_message();

    // Since &str is Copy, you can avoid the creation of &&str by adding .copied()
    match args.get(1).copied() {
        None => page_with_inputs(),
        Some("help") => print_help(),
        Some("print") => {
            match args.get(2).copied() {
                // second argument
                Some(greet_name) => print_greet_name(greet_name),
                None => wsm::set_html_element_inner_text("div_for_errors","Error: Missing second argument for print."),
            }
        }
        Some("upper") => {
            match args.get(2).copied() {
                // second argument
                Some(greet_name) => {
                    // this can return an error. Here is the last place I can deal with the error.
                    match upper_greet_name(greet_name) {
                        // do nothing
                        Ok(()) => (),
                        // log error from anyhow
                        Err(err) => wsm::set_html_element_inner_text("div_for_errors",&format!("Error: {err}")),
                    }
                }
                None => wsm::set_html_element_inner_text("div_for_errors","Error: Missing second argument for upper."),
            }
        }
        _ => wsm::set_html_element_inner_text("div_for_errors","Error: Unrecognized arguments. Try \n http://localhost:4000/cargo_auto_template_new_wasm#help"),
    }
}


/// print help
fn print_help() {
    wsm::set_html_element_inner_text("div_for_wasm_html_injecting",
        r#"
    Welcome to cargo_auto_template_new_wasm !
    
    This is a simple yet complete template for a WASM program written in Rust.
    The file structure is on purpose similar to a Rust CLI project and accepts similar arguments.

    http://localhost:4000/cargo_auto_template_new_wasm
    http://localhost:4000/cargo_auto_template_new_wasm#help
    http://localhost:4000/cargo_auto_template_new_wasm#print/world
    http://localhost:4000/cargo_auto_template_new_wasm#upper/world

    This command should return an error:
    http://localhost:4000/cargo_auto_template_new_wasm#upper/WORLD

    © 2023 bestia.dev  MIT License github.com/bestia-dev/cargo-auto
"#,
    );
}

/// render first page
pub fn page_with_inputs() {
    // rust has `Raw string literals` that are great!
    // just add r# before the starting double quotes and # after the ending double quotes.
    let html = r#"
<h1>Template_new_wasm</h1>
<p>Write a command in the Argument 1: print or upper</p>
<div class="input-wrap">
    <label for="arg_1">Argument 1:</label>  
    <input style="width:20%;" type="text" id="arg_1" value="upper"/>
</div>
<p>Write a name in the Argument 2: world or WORLD</p>
<div class="input-wrap">
    <label for="arg_2">Argument 2:</label>  
    <input style="width:20%;" type="text" id="arg_2" value="world"/>
</div>
<p>Click on Run</p>
<div class="input-wrap">
    <input type="button" class="button" id="btn_run" value="Run"/>
</div>
<p class="small">bestia.dev</p>
        "#;

    // WARNING for HTML INJECTION! Never put user provided strings in set_html_element_inner_html.
    // Only correctly html encoded strings can use this function.
    wsm::set_html_element_inner_html("div_for_wasm_html_injecting",html);
    wsm::add_listener_to_button("btn_run", &on_click_btn_run);
}

/// the listener calls this function
fn on_click_btn_run() {
    let arg_1 = wsm::get_input_element_value_string_by_id("arg_1");
    let arg_2 = wsm::get_input_element_value_string_by_id("arg_2");
    if !arg_1.is_empty() && !arg_2.is_empty() {
        // pass arguments as URL in a new tab
        let url = format!("/cargo_auto_template_new_wasm#{arg_1}/{arg_2}");
        wsm::open_url_in_new_tab(&url);
    } else {
        // write on the same web page
        wsm::set_html_element_inner_text(
            "div_for_errors",
            &format!("Error: Both arguments are mandatory."),
        );
    }
}

// remove downloading message
fn remove_downloading_message() {
    wsm::set_html_element_inner_text("div_for_wasm_html_injecting","");
}

/// print my name
fn print_greet_name(greet_name: &str) {
    wsm::set_html_element_inner_text("div_for_wasm_html_injecting",&format!(
r#"The result is
{}
"#,
    lib_mod::format_hello_phrase(greet_name)
    ));
}

/// print my name upper, can return error
fn upper_greet_name(greet_name: &str) -> anyhow::Result<()> {
    // the function from `lib.rs`, can return error
    // use the ? syntax to bubble the error up one level or continue (early return)
    let upper = lib_mod::format_upper_hello_phrase(greet_name)?;
    wsm::set_html_element_inner_text("div_for_wasm_html_injecting",&format!(
r#"The result is
{upper}
"#
    ));
    // return
    Ok(())
}
"###,
});
    vec_file.push(crate::FileItem {
        file_name: "src/lib.rs",
        file_content: r###"//! src/lib.rs
//! This file has just the wasm_bindgen_start() function
//! and calls into main_mod.rs.
//! So the structure of the project modules can be similar to a binary CLI executable.

// region: auto_md_to_doc_comments include README.md A //!
//! # cargo-auto  
//!
//! **cargo-auto - automation tasks written in Rust language for the build process of Rust projects**  
//! ***version: 2024.203.327 date: 2024-02-03 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/cargo-auto)***  
//!
//!  ![status](https://img.shields.io/badge/maintained-green)
//!  ![status](https://img.shields.io/badge/ready_for_use-green)
//!
//!  [![crates.io](https://img.shields.io/crates/v/cargo-auto.svg)](https://crates.io/crates/cargo-auto)
//!  [![Documentation](https://docs.rs/cargo-auto/badge.svg)](https://docs.rs/cargo-auto/)
//!  [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/cargo-auto.svg)](https://web.crev.dev/rust-reviews/crate/cargo-auto/)
//!  [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/cargo-auto/)
//!  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo-auto/blob/master/LICENSE)
//!  [![Rust](https://github.com/bestia-dev/cargo-auto/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/cargo-auto/)
//!  ![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/959103982.svg)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-9023-green.svg)](https://github.com/bestia-dev/cargo-auto/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-455-blue.svg)](https://github.com/bestia-dev/cargo-auto/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-336-purple.svg)](https://github.com/bestia-dev/cargo-auto/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/cargo-auto/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-1406-orange.svg)](https://github.com/bestia-dev/cargo-auto/)
//!
//! Hashtags: #rustlang #tutorial #buildtool #developmenttool #cli  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).
//!
//! ## Try it
//!
//! First, we will use `cargo-auto` to create a new empty CLI Rust project similar to `cargo new`, but with a more complete project structure.  
//!
//!  ```bash
//! cargo install cargo-auto
//! cargo auto new_cli my_hello_project
//! cd my_hello_project
//! cargo auto
//! # it lists all the prepared automation tasks
//! # try a few
//! cargo auto build
//! cargo auto release
//! cargo auto doc
//! cargo auto test
//! ```
//!
//! We can also add `automation tasks` to an existing Rust project.
//! Inside your Rust project directory (the one with Cargo.toml or Cargo-auto.toml) run:  
//!
//! ```bash
//! cargo auto new_auto
//! cargo auto
//! # it lists all the prepared automation tasks
//! # try to build
//! cargo auto build
//! ```
//!
//! Congratulations! You are already using `cargo-auto`. Simple as that.  
//! Now you can modify the tasks to your needs. It is all Rust language.  
//!
//! ## Motivation
//!
//! Cargo is a great tool for building Rust projects. It has all the basics: `cargo build`, `cargo build --release`, `cargo fmt`, `cargo test`, `cargo doc`,...  
//! But sometimes we need to do more things like copying some files, publishing to FTP, or entering long commands. These repetitive tasks must be automated.  
//! Task automation makes work easier and faster, and simplifies the workflow while improving the consistency and accuracy of workflows.  
//! This is also sometimes referred to as "workflow automation."  
//! There are many different build systems and task runners there: `make`, `cmake`, `shell scripts`, `cargo-xtask`, `cargo-make`, `cargo-task`, `cargo-script`, `cargo-run-script`, `runner`, `python scripts`, `powershell scripts`, `cmd prompt scripts`, ...  
//! Sadly there is no standard in the Rust community for now.  
//! I want something similar to [build.rs](https://doc.rust-lang.org/cargo/reference/build-scripts.html), so I can write my "tasks" in pure Rust I don't want to learn another meta language with weird syntax and difficulty to debug. So I will make something really simple, easy, rusty, and extensible.  
//!
//! ## cargo auto new_cli
//!
//! I like very much that Rust has the command `cargo new project_name`. It creates a super simple Rust Hello project that can be built and run immediately. But this example is too simple. It lacks the basic file structures of a serious CLI program.  
//! I composed an opinionated template for a Rust CLI project. It is easy to run:
//!
//! ```bash
//! cargo auto new_cli project_name
//! ```
//!
//! ## cargo auto new_wasm
//!
//! I composed an opinionated template for a simple Rust WASM project for a browser. It is very similar to the new_cli template but for WASM.  
//! It is easy to run:
//!
//! ```bash
//! cargo auto new_wasm project_name
//! # then
//! cd project_name
//! cargo auto build
//! # follow detailed instructions
//! ```
//!
//! ## cargo auto new_pwa_wasm
//!
//! I composed an opinionated template for a simple Rust PWA-WASM project for a browser. It is very similar to the new_cli template but for WASM. It adds the PWA standard functionality to work as an offline app.  
//! The template needs the title, name, long name, and description inside a `pwa.json5` file and the `icon512x512.png` file for the icons.  
//! It is easy to run:
//!
//! ```bash
//! cargo auto new_pwa_wasm
//! # on first run it will just create the `pwa.json5` and `icon512x512.png` files
//! # modify these files for your new app
//! cargo auto new_pwa_wasm
//! # then
//! cd project_name
//! cargo auto build
//! # follow detailed instructions
//! ```
//!
//! ## scripting with rust
//!
//! Rust is a compiled language. It is not really a scripting or interpreted language. But the compilation of small projects is really fast and can be ignored. Subsequent calls will use the already-built binary so the speed will be even faster.  
//! This tool `cargo-auto` is meant for Rust projects, so it means that all the Rust infrastructure is already in place.  
//!
//! ## automation_tasks_rs helper project
//!
//! The command `cargo auto new_auto` will create a new directory `automation_tasks_rs` with a template for a helper Rust project in the root directory of your `main Rust project`. It should not interfere with the main Rust project. This directory will be added to git commits and pushed to remote repositories as part of the main project. It has its own `.gitignore` to avoid committing to its target directory.  
//! The `automation_tasks_rs` helper project contains user-defined tasks in Rust code. Your tasks. This helper project should be opened in a new editor starting from the `automation_tasks_rs` directory. It does not share dependencies with the main project. It is completely separate and independent.  
//! You can edit it and add your dependencies and Rust code. No limits. Freedom of expression.  
//! This is now your code, your tasks, and your helper Rust project!  
//! Because only you know what you want to automate and how to do it.  
//! Never write secrets, passwords, passcodes, or tokens inside your Rust code. Because then it is pushed to GitHub and the whole world can read it in the next second!
//! Basic example (most of the useful functions are already there):  
//!
//! ```rust
//! /// match arguments and call tasks functions
//! fn match_arguments_and_call_tasks(mut args: std::env::Args){
//!     // the first argument is the user defined task: (no argument for help), build, release,...
//!     let arg_1 = args.next();
//!     match arg_1 {
//!         None => print_help(),
//!         Some(task) => {            
//!             println!("Running auto task: {}", &task);
//!             if &task == "build"{
//!                 task_build();
//!             } else if &task == "release" {
//!                 task_release();
//!             } else if &task == "doc" {
//!                 task_doc();
//!             } else {
//!                 println!("Task {} is unknown.", &task);
//!                 print_help();
//!             }
//!         }
//!     }
//! }
//!
//! /// write a comprehensible help for user defined tasks
//! fn print_help() {
//!     println!(r#"
//!     User defined tasks in automation_tasks_rs:
//! cargo auto build - builds the crate in debug mode
//! cargo auto release - builds the crate in release mode
//! cargo auto docs - builds the docs
//! "#);
//! }
//!
//! // region: tasks
//!
//! /// cargo build
//! fn task_build() {
//!     run_shell_command("cargo fmt");
//!     run_shell_command("cargo build");
//! }
//!
//! /// cargo build --release
//! fn task_release() {
//!     run_shell_command("cargo fmt");
//!     run_shell_command("cargo build --release");
//! }
//!
//! /// cargo doc, then copies to /docs/ folder, because this is a github standard folder
//! fn task_doc() {
//!     run_shell_command("cargo doc --no-deps --document-private-items");
//!     // copy target/doc into docs/ because it is github standard
//!     run_shell_command("rsync -a --info=progress2 --delete-after target/doc/ docs/");
//!     // Create simple index.html file in docs directory
//!     run_shell_command(&format!(
//!         "echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",
//!         cargo_toml.package_name().replace("-","_")
//!     ));
//!     run_shell_command("cargo fmt");
//! }
//!
//! // endregion: tasks
//!
//! ```
//!
//! ## cargo auto subcommand
//!
//! The command `cargo install cargo-auto` will add a new subcommand to cargo:
//!
//! ```bash
//! cargo auto
//! ```
//!
//! This binary is super simple. It has only 1 trivial dependency: `lazy_static`.  
//! The binary only reads the CLI arguments and runs the `automation_tasks_rs` binary with them. If needed it will compile `automation_tasks_rs` first.  
//! The code-flow of the source code of `cargo-auto` is simple, fully commented, and straightforward to audit.  
//! The source code is on [GitHub](https://github.com/bestia-dev/cargo-auto) with MIT open-source licensing.  
//!
//! ## bash auto-completion
//!
//! With the help of the crate [dev_bestia_cargo_completion](https://crates.io/crates/dev_bestia_cargo_completion), the commands `cargo` and `cargo auto` get bash auto-completion. Try it!  
//!
//! ## cargo auto new_auto
//!
//! Inside the cargo-auto project, there is a Rust sub-project that is a template. I can open a new editor for these directories and build this crate independently. So it is easy to debug and develop.  
//! Sadly, I cannot publish these directories and files to `crates.io`. I can effectively publish only the source code inside my main Rust project `cargo-auto`.  
//! Therefore, before publishing I copy the content of these files into the modules `template_new_auto_mod.rs` on every build. It is not difficult now that Rust has fantastic [raw strings](https://doc.rust-lang.org/rust-by-example/std/str.html).  
//!
//! ## more complex tasks
//!
//! You can write more complex tasks in Rust language.  
//! For example in this project I use automation to create GitHub Releases: <https://github.com/bestia-dev/dropbox_backup_to_external_disk>  
//! Here is a pretty complex workspace with more sub-projects:  
//! <https://github.com/bestia-dev/cargo_crev_reviews_workspace>  
//! There is no end to your imagination. If you write something that looks like it can help other developers, please share it with me and I will add it here.
//!
//! ## development
//!
//! Usually, I compile and run the code of `cargo-auto` with added arguments like this:  
//!
//! ```bash
//! cargo run -- new_auto
//! cargo run -- build
//! cargo run -- release
//! ```
//!
//! ## TODO
//!
//! new wasm, new wasm_pwa, new wasm_pwa_server, new wasm_pwa_server_pgrsql
//!
//! ## cargo crev reviews and advisory
//!
//! We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).  
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
//! to verify the trustworthiness of each of your dependencies.  
//! Please, spread this info.  
//! You can also read reviews quickly on the web:  
//! <https://web.crev.dev/rust-reviews/crates/>  
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

use wasm_bindgen::prelude::*;

mod main_mod;
pub use main_mod::wsm;
pub use main_mod::LibraryError;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    wsm::debug_write(&format!(
        "cargo_auto_template_new_wasm v{}",
        env!("CARGO_PKG_VERSION")
    ));

    main_mod::main();
    // return
    Ok(())
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: ".gitignore",
        file_content: r###"# Generated by Cargo
# will have compiled files and executables
/target/

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
# Cargo.lock

# These are backup files generated by rustfmt
**/*.rs.bk

# result of compilation does not need to go to repository
/pkg/
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/Cargo.toml",
        file_content: r###"
[package]
name = "automation_tasks_rs"
version = "0.1.1"
authors = ["bestia.dev"]
homepage = "https://bestia.dev"
edition = "2021"
description = "cargo auto - automation tasks written in Rust language"
publish = false

[dependencies]
cargo_auto_lib = "1.0.78""###,
    });
    vec_file.push(crate::FileItem{
            file_name :"automation_tasks_rs/src/main.rs",
            file_content : r###"//! automation_tasks_rs for cargo_auto_template_new_wasm

use cargo_auto_lib::*;

// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";


fn main() {
    exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("{YELLOW}Running automation task: {task}{RESET}");
                if &task == "build" {
                    task_build();
                } else if &task == "release" {
                    task_release();
                } else if &task == "doc" {
                    task_doc();
                } else if &task == "test" {
                    task_test();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else if &task == "publish_to_web" {
                    task_publish_to_web();                    
                } else {
                    println!("{RED}Error: Task {task} is unknown.{RESET}");
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
    {YELLOW}Welcome to cargo-auto !
    This program automates your custom tasks when developing a Rust project.{RESET}

    {YELLOW}User defined tasks in automation_tasks_rs:{RESET}
{GREEN}cargo auto build{RESET}{YELLOW} - builds the crate in debug mode, fmt, increment version{RESET}
{GREEN}cargo auto release{RESET}{YELLOW} - builds the crate in release mode, fmt, increment version{RESET}
{GREEN}cargo auto doc{RESET}{YELLOW} - builds the docs, copy to docs directory{RESET}
{GREEN}cargo auto test{RESET}{YELLOW} - runs all the tests{RESET}
{GREEN}cargo auto commit_and_push "message"{RESET}{YELLOW} - commits with message and push with mandatory message{RESET}
    {YELLOW}(If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git.){RESET}
{GREEN}cargo auto publish_to_web - publish to web, git tag{RESET}
    {YELLOW}(You need credentials for publishing over SSH. Use ssh-agent and ssh-add to store the credentials for SSH.){RESET}

    {YELLOW}© 2023 bestia.dev  MIT License github.com/bestia-dev/cargo-auto{RESET}
"#
    );
    print_examples_cmd();
}

/// all example commands in one place
fn print_examples_cmd(){
/*
    println!(r#"{YELLOW}run examples:{RESET}{GREEN}
cargo run --example example1{RESET}
"#);
*/
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "release", "doc", "test", "commit_and_push", "publish_to_web"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// wasm-pack build
fn task_build() {
    auto_version_increment_semver_or_date();
    run_shell_command("cargo fmt");
    auto_cargo_toml_to_md();
    auto_lines_of_code("");
    run_shell_command("wasm-pack build --target web");
    run_shell_command("\\rsync -a --delete-after pkg/ web_server_folder/cargo_auto_template_new_wasm/pkg/");
    println!(
        r#"
    {YELLOW}After `cargo auto build`, open port 4000 in VSCode and run the basic web server
    in a separate VSCode bash terminal, so it can serve constantly in the background.{RESET}
{GREEN}basic-http-server -a 0.0.0.0:4000 ./web_server_folder{RESET}
    {YELLOW}and open the browser on{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#print/world{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#upper/world{RESET}
    {YELLOW}This will return an error:{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#upper/WORLD{RESET}
    {YELLOW}If all is fine, run{RESET}
{GREEN}cargo auto release{RESET}
"#
    );
}

/// wasm-pack build --release
fn task_release() {
    auto_version_increment_semver_or_date();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");

    run_shell_command("cargo fmt");
    run_shell_command("wasm-pack build --target web");
    run_shell_command("\\rsync -a --delete-after pkg/ web_server_folder/cargo_auto_template_new_wasm/pkg/");
    println!(
        r#"
    {YELLOW}After `cargo auto build`, open port 4000 in VSCode and run the basic web server
    in a separate VSCode bash terminal, so it can serve constantly in the background.{RESET}
{GREEN}basic-http-server -a 0.0.0.0:4000 ./web_server_folder{RESET}
    {YELLOW}and open the browser on{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm{RESET}    
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#print/world{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#upper/world{RESET}
    {YELLOW}This will return an error:{RESET}
{GREEN}http://localhost:4000/cargo_auto_template_new_wasm#upper/WORLD{RESET}
    {YELLOW}If all is fine, run{RESET}
{GREEN}cargo auto doc{RESET}
"#
    );
    print_examples_cmd();
}

/// cargo doc, then copies to /docs/ folder, because this is a github standard folder
fn task_doc() {
    let cargo_toml = CargoToml::read();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");
    auto_plantuml(&cargo_toml.package_repository().unwrap());
    auto_md_to_doc_comments();

    run_shell_command("cargo doc --no-deps --document-private-items");
    // copy target/doc into docs/ because it is github standard
    run_shell_command("rsync -a --info=progress2 --delete-after target/doc/ docs/");
    // Create simple index.html file in docs directory
    run_shell_command(&format!(
        "echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",
        cargo_toml.package_name().replace("-","_")
    ));
    run_shell_command("cargo fmt");
    // message to help user with next move
    println!(
        r#"
    {YELLOW}After `cargo auto doc`, check `docs/index.html`. If ok, then test the documentation code examples{RESET}
{GREEN}cargo auto test{RESET}
    {YELLOW}{RESET}"#
    );
}

/// cargo test
fn task_test() {
    run_shell_command("cargo test");
    println!(
        r#"
    {YELLOW}After `cargo auto test`. If ok, then {RESET}
{GREEN}cargo auto commit_and_push "message"{RESET}
    {YELLOW}with mandatory commit message{RESET}
{GREEN}{RESET}"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    match arg_2 {
        None => println!("{RED}Error: Message for commit is mandatory.{RESET}"),
        Some(message) => {
            run_shell_command(&format!(r#"git add -A && git commit --allow-empty -m "{}""#, message));
            run_shell_command("git push");            
            println!(
                r#"
    {YELLOW}After `cargo auto commit_and_push "message"`{RESET}
{GREEN}cargo auto publish_to_crates_io{RESET}
    {YELLOW}{RESET}"#
                );
        }
    }
}

/// publish to web
fn task_publish_to_web() {
    println!(r#"{YELLOW}Use ssh-agent and ssh-add to store your credentials for publish to web.{RESET}"#);
    let cargo_toml = CargoToml::read();
    // git tag
    let shell_command = format!(
        "git tag -f -a v{version} -m version_{version}",
        version = cargo_toml.package_version()
    );
    run_shell_command(&shell_command);
    let shell_command = format!(
        "rsync -e ssh -a --info=progress2 --delete-after ~/rustprojects/{package_name}/web_server_folder/ project_author@project_homepage:/var/www/project_homepage/pwa_short_name/",
        package_name = cargo_toml.package_name()
    );
    run_shell_command(&shell_command);
    println!(
        r#"{YELLOW}
    After `cargo auto publish_to_web`, 
    check 
https://bestia.dev/{package_name}
{RESET}"#,
        package_name = cargo_toml.package_name()
    );
}

// endregion: tasks
"###,
});
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/.gitignore",
        file_content: r###"/target
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "LICENSE",
        file_content: r###"MIT License

Copyright (c) 2023 bestia.dev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"###,
    });
    // endregion: files copied into strings by automation tasks

    // return
    vec_file
}
