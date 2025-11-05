// public_api_mod.rs

// region: auto_md_to_doc_comments include doc_comments/public_api_mod.md A //!
//! public_api_mod gives more control over changes in the public api.
//!
//! The Public API of a library is a pain in the a...  
//! Every time I modify something I have to think how it will affect the users of the library.  
//! They could have tens or hundreds of places where they use the library. Breaking changes are terrible things.  
//! The developers are not willing to change their code every time a library changes slightly.  
//! Yes, there is the semver to show if the new library is API compatible. That helps a lot.  
//! It is dreaded if the first version of a function does not return a Result<>.  
//! Then later we will surely come to the point, that we need to return a Result<>. This is a terrible breaking change.  
//! It is wise to return a Result always. Even when that is not needed right now. It will surely be needed in the future.  
//! Another tactic is to make new functions with a different name and mark the old functions as Obsolete.
//!
//! This library is used by the automation_tasks_rs executable.  
//! I want to have here the complete and exact definition of the public API.  
//! Therefore I will not use reexports like `pub use` or `pub mod`.  
//! This way I can always know easily if my public API has changed.  
//! Just compare the `public_api_mod.rs` file in git diff.  
//! Adding functions, structs, methods and enums is ok, it does not break the Public API.  
//! But modifying existing functions, methods or enums will break the compatibility.  
//!
// endregion: auto_md_to_doc_comments include doc_comments/public_api_mod.md A //!

// region: Public API constants
// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
/// ANSI color
pub const RED: &str = "\x1b[31m";
/// ANSI color
pub const GREEN: &str = "\x1b[32m";
/// ANSI color
pub const YELLOW: &str = "\x1b[33m";
/// ANSI color
pub const BLUE: &str = "\x1b[34m";
/// ANSI color
pub const RESET: &str = "\x1b[0m";
// endregion: Public API constants

// region: re-export entire dependencies
pub use inquire;
// endregion: re-export entire dependencies

// region: Public API structs and methods

/// Result type with fixed LibError using thiserror.  
///
/// It makes simpler to write returns from functions.  
pub use crate::cargo_auto_lib::error_mod::{Error, Result};

/// Similar to std::process::Output, but with i32 and Strings for easier work.
pub use crate::cargo_auto_lib::auto_shell_mod::ShellOutput;

// reexporting a struct needs to export the trait to also reexports all the methods

/// Read data from Cargo.toml.  
pub use crate::cargo_auto_lib::auto_cargo_toml_mod::CargoToml;

// Just for making the struct methods obvious as public methods
// I created a special trait for this struct
// AFTERTHOUGHT: This makes me think if most the functions are methods,
// then it is consistent how to make the public API definition.
// There is a downside: the caller must bring the trait into scope. A little annoying.

/// Trait with methods to read data from Cargo.toml.
pub trait CargoTomlPublicApiMethods {
    /// read Cargo.toml, for workspaces it is the Cargo.toml of the first member
    fn read() -> Result<Self>
    where
        Self: Sized;
    /// Cargo.toml package name
    fn package_name(&self) -> String;
    /// Cargo.toml package version
    fn package_version(&self) -> String;
    /// Cargo.toml package authors as string
    fn package_authors_string(&self) -> String;
    /// Cargo.toml package authors as string without emails
    fn package_author_name(&self) -> String;
    /// Cargo.toml package repository
    fn package_repository(&self) -> Option<String>;
    /// Cargo.toml package description
    fn package_description(&self) -> Option<String>;
    /// Cargo.toml package homepage
    fn package_homepage(&self) -> String;
    /// Cargo.toml workspace members
    fn workspace_members(&self) -> Option<Vec<String>>;
    /// github_owner from package.repository
    fn github_owner(&self) -> Option<String>;
    /// Cargo.toml package keywords
    fn package_keywords(&self) -> Vec<String>;
}

