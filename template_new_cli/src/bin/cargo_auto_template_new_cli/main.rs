//! src/bin/cargo_auto_template_new_cli/main.rs

// This `main.rs` is the code for the CLI application.
// The build of this project will create the CLI application.
// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`
// This `lib.rs` can be used as dependency crate for other projects.

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.

// Linux terminal colors
use cargo_auto_template_new_cli::{GREEN, RED, RESET, YELLOW};

/// entry point into the bin-executable
fn main() {
    // logging is essential for every project
    pretty_env_logger::init();

    // super simple argument parsing. There are crates that can parse more complex arguments.
    match std::env::args().nth(1).as_deref() {
        None | Some("--help") | Some("-h") => print_help(),
        Some("print") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(greet_name) => {
                print_greet_name(greet_name);
            }
            None => println!("{RED}Error: Missing arguments `greet_name`.{RESET}"),
        },
        Some("upper") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(greet_name) => {
                // this can return an error. Here is the last place I can deal with the error.
                match upper_greet_name(greet_name) {
                    // do nothing
                    Ok(()) => (),
                    // log error from anyhow
                    Err(err) => println!("{RED}Error: {err}{RESET}"),
                }
            }
            None => println!("{RED}Error: Missing arguments `greet_name`.{RESET}"),
        },
        _ => println!("{RED}Error: Unrecognized arguments. Try `cargo_auto_template_new_cli --help`{RESET}"),
    }
}

/// print help
fn print_help() {
    println!(
        r#"
    {YELLOW}Welcome to cargo_auto_template_new_cli !
    This is a simple yet complete template for a CLI program written in Rust.{RESET}

{GREEN}cargo_auto_template_new_cli --help{RESET}
{GREEN}cargo_auto_template_new_cli print world{RESET}
{GREEN}cargo_auto_template_new_cli upper world{RESET}

    {YELLOW}This command should return an error:{RESET}
{GREEN}cargo_auto_template_new_cli upper WORLD{RESET}
  
    {YELLOW}© 2024 bestia.dev  MIT License github.com/bestia-dev/cargo-auto{RESET}
"#
    );
}

/// print my name
fn print_greet_name(greet_name: &str) {
    // call the function from the `lib.rs`
    println!("{}", cargo_auto_template_new_cli::format_hello_phrase(greet_name));
}

/// print my name upper, can return error
fn upper_greet_name(greet_name: &str) -> anyhow::Result<()> {
    // the function from `lib.rs`, can return error
    // use the ? syntax to bubble the error up one level or continue (early return)
    let upper = cargo_auto_template_new_cli::format_upper_hello_phrase(greet_name)?;
    println!("{}", upper);
    // return
    Ok(())
}
