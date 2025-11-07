// auto_md_to_doc_comments_mod

//! Finds rs files with markers and include segments from md files as doc comments.

use glob::glob;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    cargo_auto_lib::{
        error_mod::Error,
        public_api_mod::{RED, RESET, YELLOW},
        Result,
    },
    generic_functions_mod::{pos, ResultLogError},
};

/// Markers found in rs files
#[derive(Debug)]
struct RsMarker {
    pub md_filename: String,
    pub marker_name: String,
    pub comment_symbol: String,
    pub pos_start: usize,
    pub pos_end: usize,
}

/// Markers found in md files
#[derive(Debug)]
struct MdSegment {
    pub md_filename: String,
    pub marker_name: String,
    pub pos_start: usize,
    pub pos_end: usize,
    pub text: String,
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
    let mut cache_md_segments = vec![];
    for rs_filename in rs_files().log(pos!())?.iter() {
        let mut rs_text_content = std::fs::read_to_string(rs_filename).log(pos!())?;

        // check if file have CRLF instead of LF and show error
        if rs_text_content.contains("\r\n") {
            return Err(Error::ErrorFromString(format!(
                "{RED}Error: {rs_filename} has CRLF line endings instead of LF. Correct the file! {RESET}"
            )));
        }

        let markers = rs_file_markers(&rs_text_content).log(pos!())?;
        if !markers.is_empty() {
            for marker in markers.iter().rev() {
                let segment_text = get_md_segments_using_cache(
                    &mut cache_md_segments,
                    &marker.md_filename,
                    &marker.marker_name,
                    &marker.comment_symbol,
                )
                .log(pos!())?;
                rs_text_content.replace_range(marker.pos_start..marker.pos_end, &segment_text);
            }
            println!("  {YELLOW}Write file: {rs_filename}{RESET}");
            std::fs::write(rs_filename, rs_text_content).log(pos!())?;
        }
    }
    Ok(())
}

/// All rs files in src, tests and examples folders.
fn rs_files() -> Result<Vec<String>> {
    let mut rs_files = vec![];
    // in Unix shell ** means recursive match through all the subdirectories
    for filename_result in glob("src/**/*.rs").log(pos!())? {
        let filename_pathbuff = filename_result?;
        let rs_filename = filename_pathbuff
            .to_str()
            .ok_or_else(|| Error::ErrorFromStr("filename_pathbuff is None"))
            .log(pos!())?
            .to_string();
        rs_files.push(rs_filename);
    }
    for filename_result in glob("tests/**/*.rs").log(pos!())? {
        let filename_pathbuff = filename_result?;
        let rs_filename = filename_pathbuff
            .to_str()
            .ok_or_else(|| Error::ErrorFromStr("filename_pathbuff is None"))
            .log(pos!())?
            .to_string();
        rs_files.push(rs_filename);
    }
    for filename_result in glob("examples/**/*.rs").log(pos!())? {
        let filename_pathbuff = filename_result?;
        let rs_filename = filename_pathbuff
            .to_str()
            .ok_or_else(|| Error::ErrorFromStr("filename_pathbuff is None"))
            .log(pos!())?
            .to_string();
        rs_files.push(rs_filename);
    }
    //return
    Ok(rs_files)
}