/// Shell command builder with simple but limited sanitizer.
///
/// The limited sanitization will error if the value contains double quotes.
/// Command injections attack is possible because the shell command mixes executable code and data in a single string.
/// The attacker could format the "user input" data in a way that it transforms it into "executable code".
/// A true sanitization is hard to do in software. It would mean to understand all the intricacies of bash syntax?!
/// Another solution is to create a complex object model to have every command and data separated. Too complicated and developer unfriendly.
/// Instead here we take that the developer is a trusted person and he knows how to create the template correctly,
/// so that the placeholders are always de-facto delimited with double-quote inside the shell command.
/// This avoids the problem of injection of any other symbol except double-quotes.
/// The injection of double quote would finish the double-quote data and open the door tho write executable code.
/// It would be very complicated to check if "escaped double quotes" are or not correct in the context of the template.
/// So I don't allow them at all. This covers the vast majority of simple use cases.
/// Placeholders are delimited with curly brackets.
pub use crate::cargo_auto_lib::auto_shell_mod::ShellCommandLimitedDoubleQuotesSanitizer;

/// Trait with methods for ShellCommandLimitedDoubleQuotesSanitizer.
pub trait ShellCommandLimitedDoubleQuotesSanitizerTrait {
    /// Template for the shell command with placeholders
    ///
    /// The limited sanitization will error if the value contains double quotes.
    /// Placeholders are delimited with curly brackets.
    /// The developer must be super careful to write the template correctly.
    /// The placeholders must be inside a block delimited with double quotes.
    /// In a way that only an injection of a double quote can cause problems.
    /// There is no software check of the correctness of the template.
    fn new(template: &str) -> Result<Self>
    where
        Self: Sized;
    /// Replace placeholders with the value
    ///
    /// The limited sanitization will error if the value contains double quotes.
    /// Enter the placeholder parameter delimited with curly brackets.
    /// It would be very complicated to check if "escaped double quotes" are or not correct in the context of the template.
    /// So I don't allow them at all. This covers the vast majority of simple use cases.
    fn arg(&mut self, placeholder: &str, value: &str) -> Result<&mut Self>;

    /// Just like arg(), but for secrets that must be not echoed on the screen
    fn arg_secret(&mut self, placeholder: &str, value: &secrecy::SecretString) -> Result<&mut Self>;

    /// Run the sanitized command with no additional checks
    fn run(&self) -> Result<()>;
}

// endregion: Public API structs and methods

// region: Public API functions
/// Find 'from pos'.
pub fn find_from(text: &str, from_pos: usize, find: &str) -> Result<usize> {
    crate::cargo_auto_lib::utils_mod::find_from(text, from_pos, find)
}

/// Return the position of end of the delimited data before the delimiter.
pub fn find_pos_end_data_before_delimiter(md_text_content: &str, pos: usize, delimiter: &str) -> Result<usize> {
    crate::cargo_auto_lib::utils_mod::find_pos_end_data_before_delimiter(md_text_content, pos, delimiter)
}

/// Return the position of start of the delimited data after the delimiter.
pub fn find_pos_start_data_after_delimiter(md_text_content: &str, pos: usize, delimiter: &str) -> Result<usize> {
    crate::cargo_auto_lib::utils_mod::find_pos_start_data_after_delimiter(md_text_content, pos, delimiter)
}

/// The original `concat()` function does not have a delimiter.
pub fn concatenate_vec_to_string(vec: &[String], delimiter: &str) -> String {
    crate::cargo_auto_lib::utils_mod::concatenate_vec_to_string(vec, delimiter)
}

// region: auto_md_to_doc_comments include doc_comments/traverse_dir_with_exclude_dir.md A ///
/// Traverse dir and its sub-dir, but avoid excluded dirs.
///
/// The find_file and the exclude dir strings must start with /.
///
/// ## Example
///
/// ```Rust ignore
///
/// let files = cargo_auto_lib::traverse_dir_with_exclude_dir(
///     Path::new("/home/project/src"),
///     "/*.rs",
///     // avoid big folders
///     &vec![
///         "/.git".to_string(),
///         "/target".to_string(),
///         "/docs".to_string()
///     ]
/// ).expect("error");
/// for rs_file_name in files.iter() {
///     println!("{}", &rs_file_name);
/// }
/// ```
///
// endregion: auto_md_to_doc_comments include doc_comments/traverse_dir_with_exclude_dir.md A ///
pub fn traverse_dir_with_exclude_dir(dir: &std::path::Path, find_file: &str, exclude_dirs: &[String]) -> Result<Vec<String>> {
    crate::cargo_auto_lib::utils_mod::traverse_dir_with_exclude_dir(dir, find_file, exclude_dirs)
}

