// copy_files_to_strings_mod.rs


/// copy all files from the folder into a module as strings
/// the module has the markers: region: files copied into strings by automation tasks and endregion:
/// the name of the strings are equal to the path of the file. Double underscore is replaced with slash.
/// first we create the complete text, then we check if the old text needs to be replaced
pub fn copy_folder_files_into_module(folder_path:std::path::Path, module_path: std::path::Path){
    // traverse and get all file_names
     let files = cargo_auto_lib::utils_mod::traverse_dir_with_exclude_dir(
        folder_path,
     "/*.rs",
     // avoid big folders and other folders
     &vec![
         "/.git".to_string(),
         "/target".to_string(),
         "/docs".to_string()
     ]
 ).unwrap();
 for file_name in files.iter() {
     println!("{}", &file_name);
 }
}
