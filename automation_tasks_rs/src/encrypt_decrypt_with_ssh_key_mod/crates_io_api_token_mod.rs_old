// crates_io_api_token_mod.rs

//! # decrypt crates.io api token from file or ask the user to input the access token, encrypt it and save into file
//!
//! Publish to crates.io needs the crates.io secret access_token. This is a secret important just like a password or even more.  
//! There is the original "cargo login" function that saves this critical secret in plain text in `~/.cargo/credentials.toml`.  Plain-text for secrets in a well-known file is a big no-no. Every malware will just upload it in a millisecond.  
//!
//! I don't want to pass secret to an "obscure" library crate that is difficult to review and can change in any point in time to become malicious.  
//!
//! Instead of that, copy and paste this module `crates_io_api_token_mod.rs` file into your project.  
//! The secrets will stay in your codebase that is easy to inspect and guaranteed that will never change without your consent.  
//!
//! ## encrypt_decrypt_with_ssh_key_mod
//!
//! This module depends on the generic module for encryption `encrypt_decrypt_with_ssh_key_mod.rs`. That module also needs to be copy and paste into your project.
//!
//! ## Other dependencies
//!
//! In `Cargo.toml` there are a group od dependencies needed for this to work. They are so generic that I don't expect any malware in them to be able to steal some usable secrets.  
//!
//! Beware that the versions of crates in `Cargo.toml` are not precisely pinpointed. In rust the symbol '=' means "the same major number equal or newer to". This means from one compilation to another, it can automatically change to a newer version without the programmer even noticing it.
//!
//! This is great if the newer version is solving some security issue. But this is super-bad if the newer version is malware supply chain attack. We have no idea how to distinguish one from another.
//!
//! Just to mention: there exists the trick to control the `Cargo.lock` file and forbid the change of the version number, but more times than not, you will not want to commit the lock file into the GitHub repository.
//!
//! ```toml
//! [dependencies]
//! ssh-key = { version = "0.6.7", features = [ "rsa", "encryption","ed25519"] }
//! ssh_agent_client_rs_git_bash = "0.0.11"
//! rsa = { version = "0.9.7", features = ["sha2","pem"] }
//! zeroize = {version="1.8.1", features=["derive"]}
//! aes-gcm = "0.10.3"
//! camino = "1.1.6"
//! base64ct = {version = "1.6.0", features = ["alloc"] }
//! secrecy = "0.10.3"
//! ```
//!

#![allow(dead_code)]

use cargo_auto_lib::ShellCommandLimitedDoubleQuotesSanitizerTrait;
use secrecy::{SecretBox, SecretString};

use super::encrypt_decrypt_mod as ende;
use crate::{BLUE, GREEN, RED, RESET, YELLOW};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CratesIoConfig {
    pub crates_io_private_key_file_name: String,
}

/// Application state is initialized in the main() function.
///
/// And then is accessible all over the code.
pub static CRATES_IO_CONFIG: std::sync::OnceLock<CratesIoConfig> = std::sync::OnceLock::new();

/// Application state (static) is initialized only once in the main() function.
///
/// And then is accessible all over the code.
pub fn crates_io_config_initialize() {
    if CRATES_IO_CONFIG.get().is_some() {
        return;
    }

    let crates_io_config_json = std::fs::read_to_string("automation_tasks_rs/crates_io_config.json").unwrap();
    let crates_io_config: CratesIoConfig = serde_json::from_str(&crates_io_config_json).unwrap();
    let _ = CRATES_IO_CONFIG.set(crates_io_config);
}