// region: auto_md_to_doc_comments include doc_comments/auto_cargo_toml_to_md.md A ///
/// This function includes data from Cargo.toml to markdown files.  
///
/// ![auto_cargo_toml_to_md.png](https://github.com/automation-tasks-rs/cargo_auto_lib/blob/main/images/auto_cargo_toml_to_md.png?raw=true)
///
/// This is nice for avoiding out of sync data.  
/// Run it on every build with `automation_tasks_rs` and [cargo auto](https://crates.io/crates/cargo-auto).  
///   
/// In the md file write these markers in invisible markdown comments.
///
/// ```markdown
/// [//comment]: # (auto_cargo_toml_to_md start)
///
/// [//comment]: # (auto_cargo_toml_to_md end)
///
/// ```
///
/// In this instructions I changed `[//]` to `[//comment]` to not process these markers.
///
/// `auto_cargo_toml_to_md` deletes the old lines between the markers and includes the Cargo.toml data:  
/// description, repository, version, utc_now, authors and creates badges for keywords and categories.
///
/// The words topics, keywords, hashtags and tags all mean the same concept.  
/// In cargo.toml we have keywords.  
/// In README.md I want to have badges with different color. And hashtags for SEO.  
/// In GitHub they are topics.
///
/// Some keywords have defined colors, others are orange like Rust.  
/// This can be expanded in the future.  
///
/// - Yellow: work-in-progress
/// - Green: maintained, ready-for-use
/// - Red: obsolete, archived
///
// endregion: auto_md_to_doc_comments include doc_comments/auto_cargo_toml_to_md.md A ///
pub fn auto_cargo_toml_to_md() -> Result<()> {
    crate::cargo_auto_lib::auto_cargo_toml_to_md_mod::auto_cargo_toml_to_md()
}

/// I want html pages to be correct microXML when I use them for single page application.
///
/// Before build or release this function will check for correctness.
pub fn auto_check_micro_xml(path_to_html_pages: &str) -> Result<()> {
    crate::cargo_auto_lib::auto_check_micro_xml_mod::auto_check_micro_xml(path_to_html_pages)
}

/// Deletes old js snippets when working with wasm-pack.  
///
/// The old folders for `js snippets` are not automatically deleted on building with `wasm-pack`.  
/// This utils do that.  
/// The function must be executed in the root project folder where is the Cargo.toml.  
pub fn auto_delete_old_js_snippets() -> Result<()> {
    crate::cargo_auto_lib::auto_delete_old_js_snippets_mod::auto_delete_old_js_snippets()
}

/// Print one or more sub_commands.
pub fn completion_return_one_or_more_sub_commands(sub_commands: Vec<&str>, word_being_completed: &str) {
    crate::cargo_auto_lib::auto_helper_functions_mod::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed)
}

// region: auto_md_to_doc_comments include doc_comments/exit_if_not_run_in_rust_project_root_directory.md A ///
/// Check if the code was run inside the Rust project root directory.  
///
/// There must be the `Cargo.toml` file and the directory `automation_tasks_rs`  
/// If not, exit with error message.  
///
// endregion: auto_md_to_doc_comments include doc_comments/exit_if_not_run_in_rust_project_root_directory.md A ///
pub fn exit_if_not_run_in_rust_project_root_directory() {
    crate::cargo_auto_lib::auto_helper_functions_mod::exit_if_not_run_in_rust_project_root_directory()
}

/// Run one shell command with static str.
///
/// We trust the "developer" that he will not make "command injection" in his own code.
/// The problem that must be sanitized is always "user input".
/// Exit task execution if the command has Exit Status != 0.
pub fn run_shell_command_static(shell_command: &'static str) -> Result<()> {
    crate::cargo_auto_lib::auto_shell_mod::run_shell_command_static(shell_command)
}

