// outside_of_rust_project_mod.rs

// region: use statements
// endregion

/// it is run outside a Rust project
/// It must have the argument "new_cli" and the project title
pub fn parse_args(args: &mut std::env::Args) {
    // the first argument is the task: new_cli
    // wooow! There is a difference if I call the standalone binary or as a cargo subcommand:
    // cargo-auto build     - build is the arg_1
    // cargo auto build     - build is the arg_2
    let arg_1 = args.next();
    match arg_1 {
        None => print_help_from_cargo_auto(),
        Some(task) => {
            if task != "auto" {
                // when calling as `cargo auto build`
                match_first_argument(&task, args);
            } else {
                // when calling as `cargo-auto build`
                let arg_2 = args.next();
                match arg_2 {
                    None => print_help_from_cargo_auto(),
                    Some(task) => match_first_argument(&task, args),
                }
            }
        }
    }
}

///  
fn print_help_from_cargo_auto() {
    println!("");
    println!("Outside of a Rust project, cargo-auto can create a new Rust project for CLI:");
    println!("$ cargo auto new_cli project_name");
}

/// the first argument is the task: new_cli,...  
/// in development use: `cargo run -- new_cli`  
fn match_first_argument(task: &str, args: &mut std::env::Args) {
    if task == "completion" {
        completion();
    } else if task == "new_cli" {
        let arg_2 = args.next();
        new_cli(arg_2);
    } else {
        print_help_from_cargo_auto();
    }
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    /// println one, more or all sub_commands
    fn completion_return_one_or_more_sub_commands(sub_commands: Vec<&str>, word_being_completed: &str) {
        let mut sub_found = false;
        for sub_command in sub_commands.iter() {
            if sub_command.starts_with(word_being_completed) {
                println!("{}", sub_command);
                sub_found = true;
            }
        }
        if sub_found == false {
            // print all sub-commands
            for sub_command in sub_commands.iter() {
                println!("{}", sub_command);
            }
        }
    }

    let args: Vec<String> = std::env::args().collect();
    let last_word = args[2].as_str();
    let mut word_being_completed = " ";
    if args.len() > 3 {
        word_being_completed = args[3].as_str();
    }
    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["new_cli"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
}

pub fn new_cli(arg_2: Option<String>) {
    match arg_2 {
        None => println!("Project name argument is missing: `cargo auto new_cli project_name`"),
        Some(project_name) => {
            crate::template_new_cli_mod::copy_to_files(&project_name);
            println!("");
            println!(
                "The command `crate auto new_cli` generated the directory `{}`",
                &project_name
            );
            println!("You can open this new rust project in a new rust editor.");
            println!("For example:");
            println!("`cd {}`", &project_name);
            println!("`code .`");
        }
    }
}
