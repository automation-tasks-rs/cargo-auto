//! automation_tasks_rs for cargo-auto

mod copy_files_to_strings_mod;

use cargo_auto_lib::*;


fn main() {
    exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("Running automation task: {}", &task);
                if &task == "build" {
                    task_build();
                } else if &task == "release" {
                    task_release();
                } else if &task == "test" {
                    task_test();
                } else if &task == "doc" {
                    task_doc();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else if &task == "publish_to_crates_io" {
                    task_publish_to_crates_io();
                } else {
                    println!("Task {} is unknown.", &task);
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
User defined tasks in automation_tasks_rs:
cargo auto build - builds the crate in debug mode, fmt, increment version
cargo auto release - builds the crate in release mode, fmt, increment version
cargo auto doc - builds the docs, copy to docs directory
cargo auto test - runs all the tests
cargo auto commit_and_push "message" - commits with message and push with mandatory message
      (If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git.)
cargo auto publish_to_crates_io - publish to crates.io, git tag
      (YOu need to save the credentials before publishing. On crates.io get the 'access token'. Then save it locally with the command `cargo login TOKEN`)
"#
    );
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec![
            "build",
            "release",
            "doc",
            "test",
            "commit_and_push",
            "publish_to_crates_io",
        ];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["with_lib"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// cargo build
fn task_build() {
    let cargo_toml = CargoToml::read();

    copy_files_to_strings_mod::copy_folder_files_into_module(
        std::path::Path::new("template_new_auto"), 
        std::path::Path::new("src/template_new_auto_mod.rs"));
 
    copy_files_to_strings_mod::copy_folder_files_into_module(
        std::path::Path::new("template_new_cli"), 
        std::path::Path::new("src/template_new_cli_mod.rs"));

    auto_version_increment_semver_or_date();
    run_shell_command("cargo fmt");
    run_shell_command("cargo build");
    println!(
        r#"
After `cargo auto build`, run the compiled binary
run `./target/debug/{package_name} argument`, if ok, then
run `cargo auto release`
"#,
        package_name = cargo_toml.package_name(),
    );
}


/// cargo build --release
fn task_release() {

    copy_files_to_strings_mod::copy_folder_files_into_module(
        std::path::Path::new("template_new_auto"), 
        std::path::Path::new("src/template_new_auto_mod.rs"));
 
    copy_files_to_strings_mod::copy_folder_files_into_module(
        std::path::Path::new("template_new_cli"), 
        std::path::Path::new("src/template_new_cli_mod.rs"));

    let cargo_toml = CargoToml::read();
    auto_version_increment_semver_or_date();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");

    run_shell_command("cargo fmt");
    run_shell_command("cargo build --release");
    println!(
        r#"
After `cargo auto release`, run the compiled binary
run `./target/release/{package_name} argument` if ok, then
run `cargo auto doc`
"#,
        package_name = cargo_toml.package_name(),
    );
}


/// cargo doc, then copies to /docs/ folder, because this is a github standard folder
fn task_doc() {
    let cargo_toml = CargoToml::read();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");
    auto_md_to_doc_comments();

    run_shell_command("cargo doc --no-deps --document-private-items");
    // copy target/doc into docs/ because it is github standard
    run_shell_command("rsync -a --info=progress2 --delete-after target/doc/ docs/");
    // Create simple index.html file in docs directory
    run_shell_command(&format!(
        "echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",
        cargo_toml.package_name().replace("-", "_")
    ));
    // message to help user with next move
    println!(
        r#"
After `cargo auto doc`, check `docs/index.html`. If ok, then 
run `cargo auto test`
"#
    );
}

/// cargo test
fn task_test() {
    run_shell_command("cargo test");
    println!(
        r#"
After `cargo auto test`. If ok, then 
run `cargo auto commit_and_push "message"` with mandatory commit message
"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    match arg_2 {
        None => println!("Error: message for commit is mandatory"),
        Some(message) => {
            run_shell_command(&format!(r#"git add -A && git commit -m "{}""#, message));
            run_shell_command("git push");
            println!(
                r#"
After `cargo auto commit_and_push "message"`
run `cargo auto publish_to_crates_io`
"#
            );
        }
    }
}

/// publish to crates.io and git tag
fn task_publish_to_crates_io() {
    println!(r#"The crates.io access token must already be saved locally with `cargo login TOKEN`"#);

    let cargo_toml = CargoToml::read();
    // git tag
    let shell_command = format!(
        "git tag -f -a v{version} -m version_{version}",
        version = cargo_toml.package_version()
    );
    run_shell_command(&shell_command);

    // cargo publish
    run_shell_command("cargo publish");
    println!(
        r#"
After `cargo auto publish_to_crates_io`, 
check `https://crates.io/crates/{package_name}`.
Install the crate with `cargo install {package_name}` and check how it works.
"#,
        package_name = cargo_toml.package_name(),
        // package_version = cargo_toml.package_version()
    );
}

// endregion: tasks
