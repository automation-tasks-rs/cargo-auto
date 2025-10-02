// bin_cli_functions.rs

//! Functions to work with CLI binary executable projects.
//!
//! Binary executables need some standard functions to help to develop them efficiently.

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
pub fn tracing_init() -> anyhow::Result<()> {
    // uncomment this line to enable tracing to file
    // let file_appender = tracing_appender::rolling::daily("tmp/logs", "cargo_auto.log");

    let offset = time::UtcOffset::current_local_offset()?;
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
        .add_directive("hyper_util=error".parse()?)
        .add_directive("reqwest=error".parse()?);

    tracing_subscriber::fmt()
        .with_file(true)
        .with_timer(timer)
        .with_line_number(true)
        .with_ansi(true)
        // .with_writer(file_appender)
        .with_env_filter(filter)
        .init();
    Ok(())
}
