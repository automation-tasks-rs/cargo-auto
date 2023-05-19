// copy_files_to_strings_mod.rs

/// copy all files from the folder into a module as strings (static &str)
/// the module has the markers: region: files copied into strings by automation tasks and endregion:
/// the string will be in a vector with the file name
/// first we create the complete text, then we check if the old text needs to be replaced
pub fn copy_folder_files_into_module(folder_path: &std::path::Path, module_path: &std::path::Path) {
    println!("copy_folder_files_into_module {}, {}", folder_path.to_string_lossy(), module_path.to_string_lossy() );
    // traverse and get all file_names
    let files = cargo_auto_lib::utils_mod::traverse_dir_with_exclude_dir(
        &folder_path,
        "",
        // avoid big folders and other folders
        &vec!["/.git".to_string(), "/target".to_string(), "/docs".to_string()],
    )
    .unwrap();
    let mut new_code = String::new();
    for file_name in files.iter() {
        let file_name_short = file_name.trim_start_matches(&format!("{}/",folder_path.to_string_lossy()));
        // avoid Cargo.lock file
        if file_name_short=="Cargo.lock"{
            continue;
        }
        let file_content = std::fs::read_to_string(&file_name).unwrap();
        new_code.push_str(&format!(r####"vec_file.push(crate::FileItem{{
            file_name :"{}",
            file_content : r###"{}"###,
}});    
"####, &file_name_short, &file_content));
    }

    // read the content of the module, delimited by markers
    let module_content =  std::fs::read_to_string(module_path).unwrap();
    let start_pos = find_pos_start_data_after_delimiter(&module_content, 0, "// region: files copied into strings by automation tasks\n").expect("didn't find // region: files copied..");
    let end_pos = find_pos_end_data_before_delimiter(&module_content, 0, "// endregion: files copied into strings by automation tasks").expect("didn't find // endregion: files copied..");
    let old_code = &module_content[start_pos..end_pos];

    // compare the text, if different replace
    if old_code != new_code{       
        let mut new_module_content = String::new();
        new_module_content.push_str(&module_content[..start_pos]);
        new_module_content.push_str(&new_code);
        new_module_content.push_str(&module_content[end_pos..]);
        std::fs::write(module_path, &new_module_content).unwrap();  
    }
}

/// return the position of start of the delimited data after the delimiter
pub fn find_pos_start_data_after_delimiter(
    md_text_content: &str,
    pos: usize,
    delimiter: &str,
) -> Option<usize> {
    if let Some(pos_start_data) = find_from(md_text_content, pos, delimiter) {
        let pos_start_data = pos_start_data + delimiter.len();
        return Some(pos_start_data);
    }
    // return
    None
}

/// return the position of end of the delimited data before the delimiter
pub fn find_pos_end_data_before_delimiter(
    md_text_content: &str,
    pos: usize,
    delimiter: &str,
) -> Option<usize> {
    if let Some(pos_end_data) = find_from(md_text_content, pos, delimiter) {
        return Some(pos_end_data);
    }
    //return
    None
}

/// find from_pos
pub fn find_from(text: &str, from_pos: usize, find: &str) -> Option<usize> {
    let slice01 = text.get(from_pos..).unwrap();
    let option_location = slice01.find(find);
    if let Some(location) = option_location {
        // return Option with usize
        Some(from_pos + location)
    } else {
        // return Option with none
        option_location
    }
}

