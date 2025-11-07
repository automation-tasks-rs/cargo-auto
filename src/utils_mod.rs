// utils_mod.rs

//! Functions for various utilities.

/// macro to get source code position to log errors before propagation
///
/// example:  read_to_string("x").log(pos!())?;
#[macro_export]
macro_rules! pos {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        &format!("{}:{}:{}:", file!(), line!(), column!())
    };
}

/// Trait to log the error from Result before propagation with ?.
pub trait ResultLogError<T, E>: Sized {
    fn log(self, file_line_column: &str) -> Self;
}

/// Implements LogError for anyhow::Result.
impl<T, E: std::fmt::Debug> ResultLogError<T, E> for core::result::Result<T, E> {
    fn log(self, file_line_column: &str) -> Self {
        self.inspect_err(|err| tracing::error!("{} {:?}", file_line_column, err))
    }
}

/// Run one shell command and return true if success.
pub fn run_shell_command_success(shell_command: &str) -> bool {
    if !shell_command.starts_with("echo ") && !shell_command.starts_with("printf ") {
        println!("    $ {}", shell_command);
    }
    match std::process::Command::new("sh").arg("-c").arg(shell_command).status() {
        Ok(status) => status.success(),
        Err(_err) => false,
    }
}
