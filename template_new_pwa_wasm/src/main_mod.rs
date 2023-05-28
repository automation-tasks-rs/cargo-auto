// src/main_mod.rs
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
    // http://localhost:4000/cargo_auto_template_new_pwa_wasm#arg_1/arg_2
    let location = wsm::window().location();
    let mut location_hash_fragment = unwrap!(location.hash());
    // dbg! is now writing to the console, crate wasm-rs-dbg
    dbg!(&location_hash_fragment);

    // in std::env::args() the nth(0) is the exe name. Let's make it similar.
    if !location_hash_fragment.is_empty() {
        // replace # with delimiter /
        location_hash_fragment.replace_range(..1, "/");
    }
    let location_hash_fragment = format!("cargo_auto_template_new_pwa_wasm{}", location_hash_fragment);
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
                None => html_println("Error: Missing second argument for print."),
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
                        Err(err) => html_println(&format!("Error: {err}")),
                    }
                }
                None => html_println("Error: Missing second argument for upper."),
            }
        }
        _ => html_println("Error: Unrecognized arguments. Try \n http://localhost:4000/cargo_auto_template_new_pwa_wasm#help"),
    }
}

/// it 'prints' inside a dedicated element in html
fn html_println(text: &str) {
    wsm::set_html_element_inner_text("p_for_html_println", text);
}

/// print help
fn print_help() {
    html_println(
        r#"
    Welcome to cargo_auto_template_new_pwa_wasm !
    This is a simple yet complete template for a WASM program written in Rust.
    The file structure is on purpose similar to a Rust CLI project and accepts similar arguments.

    http://localhost:4000/cargo_auto_template_new_pwa_wasm
    http://localhost:4000/cargo_auto_template_new_pwa_wasm#help
    http://localhost:4000/cargo_auto_template_new_pwa_wasm#print/world
    http://localhost:4000/cargo_auto_template_new_pwa_wasm#upper/world

    This command should return an error:
    http://localhost:4000/cargo_auto_template_new_pwa_wasm#upper/WORLD

    © 2023 bestia.dev  MIT License github.com/bestia-dev/cargo-auto
"#,
    );
}

/// render first page
pub fn page_with_inputs() {
    // rust has `Raw string literals` that are great!
    // just add r# before the starting double quotes and # after the ending double quotes.
    let html = r#"
<h1>Template_new_pwa_wasm</h1>
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

    let div_for_wasm_html_injecting = wsm::get_element_by_id("div_for_wasm_html_injecting");
    div_for_wasm_html_injecting.set_inner_html(&html);
    wsm::add_listener_to_button("btn_run", &on_click_btn_run);
}

/// the listener calls this function
fn on_click_btn_run() {
    let arg_1 = wsm::get_input_element_value_string_by_id("arg_1");
    let arg_2 = wsm::get_input_element_value_string_by_id("arg_2");
    if !arg_1.is_empty() && !arg_2.is_empty() {
        // pass arguments as URL in a new tab
        let url = format!("/cargo_auto_template_new_pwa_wasm#{arg_1}/{arg_2}");
        wsm::open_url_in_new_tab(&url);
    } else {
        // write on the same web page
        wsm::set_html_element_inner_text(
            "p_for_html_println",
            &format!("Error: Both arguments are mandatory."),
        );
    }
}

// remove downloading message
fn remove_downloading_message() {
    let div_for_wasm_html_injecting = wsm::get_element_by_id("div_for_wasm_html_injecting");
    div_for_wasm_html_injecting.set_inner_html("");
}

/// print my name
fn print_greet_name(greet_name: &str) {
    let div_for_wasm_html_injecting = wsm::get_element_by_id("div_for_wasm_html_injecting");
    div_for_wasm_html_injecting.set_inner_html("<h1>The result is</h1>");
    // call the function from the `lib.rs`
    html_println(&format!("{}", lib_mod::format_hello_phrase(greet_name)));
}

/// print my name upper, can return error
fn upper_greet_name(greet_name: &str) -> anyhow::Result<()> {
    let div_for_wasm_html_injecting = wsm::get_element_by_id("div_for_wasm_html_injecting");
    div_for_wasm_html_injecting.set_inner_html("<h1>The result is</h1>");
    // the function from `lib.rs`, can return error
    // use the ? syntax to bubble the error up one level or continue (early return)
    let upper = lib_mod::format_upper_hello_phrase(greet_name)?;
    html_println(&format!("{}", upper));
    // return
    Ok(())
}
