// utils_mod.rs

//! Functions for various utilities.

use crate::{
    cargo_auto_lib::error_mod::{Error, Result},
    generic_functions_mod::{pos, ResultLogError},
};

// region: delimiters cannot be INACTIVE like markers

/// Position of start of the delimited data after the delimiter
pub fn find_pos_start_data_after_delimiter(md_text_content: &str, pos: usize, delimiter: &str) -> Result<usize> {
    if let Ok(pos_start_data) = find_from(md_text_content, pos, delimiter) {
        let pos_start_data = pos_start_data + delimiter.len();
        return Ok(pos_start_data);
    }
    // return
    Err(Error::ErrorFromStr("not found"))
}

/// Position of end of the delimited data before the delimiter
pub fn find_pos_end_data_before_delimiter(md_text_content: &str, pos: usize, delimiter: &str) -> Result<usize> {
    if let Ok(pos_end_data) = find_from(md_text_content, pos, delimiter) {
        return Ok(pos_end_data);
    }
    //return
    Err(Error::ErrorFromStr("not found"))
}

// endregion: delimiters cannot be INACTIVE like markers

/// Find from pos
pub fn find_from(text: &str, from_pos: usize, find: &str) -> Result<usize> {
    let slice01 = text
        .get(from_pos..)
        .ok_or_else(|| Error::ErrorFromStr("text get is None"))
        .log(pos!())?;
    let option_location = slice01.find(find);
    if let Some(location) = option_location {
        // return Ok with usize
        Ok(from_pos + location)
    } else {
        // return Err
        Err(Error::ErrorFromStr("location is not find"))
    }
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
    // if the parameter is /*.rs, I can eliminate /*
    let find_file = &find_file.replace("/*", "");

    let mut v = Vec::new();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir).log(pos!())? {
            let entry = entry?;
            let path = entry.path();
            let str_path = path.to_str().ok_or_else(|| Error::ErrorFromStr("path is None")).log(pos!())?;
            if path.is_dir() {
                let mut is_excluded = false;
                for excl in exclude_dirs {
                    if str_path.ends_with(excl) {
                        is_excluded = true;
                        break;
                    }
                }
                if !is_excluded {
                    let mut sub_v = traverse_dir_with_exclude_dir(&path, find_file, exclude_dirs).log(pos!())?;
                    v.append(&mut sub_v);
                }
            } else if str_path.ends_with(find_file) {
                v.push(str_path.to_string());
            }
        }
    }
    Ok(v)
}

/// The original `concat()` function does not have a delimiter.
pub fn concatenate_vec_to_string(vec: &[String], delimiter: &str) -> String {
    let size = vec.iter().fold(0, |a, b| a + b.len());
    let mut concatenated_string = String::with_capacity(size);
    for (i, item) in vec.iter().enumerate() {
        if i > 0 {
            concatenated_string.push_str(delimiter);
        }
        concatenated_string.push_str(item);
    }
    // return
    concatenated_string
}

/// UTC date in iso standard like 2024-12-31.
pub fn now_utc_date_iso() -> String {
    chrono::Utc::now().format("%Y-%m-%d").to_string()
}
