// auto_cargo_toml_to_md_mod

//! Includes data from Cargo.toml to `md` files: version, authors, description,...
//! ![auto_cargo_toml_to_md.png](https://github.com/automation-tasks-rs/cargo_auto_lib/blob/main/images/auto_cargo_toml_to_md.png?raw=true)
//! Read more in the auto_cargo_toml_to_md() function.

// region: use statements

use crate::cargo_auto_lib::error_mod::{Error, Result};
use crate::cargo_auto_lib::public_api_mod::{GREEN, RED, RESET, YELLOW};
use crate::generic_functions_mod::{pos, ResultLogError};
use glob::glob;
use lazy_static::lazy_static;
use regex::*;

// this trait must be in scope to use these methods of CargoToml
use crate::cargo_auto_lib::public_api_mod::CargoTomlPublicApiMethods;

// endregion: use statements

lazy_static! {
    /// Regex for start marker
    static ref REGEX_MD_START: Regex = Regex::new(r#"(?m)^\[//\]: # \(auto_cargo_toml_to_md start\)$"#).expect("regex new");
    /// Regex for end marker
    static ref REGEX_MD_END: Regex = Regex::new(r#"(?m)^\[//\]: # \(auto_cargo_toml_to_md end\)$"#).expect("regex new");
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
    let cargo_toml = crate::cargo_auto_lib::auto_cargo_toml_mod::CargoToml::read().log(pos!())?;
    let version = cargo_toml.package_version();
    let author_name = cargo_toml.package_author_name();
    let homepage = cargo_toml.package_homepage();
    let repository = cargo_toml.package_repository().unwrap_or("".to_owned());
    let description = cargo_toml.package_description().unwrap_or("".to_owned());
    let keywords = cargo_toml.package_keywords().to_vec();
    let now_utc_date_iso = &crate::cargo_auto_lib::utils_mod::now_utc_date_iso();

    let mut new_text = format!("\n**{description}**  \n");
    new_text.push_str(&format!(
        "***version: {version} date: {now_utc_date_iso} author: [{author_name}]({homepage}) repository: [GitHub]({repository})***\n\n"
    ));

    for keyword in keywords.iter() {
        let color = if keyword == "work-in-progress" {
            "yellow"
        } else if keyword == "maintained" || keyword == "ready-for-use" {
            "green"
        } else if keyword == "obsolete" || keyword == "archived" {
            "red"
        } else {
            "orange"
        };
        // inside the shield badge syntax, hyphens must be replaced by underscore
        let keyword_underscore = keyword.replace('-', "_");
        new_text.push_str(&format!(
            " ![{keyword}](https://img.shields.io/badge/{keyword_underscore}-{color})\n"
        ));
    }
    new_text.push('\n');

    for filename_result in glob("*.md").log(pos!())? {
        let filename_pathbuff = filename_result?;
        let md_filename = filename_pathbuff
            .to_str()
            .ok_or_else(|| Error::ErrorFromStr("filename_pathbuff is None"))
            .log(pos!())?;
        // println!("checking md_filename: {}", &md_filename);
        let mut md_text_content = std::fs::read_to_string(md_filename).log(pos!())?;

        // check if file have CRLF and show error
        if md_text_content.contains("\r\n") {
            return Err(Error::ErrorFromString(format!(
                "{RED}Error: {md_filename} has CRLF line endings instead of LF. Correct the file! {RESET}"
            )));
        }

        if let Some(cap) = REGEX_MD_START.captures(&md_text_content) {
            let pos_start = cap
                .get(0)
                .ok_or_else(|| Error::ErrorFromStr("cap get 0 is None"))
                .log(pos!())?
                .end()
                + 1;
            if let Some(cap) = REGEX_MD_END.captures(&md_text_content) {
                let pos_end = cap
                    .get(0)
                    .ok_or_else(|| Error::ErrorFromStr("cap get 0 is None"))
                    .log(pos!())?
                    .start();
                md_text_content.replace_range(pos_start..pos_end, &new_text);
                println!("  {YELLOW}Write to md file: {}{RESET}", md_filename);
                println!("{GREEN}{}{RESET}", &new_text.trim_end_matches("\n\n"));
                std::fs::write(md_filename, md_text_content).log(pos!())?;
            }
        }
    }
    Ok(())
}
