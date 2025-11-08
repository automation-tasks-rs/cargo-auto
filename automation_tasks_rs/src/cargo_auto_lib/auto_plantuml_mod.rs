// auto_plantuml_mod.rs

//! Includes the plantuml svg generated from the plantuml code.

use std::str::FromStr;

use crate::cargo_auto_lib::error_mod::{Error, Result};
use crate::cargo_auto_lib::public_api_mod::{RED, RESET, YELLOW};
use crate::cargo_auto_lib::utils_mod::*;
use crate::utils_mod::{pos, ResultLogError};
use lazy_static::lazy_static;

lazy_static! {
    /// Regex for start marker
    static ref REGEX_PLANTUML_START: regex::Regex = regex::Regex::new(
        r#"(?m)^\[//\]: # \(auto_plantuml start\)$"#
    ).expect("regex new");
        /// Regex for end marker
    static ref REGEX_PLANTUML_END: regex::Regex = regex::Regex::new(
        r#"(?m)^\[//\]: # \(auto_plantuml end\)$"#
    ).expect("regex new");
    /// Capture group for image link
    static ref REGEX_IMG_LINK: regex::Regex = regex::Regex::new(
    r#"!\[.+\]\(.+/svg_(.+)\.svg\)"#
    ).expect("regex new");
}

// region: auto_md_to_doc_comments include doc_comments/auto_plantuml.md A ///
/// Includes the plantuml svg generated from the plantuml code.
///
/// Search for markers in md files and process plantuml code.  
///
/// ```markdown
/// [//comment]: # (auto_plantuml start)
///
/// '''plantuml
///     @startuml
///     [Bob] ..> [Alice]
///     @enduml
/// '''
///
/// ![svg_534231](images/svg_534231.svg)  
///
/// [//comment]: # (auto_plantuml end)
/// ```
///
/// In this instructions I changed `[//]` to `[//comment]` and  ticks to single quotes to not process these markers.
///
/// Between the last triple backtick and the end marker is the processed svg file.  
/// Calculate a short hash from the plantuml code.  
/// If the name of the svg file contains this hash code it means, we already have the correct svg file.  
/// Else we have to delete the old svg file and get a new one from the modified plantuml code.  
/// Call the plantuml.com server with the plantuml code and saves the result svg file in folder images/.  
/// Add the hash code to the filename.
///
/// Process plantuml in current directory.  
/// Finds markers (auto_plantuml start) and (auto_plantuml end) in md files.  
/// If needed calls the web service and saves the svg file.  
/// Between markers adds the link to the svg file.  
/// repo_url like <https://github.com/automation-tasks-rs/sey_currency_converter_pwa>
/// So the image file link is from the repository and accessible everywhere.
///
// endregion: auto_md_to_doc_comments include doc_comments/auto_plantuml.md A ///
pub fn auto_plantuml(repo_url: &str) -> Result<()> {
    let path = std::env::current_dir().log(pos!())?;
    auto_plantuml_for_path(&path, repo_url).log(pos!())?;
    Ok(())
}

