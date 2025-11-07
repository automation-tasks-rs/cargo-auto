// auto_version_from_date_mod

//! The new version as date is written to Cargo.toml and service_worker.js.

// region: use statements

use crate::cargo_auto_lib::error_mod::{Error, Result};
use crate::cargo_auto_lib::public_api_mod::{RED, RESET, YELLOW};
use crate::generic_functions_mod::{pos, ResultLogError};
use chrono::DateTime;
use chrono::Timelike;
use chrono::{Datelike, Utc};
use serde_derive::{Deserialize, Serialize};
use sha2::Digest;
use std::str::FromStr;

// endregion: use statements

// region: structs
/// File metadata
#[derive(Serialize, Deserialize)]
pub struct FileMetaData {
    /// Filename with path from Cargo.toml folder
    filename: String,
    /// Hash of file content
    filehash: String,
}

/// The struct that represents the file .automation_tasks_rs_file_hashes.json
#[derive(Serialize, Deserialize)]
pub struct AutoVersionFromDate {
    /// Vector of file metadata
    pub vec_file_metadata: Vec<FileMetaData>,
}

// endregion: structs

// region: public functions

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
    auto_version_from_date_internal(false)
}

/// Just like auto_version_from_date(), but force the new version even if no files are changed.
///
/// It is slower, but easier to understand when deployed.
pub fn auto_version_from_date_forced() -> Result<()> {
    auto_version_from_date_internal(true)
}

// endregion: public functions

// region: private functions

/// Internal function to get version from date
fn auto_version_from_date_internal(force_version: bool) -> Result<()> {
    let date = Utc::now();
    let new_version = version_from_date(date);
    let vec_of_metadata = read_file_metadata().log(pos!())?;
    let is_files_equal = if force_version {
        false
    } else {
        let js_struct = read_json_file(".automation_tasks_rs_file_hashes.json").log(pos!())?;
        are_files_equal(&vec_of_metadata, &js_struct.vec_file_metadata)
    };

    if !is_files_equal {
        write_version_to_cargo_and_modify_metadata(&new_version, vec_of_metadata).log(pos!())?;
    }
    modify_service_js(&new_version).log(pos!())?;
    Ok(())
}

