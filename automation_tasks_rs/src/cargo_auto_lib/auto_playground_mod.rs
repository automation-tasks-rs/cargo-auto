// auto_playground_mod.rs

//! Includes the link to playground with the rust code in a parameter.

use crate::cargo_auto_lib::error_mod::{Error, Result};
use crate::cargo_auto_lib::public_api_mod::{RED, RESET, YELLOW};
use crate::cargo_auto_lib::utils_mod::*;
use crate::generic_functions_mod::ResultLogError;
use lazy_static::lazy_static;

lazy_static! {
    /// Capture the link in markdown style [name](link)
    static ref REGEX_MD_LINK: regex::Regex = regex::Regex::new(
    r#".+\[.+\]\((.+)\).+"#
    ).expect("regex new");
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
    println!("  {YELLOW}Running auto_playground{RESET}");
    let path = std::env::current_dir().log()?;
    //use traverse instead of glob
    let files = crate::cargo_auto_lib::utils_mod::traverse_dir_with_exclude_dir(
        &path,
        "/*.md",
        // exclude folders
        &["/.git".to_string(), "/target".to_string(), "/docs".to_string(), "/pkg".to_string()],
    ).log()?;
    for md_filename in files {
        let md_filename = camino::Utf8Path::new(&md_filename);

        let mut is_changed = false;
        let md_old = std::fs::read_to_string(md_filename).log()?;

        // check if file have CRLF and show error
        if md_old.contains("\r\n") {
            return Err(Error::ErrorFromString(format!(
                "{RED}Error: {md_filename} has CRLF line endings instead of LF. Correct the file! {RESET}"
            )));
        }
        let mut iteration_start_pos = 0;
        let mut md_new = String::new();
        // find markers
        while let Ok(marker_start) =
            find_pos_start_data_after_delimiter(&md_old, iteration_start_pos, "\n[//]: # (auto_playground start)\n")
        {
            let Ok(code_start) = find_pos_start_data_after_delimiter(&md_old, marker_start, "\n```") else {
                return Ok(());
            };
            // the first triple tick can be ``` or ```rust or ```Rust or ```Rust ignore. Therefore I find the triple tick and then I find next newline.
            let Ok(code_start) = find_pos_start_data_after_delimiter(&md_old, code_start, "\n") else {
                return Ok(());
            };
            let Ok(code_end) = find_pos_end_data_before_delimiter(&md_old, code_start + 3, "\n```\n") else {
                return Ok(());
            };
            let Ok(marker_end) = find_pos_end_data_before_delimiter(&md_old, marker_start, "\n[//]: # (auto_playground end)\n") else {
                return Ok(());
            };
            let rust_code = &md_old[code_start..code_end];
            let rust_code_encoded = urlencoding::encode(rust_code).to_string();
            let playground_link = format!("https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code={rust_code_encoded}");
            // Some browsers can limit the url length to 2048 characters. That is absolutely too short for any interesting code example.
            // TODO: in that case save a gist with GitHub API and send that gist_id to playground? That can be very long.

            // replace the link inside markdown link notation. First find the text between marker_start and code_start
            let text_that_has_the_link = &md_old[marker_start..code_start];
            // parse this format [Rust playground](https:...)
            let cap_group = REGEX_MD_LINK.captures(text_that_has_the_link).ok_or_else(|| {
                Error::ErrorFromString(format!(
                    "{RED}Error: The old link '{text_that_has_the_link}' is NOT in this format '[Rust playground](https:...)'{RESET}"
                ))
            }).log()?;
            let old_link = &cap_group[1];
            // replace the old link with the new one
            let text_that_has_the_link = text_that_has_the_link.replace(old_link, &playground_link);
            is_changed = true;
            md_new.push_str(&format!(
                "{}{}{}",
                &md_old[iteration_start_pos..marker_start],
                &text_that_has_the_link,
                &md_old[code_start..marker_end],
            ));

            // start of the next iteration
            iteration_start_pos = marker_end;
        } // end while

        // if changed, then write to disk
        if is_changed {
            println!("  {YELLOW}Code inside {md_filename} has changed. Playground link corrected.{RESET}");
            // push the remaining text
            md_new.push_str(&md_old[iteration_start_pos..md_old.len()]);
            let bak_filename = md_filename.with_extension("bak");
            std::fs::write(&bak_filename, md_old).log()?;
            std::fs::write(md_filename, md_new).log()?;
        }
    }
    println!("  {YELLOW}Finished auto_playground{RESET}");
    Ok(())
}