/// Run one shell command.
///
/// Exit task execution if the command has Exit Status != 0.
/// TODO: vulnerable to command injection
pub fn run_shell_command(shell_command: &str) -> Result<()> {
    crate::cargo_auto_lib::auto_shell_mod::run_shell_command(shell_command)
}

// region: auto_md_to_doc_comments include doc_comments/auto_lines_of_code.md A ///
/// This function inserts shield badges with lines_of_code into README.rs.  
///
/// ![auto_lines_of_code.png](https://github.com/automation-tasks-rs/cargo_auto_lib/blob/main/images/auto_lines_of_code.png?raw=true)
///
/// The parameter Link will be used for shield badge. If empty_string, the git remote repository will be used.  
/// Lines of code are not a "perfect" measurement of anything.  
/// Anybody can write a very big number of lines of useless code and comments.  
/// But for 95% of the cases they are good enough.  
/// Most of the developers use some "standard" coding practices and that is quantifiable and comparable.  
///
/// The `src_code_lines` is the most important count.  
/// That is actual code written for that project without doc comments, comments, unit tests, integration tests and examples.  
/// Sometimes is great to see a big number here. It means there was a lot of work invested. But other times we want to see a small number. It
/// means the developer understands the problem very well and don't try to solve anything outside that scope.  
/// The `src_doc_comment_lines` counts doc comments. They will eventually become docs. The count of lines shows how many documentation is written.  
/// The `src_comment_lines` counts code comments. Code comments are important to understand the code. The count of lines shows how understandable is the code.  
/// The `tests_lines` counts lines in tests and shows how good is the code tested. Here are the unit tests and integration test combined.  
/// The `examples_lines` counts lines in examples and shows how good is explained how to use the code.  
///
///
/// ## Folder and file structure
///
/// The folder structure of a single Rust project is simple.  
/// The project starts in the folder that contains `Cargo.toml`.  
/// The `src/` folder contains all the rust `*.rs` files.  
/// The `tests/` folder contains integration tests.  
/// The `examples/` folder contains examples.  
/// Some rs files can be excluded from the count adding this line near the start of the file: // exclude from auto_lines_of_code
/// Inside a rs file the doc comment line start with `///` or `//!`.  
/// The normal comments start with `//` or `/!`.  
/// I will ignore the block comments. They are usually NOT used for comments, but to temporarily disable a piece of code. So I count this as code and not comments.  
/// The `src/*.rs` file can contain unit tests that start with `#[cfg(test)]`. I assume that these are always at the end of the file.  
/// There should not be any normal code after `#[cfg(test)]`, only tests.  
/// All other files: `md`, `toml`, `html`, `js`, ... are not counted.  
///
/// ## Include into README.md
///
/// If the README.md file contains these markers (don't copy the numbers 1 and 2):  
///
/// ```md
/// [//comment]: # (auto_lines_of_code start)
///
/// [//comment]: # (auto_lines_of_code end)
/// ```
///
/// In this instructions I changed `[//]` to `[//comment]` to not process these markers.
///
/// The function will include the shield badges code between them.  
/// It will erase the previous content.  
/// Use git diff to see the change.  
///
// endregion: auto_md_to_doc_comments include doc_comments/auto_lines_of_code.md A ///
pub fn auto_lines_of_code(link: &str) -> Result<()> {
    crate::cargo_auto_lib::auto_lines_of_code_mod::auto_lines_of_code(link)
}