/// Process plantuml for all md files.
///
/// For test and examples I need to provide the path.
pub fn auto_plantuml_for_path(path: &std::path::Path, repo_url: &str) -> Result<()> {
    let path = camino::Utf8Path::from_path(path)
        .ok_or_else(|| Error::ErrorFromStr("from_path is None"))
        .log(pos!())?;
    println!("  {YELLOW}Running auto_plantuml{RESET}");
    //use traverse instead of glob
    let files = crate::cargo_auto_lib::utils_mod::traverse_dir_with_exclude_dir(
        path.as_std_path(),
        "/*.md",
        // avoid big folders
        &["/.git".to_string(), "/target".to_string(), "/docs".to_string()],
    )
    .log(pos!())?;
    for md_filename in files {
        let md_filename = camino::Utf8Path::new(&md_filename);

        let mut is_changed = false;
        let mut md_text_content = std::fs::read_to_string(md_filename).log(pos!())?;

        // check if file have CRLF and show error
        if md_text_content.contains("\r\n") {
            return Err(Error::ErrorFromString(format!(
                "{RED}Error: {md_filename} has CRLF line endings instead of LF. Correct the file! {RESET}"
            )));
        }
        let mut pos = 0;
        // find markers
        while let Ok(marker_start) = find_pos_start_data_after_delimiter(&md_text_content, pos, "\n[//]: # (auto_plantuml start)\n") {
            pos = marker_start + 34;
            if let Ok(code_start) = find_pos_start_data_after_delimiter(&md_text_content, marker_start, "\n```plantuml\n") {
                if let Ok(code_end) = find_pos_end_data_before_delimiter(&md_text_content, code_start, "\n```\n") {
                    let code_end_after = code_end + 5;
                    let plantuml_code = &md_text_content[code_start..code_end];
                    let plantuml_code_hash = hash_text(plantuml_code);
                    if let Ok(marker_end) =
                        find_pos_end_data_before_delimiter(&md_text_content, marker_start, "\n[//]: # (auto_plantuml end)\n")
                    {
                        let img_link = md_text_content[code_end_after..marker_end].trim();
                        let mut get_new_svg = false;
                        if img_link.is_empty() {
                            get_new_svg = true;
                        } else {
                            // parse this format ![svg_534231](images/svg_534231.svg)
                            let cap_group = REGEX_IMG_LINK
                                .captures(img_link)
                                .ok_or_else(|| Error::ErrorFromString(format!("{RED}Error: The old img link '{img_link}' is NOT in this format '![svg_534231](images/svg_534231.svg)'{RESET}"))).log(pos!())?;
                            let old_hash = &cap_group[1];
                            if old_hash != plantuml_code_hash {
                                get_new_svg = true;
                                // delete the old image file
                                let old_file_path = camino::Utf8PathBuf::from_str(&format!(
                                    "{}/images/svg_{old_hash}.svg",
                                    md_filename
                                        .parent()
                                        .ok_or_else(|| Error::ErrorFromStr("parent is None"))
                                        .log(pos!())?
                                ))
                                .log(pos!())?;
                                if old_file_path.exists() {
                                    std::fs::remove_file(&old_file_path).log(pos!())?;
                                }
                            } else {
                                // check if the svg file exists
                                let old_file_path = camino::Utf8PathBuf::from_str(&format!(
                                    "{}/images/svg_{old_hash}.svg",
                                    md_filename
                                        .parent()
                                        .ok_or_else(|| Error::ErrorFromStr("parent is None"))
                                        .log(pos!())?
                                ))
                                .log(pos!())?;
                                if !old_file_path.exists() {
                                    get_new_svg = true;
                                }
                            }
                        }
                        if get_new_svg {
                            let relative_md_filename = md_filename.strip_prefix(path).log(pos!())?;
                            println!("  {YELLOW}{relative_md_filename} get new svg {plantuml_code_hash}{RESET}");

                            // get the new svg image
                            let svg_code = request_svg(plantuml_code).log(pos!())?;
                            let new_file_path = camino::Utf8PathBuf::from_str(&format!(
                                "{}/images/svg_{plantuml_code_hash}.svg",
                                md_filename
                                    .parent()
                                    .ok_or_else(|| Error::ErrorFromStr("parent is None"))
                                    .log(pos!())?
                            ))
                            .log(pos!())?;
                            std::fs::create_dir_all(
                                new_file_path
                                    .parent()
                                    .ok_or_else(|| Error::ErrorFromStr("parent is None"))
                                    .log(pos!())?,
                            )
                            .log(pos!())?;
                            std::fs::write(&new_file_path, svg_code).log(pos!())?;
                            // if repo_url is not empty then prepare GitHub url
                            // https://github.com/automation-tasks-rs/sey_currency_converter_pwa/raw/main/
                            let repo_full_url = if repo_url.is_empty() {
                                "".to_string()
                            } else {
                                format!("{}/raw/main/", repo_url.trim_end_matches('/'))
                            };
                            // path relative to Cargo.toml (project root)
                            let relative_svg_path = new_file_path.strip_prefix(path).log(pos!())?;
                            // create the new image lnk
                            let img_link = format!("\n![svg_{plantuml_code_hash}]({repo_full_url}{relative_svg_path})\n");
                            // delete the old img_link and insert the new one
                            md_text_content.replace_range(code_end_after..marker_end, &img_link);
                            is_changed = true;
                        }
                    }
                }
            }
        }
        // if changed, then write to disk
        if is_changed {
            std::fs::write(md_filename, md_text_content).log(pos!())?;
        }
    }
    println!("  {YELLOW}Finished auto_plantuml{RESET}");
    Ok(())
}

/// Hash text.
pub fn hash_text(text: &str) -> String {
    let hash = <sha2::Sha256 as sha2::Digest>::digest(text.as_bytes());
    // base64ct = {version = "1.5.0", features = ["alloc"] }
    // return base64_hash
    <base64ct::Base64UrlUnpadded as base64ct::Encoding>::encode_string(&hash)
}

