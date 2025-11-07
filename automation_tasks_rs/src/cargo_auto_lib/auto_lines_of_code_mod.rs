// auto_lines_of_code_mod

//! Insert shield badges with lines_of_code into README.md.

use crate::cargo_auto_lib::error_mod::Error;
use crate::cargo_auto_lib::public_api_mod::{RED, RESET, YELLOW};
use crate::cargo_auto_lib::Result;
use crate::generic_functions_mod::{pos, ResultLogError};
use std::fs::File;
use std::io::{BufRead, BufReader};

// use crate::cargo_auto_lib::auto_helper_functions_mod::*;

#[derive(Default, Debug)]
/// Struct that contains 4 types of lines count: code, doc comments, comments, test and examples.
pub struct LinesOfCode {
    /// lines with code in srs files
    pub src_code_lines: usize,
    /// lines with doc_comments in srs files
    pub src_doc_comment_lines: usize,
    /// lines with comments in srs files
    pub src_comment_lines: usize,
    /// unit plus integration tests
    pub tests_lines: usize,
    /// all lines in examples files
    pub examples_lines: usize,
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
    println!("  {YELLOW}Running auto_lines_of_code{RESET}");
    let link = if link.is_empty() {
        crate::cargo_auto_lib::auto_git_mod::process_git_remote()
    } else {
        link.to_string()
    };
    // Cargo.toml contains the list of projects
    let lines_of_code = count_lines_of_code().log(pos!())?;
    let text_to_include = to_string_as_shield_badges(&lines_of_code, &link);
    include_into_readme_md(&text_to_include).log(pos!())?;
    println!("  {YELLOW}Finished auto_lines_of_code{RESET}");
    Ok(())
}

/// Returns the counted lines of code
///
/// Does not write to README.md.
pub fn count_lines_of_code() -> Result<LinesOfCode> {
    let mut lines_of_code = LinesOfCode::default();

    // src folder
    let files = crate::cargo_auto_lib::utils_mod::traverse_dir_with_exclude_dir(
        camino::Utf8Path::new("src").as_std_path(),
        "/*.rs",
        // avoid big folders
        &["/.git".to_string(), "/target".to_string(), "/docs".to_string()],
    )
    .log(pos!())?;
    // println!("{:#?}", files);
    for rs_file_name in files.iter() {
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(rs_file_name).log(pos!())?;
        let reader = BufReader::new(file);
        let mut is_unit_test = false;
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for line in reader.lines() {
            let line = line?; // Ignore errors.
            let line = line.trim_start();
            if line == "// exclude from auto_lines_of_code" {
                break;
            }
            if line.starts_with("///") || line.starts_with("//!") {
                lines_of_code.src_doc_comment_lines += 1;
            } else if line.starts_with("//") || line.starts_with("/!") {
                lines_of_code.src_comment_lines += 1;
            } else if line.starts_with("#[cfg(test)]") {
                is_unit_test = true;
            } else if is_unit_test {
                lines_of_code.tests_lines += 1;
            } else {
                lines_of_code.src_code_lines += 1;
            }
        }
    }
    // tests folder
    let files = crate::cargo_auto_lib::utils_mod::traverse_dir_with_exclude_dir(
        camino::Utf8Path::new("tests").as_std_path(),
        "/*.rs",
        // avoid big folders
        &["/.git".to_string(), "/target".to_string(), "/docs".to_string()],
    )
    .log(pos!())?;
    // println!("{:#?}", files);
    for rs_file_name in files.iter() {
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(rs_file_name).log(pos!())?;
        let reader = BufReader::new(file);
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for _line in reader.lines() {
            lines_of_code.tests_lines += 1;
        }
    }

    // examples folder
    let files = crate::cargo_auto_lib::utils_mod::traverse_dir_with_exclude_dir(
        camino::Utf8Path::new("examples").as_std_path(),
        "/*.rs",
        // avoid big folders
        &["/.git".to_string(), "/target".to_string(), "/docs".to_string()],
    )
    .log(pos!())?;
    for rs_file_name in files.iter() {
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(rs_file_name).log(pos!())?;
        let reader = BufReader::new(file);
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for _line in reader.lines().enumerate() {
            lines_of_code.examples_lines += 1;
        }
    }
    //println!("{:#?}", &lines_of_code);
    // return
    Ok(lines_of_code)
}

/// Returns a string with the markdown code for 4 shield badges.
///
/// Every badge has the link to the url given as first parameter
/// or automatically finds out the GitHub git remote repository url.
///
/// let v = cargo_auto_lib::auto_lines_of_code_mod::count_lines_of_code();
/// let badges = cargo_auto_lib::auto_lines_of_code_mod::to_string_as_shield_badges(&v,"");
///
/// println!("{}", badges);
/// TODO: cargo-auto_lib could change the code to make some element visibility `pub` only for testing. And after return to normal.  
fn to_string_as_shield_badges(v: &LinesOfCode, link: &str) -> String {
    //println!("to_string_as_shield_badges() start");

    let src_code_lines = format!(
        "[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-{}-green.svg)]({})",
        v.src_code_lines, link
    );
    let src_doc_comment_lines = format!(
        "[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-{}-blue.svg)]({})",
        v.src_doc_comment_lines, link
    );
    let src_comment_lines = format!(
        "[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-{}-purple.svg)]({})",
        v.src_comment_lines, link
    );
    let example_lines = format!(
        "[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-{}-yellow.svg)]({})",
        v.examples_lines, link
    );
    let tests_lines = format!(
        "[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-{}-orange.svg)]({})",
        v.tests_lines, link
    );
    //return
    format!(
        "{}\n{}\n{}\n{}\n{}\n",
        src_code_lines, src_doc_comment_lines, src_comment_lines, example_lines, tests_lines
    )
}

/// Includes (writes, modifies) the shield badge code into README.md file
fn include_into_readme_md(include_str: &str) -> Result<()> {
    let start_delimiter = "[//]: # (auto_lines_of_code start)";
    let end_delimiter = "[//]: # (auto_lines_of_code end)";
    let file_name = "README.md";

    if let Ok(readme_content) = std::fs::read_to_string(file_name) {
        // check if file have CRLF instead of LF and show error
        if readme_content.contains("\r\n") {
            return Err(Error::ErrorFromString(format!(
                "{RED}Error: {} has CRLF line endings instead of LF. Correct the file! {RESET}",
                file_name
            )));
        }

        let mut new_readme_content = String::with_capacity(readme_content.len());
        if let Some(mut pos_start) = readme_content.find(start_delimiter) {
            pos_start += start_delimiter.len();
            if let Some(pos_end) = readme_content.find(end_delimiter) {
                new_readme_content.push_str(&readme_content[..pos_start]);
                new_readme_content.push('\n');
                new_readme_content.push_str(include_str);
                new_readme_content.push('\n');
                new_readme_content.push_str(&readme_content[pos_end..]);
                /*
                println!(
                    "{}include_into_readme_md write file: {}{}",
                    *GREEN, file_name, *RESET
                );
                 */
                std::fs::write(file_name, new_readme_content).log(pos!())?;
            }
        }
    }
    Ok(())
}
