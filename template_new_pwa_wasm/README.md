[//]: # (auto_md_to_doc_comments segment start A)

# rust_project_name

[//]: # (auto_cargo_toml_to_md start)

**template for a minimal pwa wasm project for browser**  
***version: 2023.519.1012 date: 2023-05-19 author: [project_author](project_homepage) repository: [Github](project_repository)***  

[//]: # (auto_cargo_toml_to_md end)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-262-green.svg)](https://github.com/bestia-dev/rust_wasm_pwa_minimal_clock/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-30-blue.svg)](https://github.com/bestia-dev/rust_wasm_pwa_minimal_clock/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-67-purple.svg)](https://github.com/bestia-dev/rust_wasm_pwa_minimal_clock/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/rust_wasm_pwa_minimal_clock/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-19-orange.svg)](https://github.com/bestia-dev/rust_wasm_pwa_minimal_clock/)

[//]: # (auto_lines_of_code end)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](project_repository/blob/master/LICENSE)
[![Rust](project_repository/workflows/RustAction/badge.svg)](project_repository)

Hashtags: #rustlang #tutorial #pwa #wasm #webassembly  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## template

Just like `cargo new` makes a soft and gentle introduction to Rust projects and development, I want to make the same for an in-browser WASM project with `cargo auto new_pwa_wasm`.  
Extremely simple, just the basic moving parts and use-cases.  
This simplest template does not have a PWA implementation or dedicated web server.

## Containerized Rust Development Environment (CRDE)

I recommend to use the Containerized Rust Development Environment (CRDE) to write Rust projects. It contains wasm-pack and basic-http-server that this project needs.  
<https://github.com/bestia-dev/docker_rust_development>  

## HTML, CSS

The simple static HTML and CSS files are in `web_server_folder/pwa_short_name`.  
Then the Rust code injects html elements into the DOM.  

## Web server and wasm

We will need the `basic-http-server` because browser security does not allow loading wasm modules from local files.  
Run the server in a separate VSCode terminal, so it can keep running all the time. In the first VSCode terminal we can build the project and in the browser we can refresh the page with F5.  

## Rust and wasm

Cargo.toml is very important to define the output as wasm library and the required dependencies to web-sys, js-sys and wasm-bindgen.
Wasm starts from the src/lib.rs. On purpose I added the main_mod.rs and lib_mod.rs to make the project structure similar to a Rust CLI project. The User Interface UI is completely different in-browser or CLI, but we can reuse the libraries that are UI agnostic.  It is smart to split a project that logic does not contain UI.

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
