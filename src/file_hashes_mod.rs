// file_hashes_mod.rs

//! Calculate file hashes.
//!
//! File hashes are used to check if some file has changed.
//! Then we can run commands like compile only if a file has changed.

use serde_derive::{Deserialize, Serialize};
use sha2::Digest;

// region: structs
/// Struct with file metadata.
#[derive(Serialize, Deserialize)]
pub struct FileMetaData {
    /// filename with path from Cargo.toml folder
    filename: String,
    /// hash of file
    filehash: String,
}

/// The struct represents the file automation_tasks_rs/._file_hashes.json.
#[derive(Serialize, Deserialize)]
pub struct FileHashes {
    /// vector of file metadata
    pub vec_file_metadata: Vec<FileMetaData>,
}

// endregion: structs

/// Check if the files are modified in automation_tasks_rs.
///
/// The modified date of files is not usable when using git.  
/// The checkout will make dates newer than they really are.  
/// I should use a hash of files and write them in the same directory for later comparison.  
pub fn is_project_changed() -> anyhow::Result<bool> {
    let vec_of_metadata = read_file_metadata()?;
    let js_struct = read_json_file(&crate::PATH_FILE_HASHES_JSON.to_string_lossy())?;
    // return true or false
    Ok(!are_all_files_equal(&vec_of_metadata, &js_struct.vec_file_metadata))
}

/// Check if all files are equal.
fn are_all_files_equal(vec_of_metadata: &[FileMetaData], js_vec_of_metadata: &[FileMetaData]) -> bool {
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

/// Make a vector of file metadata.
pub fn read_file_metadata() -> anyhow::Result<Vec<FileMetaData>> {
    let mut vec_of_metadata: Vec<FileMetaData> = Vec::new();

    // calculate hash of Cargo.toml
    let filehash = sha256_digest(&crate::PATH_CARGO_TOML)?;
    vec_of_metadata.push(FileMetaData {
        filename: crate::PATH_CARGO_TOML.to_string_lossy().to_string(),
        filehash,
    });

    // calculate hash of file of the executable file
    let filehash = sha256_digest(&crate::PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS)?;
    vec_of_metadata.push(FileMetaData {
        filename: crate::PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS.to_string_lossy().to_string(),
        filehash,
    });

    // all files in the src/ directory
    for entry in walkdir::WalkDir::new(crate::PATH_SRC.as_path()).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            let path = entry.path();
            // calculate hash of file
            let filehash = sha256_digest(path)?;
            vec_of_metadata.push(FileMetaData {
                filename: path.to_string_lossy().to_string(),
                filehash,
            });
        }
    }
    Ok(vec_of_metadata)
}

/// Read automation_tasks_rs/.file_hashes.json.
fn read_json_file(json_filepath: &str) -> anyhow::Result<FileHashes> {
    let js_struct: FileHashes;
    let f = std::fs::read_to_string(json_filepath);

    match f {
        Ok(x) => {
            // check if file have CRLF instead of LF. This are unusable - create empty struct
            if x.contains("\r\n") {
                //create empty struct
                js_struct = FileHashes {
                    vec_file_metadata: Vec::new(),
                }
            } else {
                //read struct from file
                js_struct = serde_json::from_str(x.as_str())?;
            }
        }
        Err(_error) => {
            // println!("Creating new file: {}", json_filepath);
            //create empty struct
            js_struct = FileHashes {
                vec_file_metadata: Vec::new(),
            }
        }
    };
    Ok(js_struct)
}

/// Calculate the hash for a file.
fn sha256_digest(path: &std::path::Path) -> anyhow::Result<String> {
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let mut hasher = <sha2::Sha256 as sha2::Digest>::new();
    let mut buffer = [0; 1024];
    use std::io::Read;
    loop {
        let count = reader.read(&mut buffer)?;
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

/// Save the new file metadata.
pub fn save_json_file_for_file_meta_data(vec_of_metadata: Vec<FileMetaData>) -> anyhow::Result<()> {
    let x = FileHashes {
        vec_file_metadata: vec_of_metadata,
    };
    let y = serde_json::to_string_pretty(&x)?;
    let json_filepath = crate::PATH_FILE_HASHES_JSON.as_path();
    let _f = std::fs::write(json_filepath, y);
    Ok(())
}