lazy_static! {
    /// Regex for start marker
    static ref REGEX_RS_START: Regex = Regex::new(r#"(?m)^ *?// region: auto_md_to_doc_comments include (.*?) (.*?) (.*?)$"#).expect("regex new");
    /// Regex for end marker
    static ref REGEX_RS_END: Regex = Regex::new(r#"(?m)^ *?// endregion: auto_md_to_doc_comments include (.*?) (.*?) (.*?)$"#).expect("regex new");
}
/// markers in rs files
fn rs_file_markers(rs_text_content: &str) -> Result<Vec<RsMarker>> {
    let mut markers = vec![];
    for cap in REGEX_RS_START.captures_iter(rs_text_content) {
        let rs_marker = RsMarker {
            md_filename: cap[1].to_string(),
            marker_name: cap[2].to_string(),
            comment_symbol: cap[3].to_string(),
            pos_start: cap
                .get(0)
                .ok_or_else(|| Error::ErrorFromStr("cap get 0 is None"))
                .log(pos!())?
                .end()
                + 1,
            pos_end: 0,
        };
        markers.push(rs_marker);
    }
    for cap in REGEX_RS_END.captures_iter(rs_text_content) {
        // TODO:  error what file and segment are problematic
        let marker = markers
            .iter_mut()
            .find(|m| m.md_filename == cap[1] && m.marker_name == cap[2])
            .ok_or_else(|| Error::ErrorFromStr("find is None"))
            .log(pos!())?;
        marker.pos_end = cap
            .get(0)
            .ok_or_else(|| Error::ErrorFromStr("cap get 0 is None"))
            .log(pos!())?
            .start();
    }
    // return
    Ok(markers)
}

lazy_static! {
    /// Regex for start marker
    static ref REGEX_MD_START: Regex = Regex::new(r#"(?m)^\[//\]: # \(auto_md_to_doc_comments segment start (.*?)\)$"#).expect("regex new");
    /// Regex for end marker
    static ref REGEX_MD_END: Regex = Regex::new(r#"(?m)^\[//\]: # \(auto_md_to_doc_comments segment end (.*?)\)$"#).expect("regex new");
}

/// The first time it is called read the file and extracts all the segments into a cache vector.
///
/// Subsequent calls read from the cache.
fn get_md_segments_using_cache(cache: &mut Vec<MdSegment>, md_filename: &str, marker_name: &str, comment_symbol: &str) -> Result<String> {
    // check the cache
    if let Some(_seg) = cache.iter().find(|m| m.md_filename == md_filename) {
        let segment = cache
            .iter()
            .find(|m| m.md_filename == md_filename && m.marker_name == marker_name)
            .ok_or_else(|| Error::ErrorFromStr("find is None"))
            .log(pos!())?;
        Ok(segment.text.to_string())
    } else {
        // process the file
        println!("  {YELLOW}Read file: {md_filename}{RESET}");
        let md_text_content = std::fs::read_to_string(md_filename).log(pos!())?;

        // check if file have CRLF instead of LF and show error
        if md_text_content.contains("\r\n") {
            return Err(Error::ErrorFromString(format!(
                "{RED}Error: {md_filename} has CRLF line endings instead of LF. Correct the file! {RESET}"
            )));
        }

        for cap in REGEX_MD_START.captures_iter(&md_text_content) {
            cache.push(MdSegment {
                md_filename: md_filename.to_owned(),
                marker_name: cap[1].to_owned(),
                pos_start: cap
                    .get(0)
                    .ok_or_else(|| Error::ErrorFromStr("cap get 0 is None"))
                    .log(pos!())?
                    .end()
                    + 1,
                pos_end: 0,
                text: String::new(),
            });
        }
        for cap in REGEX_MD_END.captures_iter(&md_text_content) {
            // TODO:  error what file and segment are problematic
            let segment = cache
                .iter_mut()
                .find(|m| m.md_filename == md_filename && m.marker_name == cap[1])
                .ok_or_else(|| Error::ErrorFromStr("find is None"))
                .log(pos!())?;
            segment.pos_end = cap
                .get(0)
                .ok_or_else(|| Error::ErrorFromStr("cap get 0 is None"))
                .log(pos!())?
                .start();
            // the segment begins with a comment, so don't include the next empty row
            let mut last_line_was_comment = true;
            for line in md_text_content[segment.pos_start..segment.pos_end].lines() {
                if line.starts_with("[//]: # (") {
                    // don't include md comments
                    last_line_was_comment = true;
                } else if last_line_was_comment && line.is_empty() {
                    // don't include empty line after md comments
                    last_line_was_comment = false;
                } else {
                    last_line_was_comment = false;
                    segment.text.push_str(comment_symbol);
                    if !line.is_empty() {
                        segment.text.push(' ');
                    }
                    segment.text.push_str(line);
                    segment.text.push('\n');
                }
            }
        }
        // TODO:  error what file and segment are problematic
        let segment = cache
            .iter()
            .find(|m| m.md_filename == md_filename && m.marker_name == marker_name)
            .ok_or_else(|| Error::ErrorFromStr("find is None"))
            .log(pos!())?;
        //return
        Ok(segment.text.to_string())
    }
}
