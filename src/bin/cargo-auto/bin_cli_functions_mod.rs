// bin_cli_functions.rs

//! Functions to work with CLI binary executable projects.
//!
//! Binary executables need some standard functions to help to develop them efficiently.

use tracing::Instrument;

// region: Public API constants
// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
/// ANSI color
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
/// ANSI color
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
/// ANSI color
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
/// ANSI color
#[allow(dead_code)]
pub const BLUE: &str = "\x1b[34m";
/// ANSI color
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";
// endregion: Public API constants

/// Initialize tracing to file tmp/logs/cargo_auto.log
///
/// The folder tmp/logs/ is in .gitignore and will not be committed.
pub fn tracing_init() {
    // uncomment this line to enable tracing to file
    // let file_appender = tracing_appender::rolling::daily("tmp/logs", "cargo_auto.log");

    let offset = time::UtcOffset::current_local_offset().expect("should get local offset!");
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(
        offset,
        time::macros::format_description!("[hour]:[minute]:[second].[subsecond digits:6]"),
    );

    // Filter out logs from: hyper_util, reqwest
    // A filter consists of one or more comma-separated directives
    // target[span{field=value}]=level
    // examples: tokio::net=info
    // Levels order: ERROR, WARN, INFO, DEBUG, TRACE
    // ERROR level is always logged.
    // To add other levels use the RUST_LOG environment variable:
    // ```bash
    // export RUST_LOG=cargo_auto=warn
    // export RUST_LOG=cargo_auto=info
    // export RUST_LOG=cargo_auto=debug
    // export RUST_LOG=cargo_auto=trace
    // ```
    // Unset the environment variable RUST_LOG:
    // ```bash
    // unset RUST_LOG
    // ```
    let filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive("hyper_util=error".parse().unwrap_or_else(|e| panic!("{e}")))
        .add_directive("reqwest=error".parse().unwrap_or_else(|e| panic!("{e}")));

    tracing_subscriber::fmt()
        .with_file(true)
        .with_timer(timer)
        .with_line_number(true)
        .with_ansi(true)
        // .with_writer(file_appender)
        .with_env_filter(filter)
        .init();
}

/// The original Rust report of the panic is ugly for the end user
///
/// For panics I log the location.
/// If the message contains "Exiting..." than it is a forced "not-error exit" and the location is not important.
pub fn panic_set_hook(panic_info: &std::panic::PanicHookInfo) {
    let mut string_message = "".to_string();
    if let Some(message) = panic_info.payload().downcast_ref::<String>() {
        string_message = message.to_owned();
    }
    if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
        string_message.push_str(message);
    }

    tracing::error!("{string_message}");
    eprintln!("{string_message}");

    if !string_message.contains("Exiting...") {
        let file = panic_info.location().unwrap().file();
        let line = panic_info.location().unwrap().line();
        let column = panic_info.location().unwrap().column();
        tracing::error!("{RED}Panic location: {file}:{line}:{column}{RESET}");
        eprintln!("{RED}Location: {file}:{line}:{column}{RESET}");
    }
}