/// get crates.io secret token
///
/// If exists, decrypt it from file.  
/// Else ask user to input the token and encrypt it into a file.  
pub(crate) fn get_crates_io_secret_token(private_key_file_name: &str) -> anyhow::Result<SecretString> {
    // check if the plain-text file from `cargo login` exists and warn the user
    // because it is a security vulnerability.
    println!("  {YELLOW}Check if credentials.toml from 'cargo login' exists.{RESET}");
    let tilde_file_credentials = "~/.cargo/credentials.toml";
    let file_credentials = ende::tilde_expand_to_home_dir_utf8(tilde_file_credentials)?;
    if file_credentials.exists() {
        eprintln!("{RED}Security vulnerability: Found the cargo credentials file with plain-text secret_token: {RESET}");
        eprintln!("{RED}{tilde_file_credentials}. It would be better to inspect and remove it. {RESET}");
        anyhow::bail!("Found security vulnerability");
    }

    println!("  {YELLOW}Check if the ssh private key exists.{RESET}");
    let private_key_path_struct = ende::PathStructInSshFolder::new(private_key_file_name.to_string())?;
    if !private_key_path_struct.exists() {
        eprintln!("{RED}Error: Private key {private_key_path_struct} does not exist.{RESET}");
        println!("  {YELLOW}Create the private key in bash terminal:{RESET}");
        println!(r#"{GREEN}ssh-keygen -t ed25519 -f "{private_key_path_struct}" -C "crates.io secret_token"{RESET}"#);
        anyhow::bail!("Private key file not found.");
    }

    println!("  {YELLOW}Check if the encrypted file exists.{RESET}");
    let encrypted_path_struct = ende::PathStructInSshFolder::new(format!("{private_key_file_name}.enc"))?;
    if !encrypted_path_struct.exists() {
        println!("  {YELLOW}Encrypted file {encrypted_path_struct} does not exist.{RESET}");
        println!("  {YELLOW}Get your secret token from: https://crates.io/settings/tokens {RESET}");
        println!("  {YELLOW}Never use 'cargo login' to store this secret locally. It will store it in plain-text in the file ~/.cargo.credentials.toml. {RESET}");
        println!("  {YELLOW}Plain-text for secrets in a well-known file is a big no-no. Every malware will just upload it in a millisecond. {RESET}");
        println!("  {YELLOW}This function will encrypt the secret with your ssh private key. {RESET}");
        println!();
        println!("{BLUE}Enter the secret_access_token to encrypt:{RESET}");
        let secret_access_token = secrecy::SecretString::from(
            crate::cl::inquire::Password::new("")
                .without_confirmation()
                .with_display_mode(crate::cl::inquire::PasswordDisplayMode::Masked)
                .prompt()?,
        );

        // prepare the random bytes, sign it with the private key, that is the true passcode used to encrypt the secret
        let (plain_seed_bytes_32bytes, plain_seed_string) = ende::random_seed_32bytes_and_string()?;
        // first try to use the private key from ssh-agent, else use the private file with user interaction
        let secret_passcode_32bytes: SecretBox<[u8; 32]> =
            ende::sign_seed_with_ssh_agent_or_private_key_file(&private_key_path_struct, plain_seed_bytes_32bytes)?;
        let plain_encrypted_text = ende::encrypt_symmetric(secret_passcode_32bytes, secret_access_token)?;

        // prepare a struct to save as encoded string
        let encrypted_text_with_metadata = ende::EncryptedTextWithMetadata {
            private_key_file_name: private_key_path_struct.get_file_name().to_string(),
            plain_seed_string,
            plain_encrypted_text,
            access_token_expiration: None,
            refresh_token_expiration: None,
            token_name: None,
        };
        let file_text = serde_json::to_string_pretty(&encrypted_text_with_metadata)?;
        // encode it just to obscure it a little bit
        let file_text = ende::encode64_from_string_to_string(&file_text);

        let mut file = std::fs::File::create(encrypted_path_struct.get_full_file_path())?;
        #[cfg(target_family = "unix")]
        {
            let metadata = file.metadata()?;
            let mut permissions = metadata.permissions();
            std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o600);
        }
        std::io::Write::write_all(&mut file, file_text.as_bytes())?;
        println!("  {YELLOW}Encrypted text saved to file.{RESET}");
    }

    println!("  {YELLOW}Open and read the encrypted file.{RESET}");
    let encrypted_text_with_metadata: String = ende::open_file_b64_get_string(encrypted_path_struct.get_full_file_path())?;
    // parse json
    let encrypted_text_with_metadata: ende::EncryptedTextWithMetadata = serde_json::from_str(&encrypted_text_with_metadata)?;
    println!("  {YELLOW}Decrypt the file with ssh-agent or private key.{RESET}");
    // the private key file is written inside the file
    let private_key_path_struct = ende::PathStructInSshFolder::new(encrypted_text_with_metadata.private_key_file_name.clone())?;
    if !camino::Utf8Path::new(private_key_path_struct.get_full_file_path()).exists() {
        anyhow::bail!("{RED}Error: File {private_key_path_struct} does not exist! {RESET}");
    }

    let plain_seed_bytes_32bytes = ende::decode64_from_string_to_32bytes(&encrypted_text_with_metadata.plain_seed_string)?;
    let secret_passcode_32bytes: SecretBox<[u8; 32]> =
        ende::sign_seed_with_ssh_agent_or_private_key_file(&private_key_path_struct, plain_seed_bytes_32bytes)?;

    // decrypt the secret access token string
    let secret_access_token: SecretString =
        ende::decrypt_symmetric(secret_passcode_32bytes, encrypted_text_with_metadata.plain_encrypted_text.clone())?;

    Ok(secret_access_token)
}

/// Publish to crates.io
#[allow(dead_code)]
pub fn publish_to_crates_io() -> anyhow::Result<()> {
    #[derive(serde::Deserialize, serde::Serialize)]
    struct CratesIoApiConfig {
        crates_io_secret_token_key: String,
    }

    let secret_access_token = get_crates_io_secret_token(&CRATES_IO_CONFIG.get().unwrap().crates_io_private_key_file_name)?;
    // the secret_token is redacted when print on screen
    cargo_auto_lib::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"cargo publish --token "{secret_token}" "#)
        .unwrap_or_else(|e| panic!("{e}"))
        .arg_secret("{secret_token}", &secret_access_token)
        .unwrap_or_else(|e| panic!("{e}"))
        .run()
        .unwrap_or_else(|e| panic!("{e}"));
    Ok(())
}
