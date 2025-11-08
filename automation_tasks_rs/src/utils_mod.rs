// utils_mod.rs

//! Helper functions that does not usually change.
//!
//! Don't change this code, so it can be updated regularly with
//! cargo auto update_automation_tasks_rs
//! If you want to customize it, copy the code into main.rs and modify it there.

use crate::cargo_auto_lib as cl;

#[allow(unused_imports)]
pub use cl::{BLUE, GREEN, RED, RESET, YELLOW};

/// Initialize tracing to file logs/automation_tasks_rs.log.  \
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
    // Add filters to AUTOMATION_TASKS_RS_LOG environment variable for a single execution:
    // ```bash
    // AUTOMATION_TASKS_RS_LOG="debug,hyper_util=info,reqwest=info" ./{package_name}
    // ```
    let filter = tracing_subscriber::EnvFilter::from_env("AUTOMATION_TASKS_RS_LOG");

    let builder = tracing_subscriber::fmt()
        .with_timer(timer)
        .with_ansi(false)
        .with_target(false)
        .with_env_filter(filter);
    if std::env::var("AUTOMATION_TASKS_RS_LOG").is_ok() {
        // if AUTOMATION_TASKS_RS_LOG exists than enable tracing to file
        let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
            .rotation(tracing_appender::rolling::Rotation::DAILY)
            .filename_prefix("automation_tasks_rs")
            .filename_suffix("log")
            .build("logs")
            .expect("initializing rolling file appender failed");
        builder.with_writer(file_appender).init();
    } else {
        builder.init();
    };

    Ok(())
}

/// macro to get source code position to log errors before propagation
///
/// example:  read_to_string("x").log(pos!())?;
macro_rules! pos {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        &format!("{}:{}:{}:", file!(), line!(), column!())
    };
}
pub(crate) use pos;

/// Trait to log the error from Result before propagation with ?.
pub trait ResultLogError<T, E>: Sized {
    fn log(self, file_line_column: &str) -> Self;
}

/// Implements LogError for anyhow::Result.
impl<T, E: std::fmt::Debug> ResultLogError<T, E> for core::result::Result<T, E> {
    fn log(self, file_line_column: &str) -> Self {
        self.inspect_err(|err| tracing::debug!("automation_tasks_rs/{} {:?}", file_line_column, err))
    }
}
