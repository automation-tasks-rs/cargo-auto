// auto_copy_files_to_strings_mod.rs

//! Copy all files from the folder into a module as strings (static &str).

// trait must be in scope
use base64ct::Encoding;

use crate::cargo_auto_lib::error_mod::{Error, Result};
use crate::cargo_auto_lib::public_api_mod::{RESET, YELLOW};
use crate::generic_functions_mod::{pos, ResultLogError};

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
    let folder_path = camino::Utf8Path::from_path(folder_path)
        .ok_or_else(|| Error::ErrorFromStr("folder_path is None"))
        .log(pos!())?;
    let module_path = camino::Utf8Path::from_path(module_path)
        .ok_or_else(|| Error::ErrorFromStr("module_path is None"))
        .log(pos!())?;

    println!("  {YELLOW}copy_folder_files_into_module {folder_path}, {module_path}{RESET}");
    // traverse and get all file_names
    let files = crate::cargo_auto_lib::traverse_dir_with_exclude_dir(folder_path.as_std_path(), "", exclude_big_folders).log(pos!())?;
    let mut new_code = String::new();
    for file_name in files.iter() {
        let file_name_short = file_name.trim_start_matches(&format!("{folder_path}/"));
        // avoid Cargo.lock file
        if file_name_short == "Cargo.lock" {
            continue;
        }
        // let the user define in an input parameter what files are binaries and not text.
        let mut is_binary_file = false;
        for x in ext_for_binary_files.iter() {
            if file_name_short.ends_with(x) {
                is_binary_file = true;
                break;
            }
        }

        let file_content = if is_binary_file {
            // convert binary file to base64
            let b = std::fs::read(file_name).log(pos!())?;
            base64ct::Base64::encode_string(&b)
        } else {
            // all others are text files
            std::fs::read_to_string(file_name).log(pos!())?
        };

        new_code.push_str(&format!(
            r####"vec_file.push(crate::cargo_auto_lib::FileItem{{
            file_name :"{}",
            file_content : r###"{}"###,
}});    
"####,
            &file_name_short, &file_content
        ));
    }

    // read the content of the module, delimited by markers
    let module_content = std::fs::read_to_string(module_path).log(pos!())?;
    let start_pos = crate::cargo_auto_lib::find_pos_start_data_after_delimiter(
        &module_content,
        0,
        "// region: files copied into strings by automation tasks\n",
    )
    .expect("didn't find // region: files copied..");
    let end_pos = crate::cargo_auto_lib::find_pos_end_data_before_delimiter(
        &module_content,
        0,
        "// endregion: files copied into strings by automation tasks",
    )
    .expect("didn't find // endregion: files copied..");
    let old_code = &module_content[start_pos..end_pos];

    // compare the text, if different replace
    if old_code != new_code {
        let mut new_module_content = String::new();
        new_module_content.push_str(&module_content[..start_pos]);
        new_module_content.push_str(&new_code);
        new_module_content.push_str(&module_content[end_pos..]);
        std::fs::write(module_path, &new_module_content).log(pos!())?;
    }
    Ok(())
}
