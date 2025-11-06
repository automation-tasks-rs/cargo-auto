// generic_functions.rs

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

/// Initialize tracing to file logs/cargo_auto.log.  \
///
/// The folder logs/ is in .gitignore and will not be committed.  
pub fn tracing_init() -> anyhow::Result<()> {
    let offset = time::UtcOffset::current_local_offset()?;
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(
        offset,
        time::macros::format_description!("[hour]:[minute]:[second].[subsecond digits:6]"),
    );

    // A filter consists of one or more comma-separated directives
    // target[span{field=value}]=level
    // Levels order: 1. ERROR, 2. WARN, 3. INFO, 4. DEBUG, 5. TRACE
    // ERROR level is always logged.
    // Add filters to CARGO_AUTO_LOG environment variable for a single execution:
    // ```bash
    // CARGO_AUTO_LOG="debug,hyper_util=info,reqwest=info" ./{package_name}
    // ```
    let filter = tracing_subscriber::EnvFilter::from_env("CARGO_AUTO_LOG");

    let builder = tracing_subscriber::fmt()
        .with_file(true)
        .with_timer(timer)
        .with_line_number(true)
        .with_ansi(false)
        .with_env_filter(filter);
    if std::env::var("CARGO_AUTO_LOG").is_ok() {
        // if CARGO_AUTO_LOG exists than enable tracing to file
        let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
            .rotation(tracing_appender::rolling::Rotation::DAILY)
            .filename_prefix("cargo_auto")
            .filename_suffix("log")
            .build("logs")
            .expect("initializing rolling file appender failed");
        builder.with_writer(file_appender).init();
    } else {
        builder.init();
    };

    Ok(())
}
