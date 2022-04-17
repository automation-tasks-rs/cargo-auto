//! bestia_dev_cargo_auto_new_cli/src/bin/bestia_dev_cargo_auto_new_cli.rs

// The `bin` has all the stdin and stdout.
// The `lib` must be in/out agnostic. That is the responsibility of the `bin`

// The `bin` uses the `anyhow` error library, the `lib` uses the `thiserror` library

/// entry point into the bin executable
fn main() {
    // logging is essential for every project
    pretty_env_logger::init();

    // super simple argument parsing. There are crates that can parse complex arguments.
    match std::env::args().nth(1).as_deref() {
        None | Some("--help") | Some("-h") => print_help(),
        Some("print") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(my_name) => {
                print_my_name(my_name);
            }
            None => println!("Missing arguments `my_name`."),
        },
        Some("upper") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(my_name) => {
                // this can return an error. Here is the last place I can deal with the error.
                match upper_my_name(my_name) {
                    // do nothing
                    Ok(()) => (),
                    // log error from anyhow
                    Err(err) => log::error!("{}", err),
                }
            }
            None => println!("Missing arguments `my_name`."),
        },
        _ => println!("Unrecognized arguments. Try `bestia_dev_cargo_auto_new_cli --help`"),
    }
}

/// print help
fn print_help() {
    println!(
        r#"
bestia_dev_cargo_auto_new_cli --help
bestia_dev_cargo_auto_new_cli print my_name
bestia_dev_cargo_auto_new_cli upper my_name
"#
    );
}

/// print my name
fn print_my_name(my_name: &str) {
    // call the function from the `lib`
    println!("{}", bestia_dev_cargo_auto_new_cli::format_hello_phrase(my_name));
}

/// print my name upper, can return error
fn upper_my_name(my_name: &str) -> anyhow::Result<()> {
    // the function from `lib`, can return error
    // use the ? syntax to bubble the error up one level or continue (early return)
    let upper = bestia_dev_cargo_auto_new_cli::format_upper_hello_phrase(my_name)?;
    println!("{}", upper);
    // return
    Ok(())
}