/// Request svg from plantuml server
pub fn request_svg(plant_uml_code: &str) -> Result<String> {
    let base_url = "https://plantuml.com/plantuml";
    let url_parameter = compress_plant_uml_code(plant_uml_code).log(pos!())?;
    let url = format!("{}/svg/{}", base_url, url_parameter);
    // use reqwest to GET from plantuml.com server
    // return response
    Ok(reqwest::blocking::get(url).log(pos!())?.text_with_charset("utf-8").log(pos!())?)
}

/// Deflate and strange base64, that is Url_safe
pub fn compress_plant_uml_code(plant_uml_code: &str) -> Result<String> {
    // first deflate
    let data = plant_uml_code.as_bytes();
    let compressed = deflate::deflate_bytes(data);
    // then the strange base64
    // https://plantuml.com/text-encoding
    let my_cfg = radix64::CustomConfig::with_alphabet("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_")
        .no_padding()
        .build()
        .map_err(|_| Error::ErrorFromStr("CustomConfigError"))
        .log(pos!())?;
    // return
    Ok(my_cfg.encode(&compressed))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn examples_plantuml_test() {
        // TODO: testing in windows returns different result because of / and \ tragedy.
        // similar to examples/plantuml/plantuml1.rs and check the result
        // region: prepare folders and files for this example
        // remove the 'images' folder
        std::fs::remove_dir_all("examples/plantuml/images").unwrap_or_else(|_| ());
        // copy md files from sample_data to examples
        std::fs::copy("sample_data/input1_for_plantuml.md", "examples/plantuml/input1_for_plantuml.md").expect("example");
        std::fs::copy("sample_data/input2_for_plantuml.md", "examples/plantuml/input2_for_plantuml.md").expect("example");
        // endregion: prepare folders and files for this example

        let path = camino::Utf8Path::new("examples/plantuml");
        auto_plantuml_for_path(path.as_std_path(), "").expect("example");

        // check the result
        let changed1 = std::fs::read_to_string("examples/plantuml/input1_for_plantuml.md").expect("example");
        let output1 = std::fs::read_to_string("sample_data/output1_for_plantuml.md").expect("example");
        assert_eq!(changed1, output1);

        let changed2 = std::fs::read_to_string("examples/plantuml/input2_for_plantuml.md").expect("example");
        let output2 = std::fs::read_to_string("sample_data/output2_for_plantuml.md").expect("example");
        assert_eq!(changed2, output2);

        /* cSpell:disable */
        assert!(camino::Utf8Path::new("examples/plantuml/images/svg_8eLHibrc2gjrY1qcezDiy--xk9mz1XwYyIcZwXvjlcE.svg").exists());
        assert!(camino::Utf8Path::new("examples/plantuml/images/svg_H8u0SNaGZzGAaYPHeY4eDF9TfWqVXhKa7M8wiwXSe_s.svg").exists());
        assert!(camino::Utf8Path::new("examples/plantuml/images/svg_KPAr4S3iGAVLbskqf6XXaqrWge8bXMlCkNk7EaimJs0.svg").exists());
        assert!(camino::Utf8Path::new("examples/plantuml/images/svg_lTG8S1eNgnLTJS1PruoYJEjQVW4dCn0x6Wl-pw6yPXM.svg").exists());
        assert!(camino::Utf8Path::new("examples/plantuml/images/svg_tosmzSqwSXyObaX7eRLFp9xsMzcM5UDT4NSaQSgnq-Q.svg").exists());
        /* cSpell:enable */
    }

    #[test]
    pub fn test_hash() {
        assert_eq!("n4bQgYhMfWWaL-qgxVrQFaO_TxsrC4Is0V1sFbDwCgg", hash_text("test"));
    }

    #[test]
    pub fn test_compress_plant_uml_code() {
        // http://www.plantuml.com/plantuml/uml/SoWkIImgAStDuNBCoKnELT2rKt3AJx9IS2mjoKZDAybCJYp9pCzJ24ejB4qjBk42oYde0jM05MDHLLoGdrUSokMGcfS2D1C0
        assert_eq!(
            compress_plant_uml_code(
                r#"@startuml
Alice -> Bob: Authentication Request
Bob --> Alice: Authentication Response
@enduml"#
            )
            .expect("test"),
            "SoWkIImgAStDuNBCoKnELT2rKt3AJx9IS2mjoKZDAybCJYp9pCzJ24ejB4qjBk42oYde0jM05MDHLLoGdrUSokMGcfS2D1C0"
        );
    }
}
