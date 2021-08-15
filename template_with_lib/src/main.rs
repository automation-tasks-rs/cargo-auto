/// automation_tasks_rs with_lib
fn main() {
    if is_not_run_in_rust_project_root_directory() {
        println!("Error: automation_tasks_rs must be called in the root directory of the rust project beside the Cargo.toml file and automation_tasks_rs directory.");
        // early exit
        std::process::exit(0);
    }
    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {            
            println!("Running automation_tasks_rs: {}", &task);
            // region: call task functions for the task argument
            if &task == "build" || &task == "b" {
                task_build();
            } else if &task == "release" || &task == "r" {
                task_release();
            } else if &task == "increment_minor" {
                task_increment_minor();                
            } else if &task == "docs" || &task == "doc" || &task == "d" {
                task_docs();
            } else {
                println!("Task {} is unknown.", &task);
                print_help();
            }
            // endregion: call functions for the task argument
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!("User defined tasks in automation_tasks_rs:");
    println!("cargo auto build - builds the crate in debug mode, fmt");
    println!("cargo auto release - builds the crate in release mode, version from date, fmt");
    println!("cargo auto increment_minor - increments the semver version minor part (only for libraries)");
    println!("cargo auto docs - builds the docs, copy to docs directory");
}

// region: tasks

/// example how to call a list of shell commands
fn task_build() {
    cargo_auto_lib::auto_cargo_toml_to_md();
    cargo_auto_lib::auto_lines_of_code("");
    
    #[rustfmt::skip]
    let shell_commands = [
        "echo $ cargo fmt", 
        "cargo fmt", 
        "echo $ cargo build", 
        "cargo build"];
    run_shell_commands(shell_commands.to_vec());
}

/// example how to call one shell command and combine with rust code
fn task_release() {
    // semver is used for libraries, version_from_date is used for binary
    // cargo_auto_lib::auto_semver_increment_patch();
    cargo_auto_lib::auto_version_from_date();
    cargo_auto_lib::auto_cargo_toml_to_md();
    cargo_auto_lib::auto_lines_of_code("");

    println!("$ cargo fmt");
    run_shell_command("cargo fmt");
    println!("$ cargo build --release");
    run_shell_command("cargo build --release");
}

/// semver is used for libraries, version_from_date is used for binary
fn task_increment_minor() {
    cargo_auto_lib::auto_semver_increment_minor();
    cargo_auto_lib::auto_cargo_toml_to_md();
}

/// example how to call a list of shell commands and combine with rust code
fn task_docs() {    
    cargo_auto_lib::auto_md_to_doc_comments();        
    #[rustfmt::skip]
    let shell_commands = [
        "echo $ cargo doc --no-deps --document-private-items",
        "cargo doc --no-deps --document-private-items",        
        // copy to /docs/ because it is github standard
        "echo $ rsync -a --info=progress2 --delete-after target/doc/ docs/",
        "rsync -a --info=progress2 --delete-after target/doc/ docs/",
        "echo Create simple index.html file in docs directory",
        &format!("echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",&project_directory_name()) ,
        // message to help user with next move
        "echo After successful doc, commit and push changes",
        ];
    run_shell_commands(shell_commands.to_vec());
}

// endregion: tasks

// region: helper functions

/// run one shell command
fn run_shell_command(shell_command: &str) {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(shell_command)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

/// run shell commands from a vector of strings.
fn run_shell_commands(shell_commands: Vec<&str>) {
    for shell_command in shell_commands {
        run_shell_command(shell_command);
    }
}

/// check if run in rust project root directory error and exit if not
/// there must be Cargo.toml and directory automation_tasks_rs
fn is_not_run_in_rust_project_root_directory() -> bool {
    // return negation of exists
    !(std::path::Path::new("automation_tasks_rs").exists() && std::path::Path::new("Cargo.toml").exists())
}

/// returns the directory name, that is usually also the crate name (for simplicity)
fn project_directory_name()->String{
    std::env::current_dir().unwrap().file_name().unwrap().to_string_lossy().to_string()
}

// endregion: helper functions

    