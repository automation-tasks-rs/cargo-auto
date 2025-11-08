// auto_semver_or_date_mod.rs

//! If the major number is greater than 2000, it is a date-version else it is semver.

use crate::cargo_auto_lib::error_mod::Result;
use crate::cargo_auto_lib::public_api_mod::{RESET, YELLOW};

// this trait must be in scope to use these methods of CargoToml
use crate::cargo_auto_lib::public_api_mod::CargoTomlPublicApiMethods;
use crate::utils_mod::{pos, ResultLogError};

/// Increment the version in Cargo.toml.
///
/// If the major version is greater than 2000, it is a date version  
/// else it is semver and increments the patch part.
pub fn auto_version_increment_semver_or_date() -> Result<()> {
    println!("  {YELLOW}Running auto_semver_or_date{RESET}");
    let cargo_toml = crate::cargo_auto_lib::auto_cargo_toml_mod::CargoToml::read().log(pos!())?;
    let version = cargo_toml.package_version();
    let version = semver::Version::parse(&version).log(pos!())?;
    if version.major > 2000 {
        crate::cargo_auto_lib::auto_version_from_date_mod::auto_version_from_date().log(pos!())?;
    } else {
        crate::cargo_auto_lib::auto_semver_mod::auto_semver_increment_patch().log(pos!())?;
    }
    println!("  {YELLOW}Finished auto_semver_or_date{RESET}");
    Ok(())
}

/// Increment the version in Cargo.toml, forced.
///
/// If the major version is greater than 2000, it is a date version
/// else it is semver and increments the patch part.
/// Forced is used in workspaces to force all members to have the same date version.
pub fn auto_version_increment_semver_or_date_forced() -> Result<()> {
    println!("  {YELLOW}Running auto_semver_or_date{RESET}");
    let cargo_toml = crate::cargo_auto_lib::auto_cargo_toml_mod::CargoToml::read().log(pos!())?;
    let version = cargo_toml.package_version();
    let version = semver::Version::parse(&version).log(pos!())?;
    if version.major > 2000 {
        crate::cargo_auto_lib::auto_version_from_date_mod::auto_version_from_date_forced().log(pos!())?;
    } else {
        crate::cargo_auto_lib::auto_semver_mod::auto_semver_increment_patch().log(pos!())?;
    }
    println!("  {YELLOW}Finished auto_semver_or_date{RESET}");
    Ok(())
}