// region: auto_md_to_doc_comments include doc_comments/auto_md_to_doc_comments.md A ///
/// Finds rs files with markers and include segments from md files as doc comments.  
///
/// ![auto_md_to_doc_comments.png](https://github.com/automation-tasks-rs/cargo_auto_lib/blob/main/images/auto_md_to_doc_comments.png?raw=true)
///
/// From this doc comments `cargo doc` will generated the documentation and auto-completion.  
/// We don't want to manually copy this segments. We want them to be automatically in sync.  
/// We will just run this function before every `cargo doc` with an automation task.  
/// The `auto_md_to_doc_comments` function must be executed in the project root folder where is the Cargo.toml file.  
/// First it searches all the rs files in src, tests and examples folders.  
/// If they contain the markers, than finds the md file and the named segment and include it as doc comments into the rs file.  
/// The markers are always in pairs: start and end. So exactly the content in between is changed.  
/// The markers are always comments, so they don't change the code.  
/// It works only for files with LF line delimiter. No CR and no CRLF.  
///
/// ## markers
///
/// In the rs file write these markers:  
///
/// ```code
/// //comment region: auto_md_to_doc_comments include README.md A ///
/// //comment endregion: auto_md_to_doc_comments include README.md A ///
/// ```
///
/// In your rust code, change the word `comment` with double slash `//`.  
/// In the md file put markers to mark the segment:  
///
/// ```markdown
/// [//comment]: # (auto_md_to_doc_comments segment start A)  
/// [//comment]: # (auto_md_to_doc_comments segment end A)  
/// ```
///
/// In this instructions I changed `[//]` to `[//comment]` to not process these markers.
///
/// The marker must be exclusively in one line. No other text in the same line.  
/// auto_md_to_doc_comments will delete the old lines between the markers.  
/// It will find the md file and read the content between the markers.  
/// Before each line it will add the doc comment symbol as is defined in the marker.  
/// Finally it will include the new lines as doc comments in the rs file.
///
// endregion: auto_md_to_doc_comments include doc_comments/auto_md_to_doc_comments.md A ///
pub fn auto_md_to_doc_comments() -> Result<()> {
    crate::cargo_auto_lib::auto_md_to_doc_comments_mod::auto_md_to_doc_comments()
}

/// Process plantuml in current directory.  
///
/// Finds markers (auto_plantuml start) and (auto_plantuml end) in md files.  
/// If needed, calls the web service and saves the svg file.  
/// Between markers adds the link to the svg file.  
/// repo_url like <https://github.com/automation-tasks-rs/sey_currency_converter_pwa>
/// So the image file link is from the repository and accessible everywhere.  
pub fn auto_plantuml(repo_url: &str) -> Result<()> {
    crate::cargo_auto_lib::auto_plantuml_mod::auto_plantuml(repo_url)
}

/// Process plantuml for all md files.
///
/// For test and examples I need to provide the path.
pub fn auto_plantuml_for_path(path: &std::path::Path, repo_url: &str) -> Result<()> {
    crate::cargo_auto_lib::auto_plantuml_mod::auto_plantuml_for_path(path, repo_url)
}

/// Hash text.
pub fn hash_text(text: &str) -> String {
    crate::cargo_auto_lib::auto_plantuml_mod::hash_text(text)
}

/// Increment the minor version in Cargo.toml file only if files are changed.
pub fn auto_semver_increment_minor() -> Result<()> {
    crate::cargo_auto_lib::auto_semver_mod::auto_semver_increment_minor()
}

/// Increment the minor version in Cargo.toml file even if files are not changed.
pub fn auto_semver_increment_minor_forced() -> Result<()> {
    crate::cargo_auto_lib::auto_semver_mod::auto_semver_increment_minor_forced()
}

/// Increment the patch version in Cargo.toml file only if files are changed.
pub fn auto_semver_increment_patch() -> Result<()> {
    crate::cargo_auto_lib::auto_semver_mod::auto_semver_increment_patch()
}

/// Increment the patch version in Cargo.toml file even if files are not changed.
pub fn auto_semver_increment_patch_forced() -> Result<()> {
    crate::cargo_auto_lib::auto_semver_mod::auto_semver_increment_patch_forced()
}

/// Increment the version in Cargo.toml.
///
/// If the major version is greater than 2000, it is a date version  
/// else it is semver and increments the patch part.  
pub fn auto_version_increment_semver_or_date() -> Result<()> {
    crate::cargo_auto_lib::auto_semver_or_date_mod::auto_version_increment_semver_or_date()
}

