// file_hashes_mod.rs

use serde_derive::{Deserialize, Serialize};

// region: structs
/// file metadata
#[derive(Serialize, Deserialize)]
pub struct FileMetaData {
    /// filename with path from Cargo.toml folder
    filename: String,
    /// hash of file
    filehash: String,
}

/// the struct that represents the file automation_tasks_rs/._file_hashes.json
#[derive(Serialize, Deserialize)]
pub struct FileHashes {
    /// vector of file metadata
    pub vec_file_metadata: Vec<FileMetaData>,
}

// endregion: structs

/// check if the files are modified and in automation_tasks_rs
/// The modified date of files is not usable when using git.
/// The checkout will make dates newer than they really are.
/// I should use a hash of files and write them in the same directory for later comparison.
pub fn is_project_changed() -> bool {
    let vec_of_metadata = read_file_metadata();
    let js_struct = read_json_file(&crate::PATH_FILE_HASHES_JSON.to_string_lossy());

    if are_all_files_equal(&vec_of_metadata, &js_struct.vec_file_metadata) {
        false
    } else {
        true
    }
}

fn are_all_files_equal(vec_of_metadata: &Vec<FileMetaData>, js_vec_of_metadata: &Vec<FileMetaData>) -> bool {
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

/// make a vector of file metadata
pub fn read_file_metadata() -> Vec<FileMetaData> {
    let mut vec_of_metadata: Vec<FileMetaData> = Vec::new();

    // calculate hash of file
    let filehash = sha256_digest(&crate::PATH_CARGO_TOML).unwrap();
    vec_of_metadata.push(FileMetaData {
        filename: crate::PATH_CARGO_TOML.to_string_lossy().to_string(),
        filehash,
    });

    // calculate hash of file
    let filehash = sha256_digest(&crate::PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS).unwrap();
    vec_of_metadata.push(FileMetaData {
        filename: crate::PATH_TARGET_DEBUG_AUTOMATION_TASKS_RS
            .to_string_lossy()
            .to_string(),
        filehash,
    });

    for entry in std::fs::read_dir(crate::PATH_SRC.as_path()).unwrap() {
        let entry = entry.unwrap();
        let path = entry.file_name();

        let filename = format!("{}/{}", crate::PATH_SRC.to_string_lossy(), path.to_string_lossy());
        let filename = filename.replace("\"", "");
        // calculate hash of file
        let filehash = sha256_digest(std::path::Path::new(&filename)).unwrap();
        vec_of_metadata.push(FileMetaData { filename, filehash });
    }
    vec_of_metadata
}

/// read automation_tasks_rs/.file_hashes.json
fn read_json_file(json_filepath: &str) -> FileHashes {
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
                js_struct = serde_json::from_str(x.as_str()).unwrap();
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
    js_struct
}

/// calculate the hash for a file
fn sha256_digest(path: &std::path::Path) -> anyhow::Result<String> {
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let mut context = ring::digest::Context::new(&ring::digest::SHA256);
    let mut buffer = [0; 1024];
    use std::io::Read;
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    let digest = context.finish();
    let hash_string = data_encoding::HEXLOWER.encode(digest.as_ref());
    // return
    Ok(hash_string)
}

/// save the new file metadata
pub fn save_json_file_for_file_meta_data(vec_of_metadata: Vec<FileMetaData>) {
    let x = FileHashes {
        vec_file_metadata: vec_of_metadata,
    };
    let y = serde_json::to_string_pretty(&x).unwrap();
    let json_filepath = crate::PATH_FILE_HASHES_JSON.as_path();
    let _f = std::fs::write(json_filepath, y);
}
