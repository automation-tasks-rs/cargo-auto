// utils_mod.rs

//! Functions for various utilities.

/// Trait to log the error from Result before propagation with ?.
pub trait ResultLogError<T, E>: Sized {
    fn log(self) -> Self;
}

/// Implements LogError for anyhow::Result.
impl<T, E: std::fmt::Debug> ResultLogError<T, E> for core::result::Result<T, E> {
    #[inline(always)]
    #[track_caller]
    fn log(self) -> Self {
        self.inspect_err(|err| tracing::error!(?err))
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