/// Increment the version in Cargo.toml, forced.
///
/// If the major version is greater than 2000, it is a date version  
/// else it is semver and increments the patch part.  
/// Forced is used in workspaces to force all members to have the same date version.  
pub fn auto_version_increment_semver_or_date_forced() -> Result<()> {
    crate::cargo_auto_lib::auto_semver_or_date_mod::auto_version_increment_semver_or_date_forced()
}

// region: auto_md_to_doc_comments include doc_comments/auto_version_from_date.md A ///
/// New version from date is written to Cargo.toml and service_worker.js.
///
/// In Cargo.toml writes the version as the date `yyyy.mmdd.HHMM` ex. `2019.1221.2359`.  
/// For non-library projects, the semver specification is not really useful.  
/// Having the version as the date is just fine for executables and much more human readable.  
///
/// ### service_worker.js
///
/// Inside the PWA service worker javascript file is also needed to change the version.  
/// The program searches for `service_worker.js` and modify the version.  
///
/// ### no need to change version if no files changed
///
/// If src/*.rs or Cargo.toml files are not changed from last compile, than no need to change version.  
/// The dates of the files will be stored in the file .automation_tasks_rs_file_hashes.json near to Cargo.toml.  
/// Warning: I don't check if the service worker has changed because it rarely does.  
/// To know if the projects has changed or not, this function saves the dates of all files into `.automation_tasks_rs_file_hashes.json` near Cargo.toml
///
// endregion: auto_md_to_doc_comments include doc_comments/auto_version_from_date.md A ///
pub fn auto_version_from_date() -> Result<()> {
    crate::cargo_auto_lib::auto_version_from_date_mod::auto_version_from_date()
}

/// Just like auto_version_from_date(), but force the new version even if no files are changed.
///
/// For workspaces `release` I want to have the same version in all members.  
/// It is slower, but easier to understand when deployed.
pub fn auto_version_from_date_forced() -> Result<()> {
    crate::cargo_auto_lib::auto_version_from_date_mod::auto_version_from_date_forced()
}

/// Pretty HTML for docs.  
///
/// The HTML generated by `cargo doc` is ugly and difficult to `git diff`.
/// Tidy HTML is a HTML checker and formatter installed on most Linuxes.
pub fn auto_doc_tidy_html() -> Result<()> {
    crate::cargo_auto_lib::auto_doc_tidy_html_mod::auto_doc_tidy_html()
}

/// Does git have settings for remote.
pub fn git_has_remote() -> Result<bool> {
    crate::cargo_auto_lib::auto_git_mod::git_has_remote()
}

/// Check if this folder is a local Git repository.
pub fn git_is_local_repository() -> Result<bool> {
    crate::cargo_auto_lib::auto_git_mod::git_is_local_repository()
}

/// Run one shell command and return ShellOutput {exit_status, stdout, stderr}.
///
/// TODO: vulnerable to command injection
pub fn run_shell_command_output(shell_command: &str) -> Result<ShellOutput> {
    crate::cargo_auto_lib::auto_shell_mod::run_shell_command_output(shell_command)
}

/// Run one shell command and return true if success.
///
/// TODO: vulnerable to command injection
pub fn run_shell_command_success(shell_command: &str) -> Result<bool> {
    crate::cargo_auto_lib::auto_shell_mod::run_shell_command_success(shell_command)
}

/// Get home dir using the home crate.
///
/// Error if HOME not found.
pub fn home_dir() -> Result<std::path::PathBuf> {
    crate::cargo_auto_lib::auto_helper_functions_mod::home_dir()
}

/// Replace tilde with home::home_dir, only for utf8.
pub fn tilde_expand_to_home_dir_utf8(path_str: &str) -> Result<camino::Utf8PathBuf> {
    crate::cargo_auto_lib::auto_helper_functions_mod::tilde_expand_to_home_dir_utf8(path_str)
}

/// Sync, check, create, push git tag.
pub fn git_tag_sync_check_create_push(version: &str) -> Result<String> {
    crate::cargo_auto_lib::auto_github_mod::git_tag_sync_check_create_push(version)
}

