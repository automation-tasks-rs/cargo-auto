// examples/example_1.rs

//! A simple example how to use the `lib.rs`
//! You can run it with `cargo run --example example_1`

use cargo_auto_template_new_cli_lib::*;

/// example how to use format_hello_phrase() and format_upper_hello_phrase()
fn main() {
    let greet_name = "world";
    let phrase = format_hello_phrase(greet_name);
    println!("{}", phrase);

    // possible error must be processed
    match format_upper_hello_phrase(greet_name) {
        Ok(phrase) => println!("{}", phrase),
        Err(err) => log::error!("Error: {}", err),
    }
}