/// Search for file service_worker.js and modify version
fn modify_service_js(new_version: &str) -> Result<()> {
    let start_dir = camino::Utf8Path::new("./");
    for js_filename in &crate::cargo_auto_lib::utils_mod::traverse_dir_with_exclude_dir(
        start_dir.as_std_path(),
        "/service_worker.js",
        &["/.git".to_string(), "/target".to_string()],
    )
    .log(pos!())?
    {
        // println!("{}write version in {}{}", *GREEN, js_filename, *RESET);
        let mut js_content = std::fs::read_to_string(js_filename).log(pos!())?;

        // check if file have CRLF instead of LF and show error
        if js_content.contains("\r\n") {
            return Err(Error::ErrorFromString(format!(
                "{RED}Error: {js_filename} has CRLF line endings instead of LF. Correct the file! {RESET}"
            )));
        }

        let delimiter = r#"const CACHE_NAME = '"#;
        let delimiter_len = delimiter.len();
        let option_location = js_content.find(delimiter);
        if let Some(location) = option_location {
            let start_version = location + delimiter_len;
            let option_end_quote = find_from(js_content.as_str(), start_version, r#"';"#);
            if let Ok(end_version) = option_end_quote {
                //delete all the characters in between the markers
                let old_version: String = js_content.drain(start_version..end_version).collect();
                //println!(r#"old version: "{}""#, old_version.as_str());
                if new_version != old_version {
                    println!("  {YELLOW}Modify version: {old_version} -> {new_version}{RESET}");
                    js_content.insert_str(start_version, new_version);
                    //println!("{}write file: {}{}", *YELLOW, js_filename, *RESET);
                    let _x = std::fs::write(js_filename, js_content);
                }
            } else {
                return Err(Error::ErrorFromString(format!("{RED}no end quote for version{RESET}")));
            }
        } else {
            return Err(Error::ErrorFromString(format!("{RED}service_worker.js has no version{RESET}")));
        }
    }
    Ok(())
}

/// Write version to Cargo.toml
fn write_version_to_cargo_and_modify_metadata(new_version: &str, mut vec_of_metadata: Vec<FileMetaData>) -> Result<()> {
    // println!("{}write version to Cargo.toml{}", *GREEN, *RESET);
    let cargo_filename = "Cargo.toml";
    let mut cargo_content = std::fs::read_to_string(cargo_filename).log(pos!())?;

    // check if file have CRLF instead of LF and show error
    if cargo_content.contains("\r\n") {
        return Err(Error::ErrorFromString(format!(
            "{RED}Error: {} has CRLF line endings instead of LF. Correct the file! {RESET}",
            cargo_filename
        )));
    }

    let delimiter = r#"version = ""#;
    let delimiter_len = delimiter.len();
    let option_location = cargo_content.find(delimiter);
    if let Some(location) = option_location {
        let start_version = location + delimiter_len;
        let option_end_quote = find_from(cargo_content.as_str(), start_version, r#"""#);
        if let Ok(end_version) = option_end_quote {
            //delete all the characters in between the markers
            let old_version: String = cargo_content.drain(start_version..end_version).collect();
            //println!(r#"old version: "{}""#, old_version.as_str());
            if new_version != old_version.as_str() {
                println!("  {YELLOW}Modify version: {old_version} -> {new_version}{RESET}");
                cargo_content.insert_str(start_version, new_version);
                // println!("{}write file: {}{}", *YELLOW, cargo_filename, *RESET);
                let _x = std::fs::write(cargo_filename, cargo_content);

                //the Cargo.toml is now different
                correct_file_metadata_for_cargo_tom_inside_vec(&mut vec_of_metadata).log(pos!())?;
                save_json_file_for_file_meta_data(vec_of_metadata).log(pos!())?;
            }
        } else {
            return Err(Error::ErrorFromString(format!("{RED}no end quote for version{RESET}")));
        }
    } else {
        return Err(Error::ErrorFromString(format!("{RED}Cargo.toml has no version{RESET}")));
    }
    Ok(())
}

/// Cargo.toml is now different and needs to be changed in the vec of file metadata
pub fn correct_file_metadata_for_cargo_tom_inside_vec(vec_of_metadata: &mut [FileMetaData]) -> Result<()> {
    //correct the vector only for Cargo.toml file
    let filename = "Cargo.toml".to_string();
    // calculate hash of file
    let filehash = sha256_digest(std::path::PathBuf::from_str(&filename).log(pos!())?.as_path()).log(pos!())?;
    vec_of_metadata
        .get_mut(0)
        .ok_or(Error::ErrorFromStr("error vec_of_metadata.get_mut(0)"))
        .log(pos!())?
        .filehash = filehash;
    Ok(())
}

/// If files are added or deleted, other files must be also changed
///
/// Check if the files on the filesystem are the same as in the json.
pub fn are_files_equal(vec_of_metadata: &[FileMetaData], js_vec_of_metadata: &[FileMetaData]) -> bool {
    let mut is_files_equal = true;
    for x in vec_of_metadata.iter() {
        //search in json file
        let mut is_one_equal = false;
        for y in js_vec_of_metadata.iter() {
            if x.filename == y.filename && x.filehash == y.filehash {
                is_one_equal = true;
                break;
            }
        }
        if !is_one_equal {
            // println!("{} {}", x.filename, x.filehash);
            is_files_equal = false;
            break;
        }
    }
    is_files_equal
}

/// Make a vector of file metadata
pub fn read_file_metadata() -> Result<Vec<FileMetaData>> {
    let mut vec_of_metadata: Vec<FileMetaData> = Vec::new();
    let filename = "Cargo.toml".to_string();
    // calculate hash of file
    let filehash = sha256_digest(std::path::PathBuf::from_str(&filename).log(pos!())?.as_path()).log(pos!())?;
    vec_of_metadata.push(FileMetaData { filename, filehash });

    let files_paths = crate::cargo_auto_lib::utils_mod::traverse_dir_with_exclude_dir(
        camino::Utf8Path::new("src").as_std_path(),
        "/*.rs",
        // avoid big folders
        &[],
    )
    .log(pos!())?;

    for filename in files_paths {
        // calculate hash of file
        let filehash = sha256_digest(std::path::PathBuf::from_str(&filename).log(pos!())?.as_path()).log(pos!())?;
        vec_of_metadata.push(FileMetaData { filename, filehash });
    }
    Ok(vec_of_metadata)
}

/// Calculate the hash for the content of a file
fn sha256_digest(path: &std::path::Path) -> Result<String> {
    let file = std::fs::File::open(path).log(pos!())?;
    let mut reader = std::io::BufReader::new(file);
    // let mut context = ring::digest::Context::new(&ring::digest::SHA256);
    let mut hasher = sha2::Sha256::new();

    let mut buffer = [0; 1024];
    use std::io::Read;
    loop {
        let count = reader.read(&mut buffer).log(pos!())?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    let digest = hasher.finalize();
    let hash_string = data_encoding::HEXLOWER.encode(digest.as_ref());
    // return
    Ok(hash_string)
}

/// Read .automation_tasks_rs_file_hashes.json
pub fn read_json_file(json_filepath: &str) -> Result<AutoVersionFromDate> {
    let js_struct: AutoVersionFromDate;
    let f = std::fs::read_to_string(json_filepath);

    match f {
        Ok(x) => {
            // check if file have CRLF instead of LF. This are unusable - create empty struct
            if x.contains("\r\n") {
                //create empty struct
                js_struct = AutoVersionFromDate {
                    vec_file_metadata: Vec::new(),
                }
            } else {
                //read struct from file
                js_struct = serde_json::from_str(x.as_str()).log(pos!())?;
            }
        }
        Err(_error) => {
            // println!("Creating new file: {}", json_filepath);
            //create empty struct
            js_struct = AutoVersionFromDate {
                vec_file_metadata: Vec::new(),
            }
        }
    };
    Ok(js_struct)
}

/// Save the new file metadata
pub fn save_json_file_for_file_meta_data(vec_of_metadata: Vec<FileMetaData>) -> Result<()> {
    let x = AutoVersionFromDate {
        vec_file_metadata: vec_of_metadata,
    };
    let y = serde_json::to_string_pretty(&x).log(pos!())?;
    let json_filepath = ".automation_tasks_rs_file_hashes.json";
    let _f = std::fs::write(json_filepath, y);
    Ok(())
}

/// Convert a date to a version
fn version_from_date(date: DateTime<chrono::Utc>) -> String {
    // in Rust the version must not begin with zero.
    // There is an exceptional situation where is midnight 00.
    //return
    if date.hour() == 0 {
        format!("{:04}.{}{:02}.{}", date.year(), date.month(), date.day(), date.minute())
    } else {
        format!(
            "{:04}.{}{:02}.{}{:02}",
            date.year(),
            date.month(),
            date.day(),
            date.hour(),
            date.minute()
        )
    }
}

/// Find from position in string
fn find_from(rs_content: &str, from: usize, find: &str) -> Result<usize> {
    let slice01 = rs_content
        .get(from..)
        .ok_or_else(|| Error::ErrorFromStr("get from is None"))
        .log(pos!())?;
    let option_location = slice01.find(find);
    if let Some(location) = option_location {
        //return Ok with usize
        Ok(from + location)
    } else {
        //return Error
        Err(Error::ErrorFromStr("location not found"))
    }
}

// endregion: private functions

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_date_to_version() {
        let date_time = chrono::TimeZone::with_ymd_and_hms(&Utc, 2020, 5, 22, 00, 34, 0).unwrap();

        let version = version_from_date(date_time);
        assert_eq!(version, "2020.522.34");
    }

    #[test]
    pub fn test_sha256_digest() -> Result<()> {
        let digest = sha256_digest(camino::Utf8Path::new("LICENSE").as_std_path()).log(pos!())?;
        let hash_string = data_encoding::HEXLOWER.encode(digest.as_ref());
        let expected_hex = "66343964363936663834636237373465396336653537646333646433633537386532643333623130613539663837326634383134373337386462303038653035";
        assert_eq!(&hash_string, expected_hex);
        Ok(())
    }
}