/// Get release text from RELEASES.md.
///
/// First, the user must write the content into file RELEASES.md in the section ## Unreleased.  
/// Then the automation task will copy the content to GitHub release  
/// and create a new Version title in RELEASES.md.  
pub fn body_text_from_releases_md() -> Result<String> {
    crate::cargo_auto_lib::auto_github_mod::body_text_from_releases_md()
}

/// Create a new Version title in RELEASES.md.
pub fn create_new_version_in_releases_md(release_name: &str) -> Result<()> {
    crate::cargo_auto_lib::auto_github_mod::create_new_version_in_releases_md(release_name)
}

/// UTC  date in iso standard like 2024-12-31
pub fn now_utc_date_iso() -> String {
    crate::cargo_auto_lib::utils_mod::now_utc_date_iso()
}

// region: auto_md_to_doc_comments include doc_comments/copy_folder_files_into_module.md A ///
/// Copy all files from the folder into a module as strings (static &str).
///
/// The Rust code to modify has the markers:
///
/// ```Rust ignore
/// //comment region: files copied into strings by automation tasks
///
/// //comment endregion: files copied into strings by automation tasks
///
/// ```
///
/// In this instructions I changed `[//]` to `[//comment]` to not process these markers.
///
/// First we create the complete text, then we check if the old text needs to be replaced.
///
/// Binary files need a special treatment:
///
/// ```Rust ignore
/// ext_for_binary_files=vec![".ico",".jpg",".png",".woff2"];
/// ```
///
/// Exclude big folders:
///
/// ```Rust ignore
/// exclude_big_folders = vec!["/.git","/target","/docs"];
/// ```
///
// endregion: auto_md_to_doc_comments include doc_comments/copy_folder_files_into_module.md A ///
pub fn copy_folder_files_into_module(
    folder_path: &std::path::Path,
    module_path: &std::path::Path,
    ext_for_binary_files: &[&str],
    exclude_big_folders: &[String],
) -> Result<()> {
    crate::cargo_auto_lib::auto_copy_files_to_strings_mod::copy_folder_files_into_module(
        folder_path,
        module_path,
        ext_for_binary_files,
        exclude_big_folders,
    )
}

/// Add commit message to Unreleased in RELEASES.md.
pub fn add_message_to_unreleased(message: &str) -> Result<()> {
    crate::cargo_auto_lib::auto_github_mod::add_message_to_unreleased(message)
}

// region: auto_md_to_doc_comments include doc_comments/auto_playground_run_code.md A ///
/// Include the link to run code in Rust playground.
///
/// The function searches in all markdown files for markers like this:
///
/// ```markdown
/// [//comment]: # (auto_playground start)
///
/// Run this code in the [Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20m%0A%7D):
///
/// '''Rust ignore
/// fn main(){
///     println!("Hello World!");
/// }
/// '''
///
/// [//comment]: # (auto_playground end)
/// ```
///
/// In this instructions I changed `[//]` to `[//comment]` and  ticks to single quotes to not process these markers.
///
/// Between the start marker and the first triple backtick there is the link in "()" parentheses. The link to Rust playground encodes the code with url_encoding (percents) and sends it as an Url parameter.
///
/// Info: docs.rs has already a functionality that shows the `Run` button on your code and can run code the playground if you put this line at the top of lib.rs:
///
/// ``` Rust ignore
/// #![doc(html_playground_url = "https://play.rust-lang.org")]
/// ```
///
/// But it works only on docs.rs.  
/// I want to run my code examples from everywhere: from GitHub README.md, GitHub pages and crates.io.  
///
// endregion: auto_md_to_doc_comments include doc_comments/auto_playground_run_code.md A ///
pub fn auto_playground_run_code() -> Result<()> {
    crate::cargo_auto_lib::auto_playground_mod::auto_playground_run_code()
}

/// Interactive ask to create a new local git repository.
pub fn new_local_repository(message: &str) -> Result<()> {
    crate::cargo_auto_lib::auto_git_mod::new_local_repository(message)
}
// endregion: Public API functions
