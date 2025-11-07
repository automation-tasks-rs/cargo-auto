// auto_semver_mod

//! Semver utilities.

use crate::cargo_auto_lib::public_api_mod::{GREEN, RED, RESET, YELLOW};
use crate::cargo_auto_lib::{
    error_mod::{Error, Result},
    utils_mod::*,
};
use crate::generic_functions_mod::{pos, ResultLogError};

/// Enum for version parts: Minor or Patch
enum VersionPart {
    Patch,
    Minor,
}

/// Increment the patch version in Cargo.toml file only if files are changed.
pub fn auto_semver_increment_patch() -> Result<()> {
    increment_part(VersionPart::Patch, false)
}

/// Increment the patch version in Cargo.toml file even if files are not changed.
pub fn auto_semver_increment_patch_forced() -> Result<()> {
    increment_part(VersionPart::Patch, true)
}

/// Increment the minor version in Cargo.toml file only if files are changed.
pub fn auto_semver_increment_minor() -> Result<()> {
    increment_part(VersionPart::Minor, false)
}

/// Increment the minor version in Cargo.toml file even if files are not changed.
pub fn auto_semver_increment_minor_forced() -> Result<()> {
    increment_part(VersionPart::Minor, true)
}

/// Increment a part of version in Cargo.toml file even if files are not changed or forced
fn increment_part(part: VersionPart, force_version: bool) -> Result<()> {
    let mut vec_of_metadata = crate::cargo_auto_lib::auto_version_from_date_mod::read_file_metadata().log(pos!())?;
    let is_files_equal = if force_version {
        false
    } else {
        let js_struct =
            crate::cargo_auto_lib::auto_version_from_date_mod::read_json_file(".automation_tasks_rs_file_hashes.json").log(pos!())?;
        crate::cargo_auto_lib::auto_version_from_date_mod::are_files_equal(&vec_of_metadata, &js_struct.vec_file_metadata)
    };

    if !is_files_equal {
        // println!("pub fn increment_patch");
        let cargo_toml_filename = "Cargo.toml";
        let cargo_toml_text = std::fs::read_to_string(cargo_toml_filename).log(pos!())?;

        // check if file have CRLF instead of LF and show error
        if cargo_toml_text.contains("\r\n") {
            return Err(Error::ErrorFromString(format!(
                "{RED}Error: {cargo_toml_filename} has CRLF line endings instead of LF. Correct the file! {RESET}"
            )));
        }

        // find the line with "version = " including the start quote
        if let Ok(pos_start_data) = find_pos_start_data_after_delimiter(&cargo_toml_text, 0, r#"version = ""#) {
            // find the end quote
            if let Ok(pos_end_data) = find_pos_end_data_before_delimiter(&cargo_toml_text, pos_start_data, r#"""#) {
                let version = cargo_toml_text[pos_start_data..pos_end_data].to_string();
                println!(r#"  {YELLOW}old version: "{version}"{RESET}"#);
                //increment the last number
                let pos = pos_start_data;
                let (major, pos) = parse_next_number(&cargo_toml_text, pos).log(pos!())?;
                //jump over dot
                let pos = pos + 1;
                let (mut minor, pos) = parse_next_number(&cargo_toml_text, pos).log(pos!())?;
                //jump over dot
                let pos = pos + 1;
                let (mut patch, pos) = parse_next_number(&cargo_toml_text, pos).log(pos!())?;
                let pos_at_the_end_of_semver = pos;
                // increment
                match part {
                    VersionPart::Patch => {
                        patch += 1;
                    }
                    VersionPart::Minor => {
                        minor += 1;
                        patch = 0;
                    }
                }
                // println!(r#"major: {},minor: {}, patch: {}"#, major, minor, patch);
                let new_semver = format!("{}.{}.{}", major, minor, patch);
                println!("{GREEN}new version: '{}'{RESET}", &new_semver);
                let new_cargo_toml_text = format!(
                    "{}{}{}",
                    &cargo_toml_text[..pos_start_data],
                    &new_semver,
                    &cargo_toml_text[pos_at_the_end_of_semver..]
                );
                //save the file
                let _x = std::fs::write(cargo_toml_filename, new_cargo_toml_text);

                //the Cargo.toml is now different
                crate::cargo_auto_lib::auto_version_from_date_mod::correct_file_metadata_for_cargo_tom_inside_vec(&mut vec_of_metadata)
                    .log(pos!())?;
                crate::cargo_auto_lib::auto_version_from_date_mod::save_json_file_for_file_meta_data(vec_of_metadata).log(pos!())?;
            } else {
                return Err(Error::ErrorFromString(format!("{RED}no end quote for version{RESET}")));
            }
        } else {
            return Err(Error::ErrorFromString(format!("{RED}Cargo.toml has no version{RESET}")));
        }
    }
    Ok(())
}

/// Parse next number in version
fn parse_next_number(cargo_toml_text: &str, pos: usize) -> Result<(usize, usize)> {
    let mut pos = pos;
    let mut number = "".to_string();
    let mut one_char = cargo_toml_text[pos..pos + 1]
        .chars()
        .next()
        .ok_or(Error::ErrorFromStr("error chars().next()"))
        .log(pos!())?;
    while one_char.is_numeric() {
        number.push(one_char);
        pos += 1;
        one_char = cargo_toml_text[pos..pos + 1]
            .chars()
            .next()
            .ok_or(Error::ErrorFromStr("error chars().next()"))
            .log(pos!())?;
    }
    let number: usize = number.parse().log(pos!())?;
    //return
    Ok((number, pos))
}
