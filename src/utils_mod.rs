// utils_mod.rs

//! various utilities

/// run one shell command and return true if success
pub fn run_shell_command_success(shell_command: &str) -> bool {
    if !shell_command.starts_with("echo ") {
        println!("    $ {}", shell_command);
    }
    let status = std::process::Command::new("sh").arg("-c").arg(shell_command).status().unwrap();
    // return
    status.success()
}
