// examples/example_1.rs

//! A simple example how to use the `lib`

use bestia_dev_cargo_auto_new_cli::*;

/// example how to use format_hello_phrase() and format_upper_hello_phrase()
fn main() {
    let my_name = "john doe";
    let phrase = format_hello_phrase(my_name);
    println!("{}", phrase);

    // possible error must be processed
    match format_upper_hello_phrase(my_name) {
        Ok(phrase) => println!("{}", phrase),
        Err(err) => log::error!("Error: {}", err),
    }
}
