// docker_io_api_token_mod.rs

// region: auto_md_to_doc_comments include doc_comments/docker_io_api_token_mod.md A //!
//! # decrypt docker.io api token from file or ask the user to input the access token, encrypt it and save into file
//!
//! Publish to docker.io needs the docker.io secret access_token. This is a secret important just like a password or even more.  
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
//! ssh-agent-client-rs = "0.9.1"
//! rsa = { version = "0.9.7", features = ["sha2","pem"] }
//! zeroize = {version="1.8.1", features=["derive"]}
//! aes-gcm = "0.10.3"
//! camino = "1.1.6"
//! base64ct = {version = "1.6.0", features = ["alloc"] }
//! inquire = "0.7.0"
//! secrecy = "0.10.3"
//! ```
//!
// endregion: auto_md_to_doc_comments include doc_comments/docker_io_api_token_mod.md A //!

#![allow(dead_code)]

use secrecy::{SecretBox, SecretString};

use crate::encrypt_decrypt_with_ssh_key_mod as ende;
use crate::encrypt_decrypt_with_ssh_key_mod::{BLUE, GREEN, RED, RESET, YELLOW};

/// get docker.io secret token
///
/// If exists, decrypt it from file.  
/// Else ask user to input the token and encrypt it into a file.  
pub(crate) fn get_docker_hub_secret_token(private_key_file_bare_name: &str) -> anyhow::Result<SecretString> {
    println!("  {YELLOW}Check if the ssh private key exists.{RESET}");
    let private_key_file_path = camino::Utf8PathBuf::from(format!("/home/rustdevuser/.ssh/{private_key_file_bare_name}").as_str());
    if !std::fs::exists(&private_key_file_path)? {
        eprintln!("{RED}Error: Private key {private_key_file_path} does not exist.{RESET}");
        println!("  {YELLOW}Create the private key in bash terminal:{RESET}");
        println!(r#"{GREEN}ssh-keygen -t ed25519 -f "{private_key_file_path}" -C "docker.io secret_token"{RESET}"#);
        anyhow::bail!("Private key file not found.");
    }

    println!("  {YELLOW}Check if the encrypted file exists.{RESET}");
    let encrypted_file_name = camino::Utf8PathBuf::from(format!("/home/rustdevuser/.ssh/{private_key_file_bare_name}.enc").as_str());
    if !std::fs::exists(&encrypted_file_name)? {
        println!("  {YELLOW}Encrypted file {encrypted_file_name} does not exist.{RESET}");
        println!("  {YELLOW}Get your secret token from: https://app.docker.com/settings/personal-access-tokens {RESET}");
        println!("  {YELLOW}This function will encrypt the secret with your ssh private key. {RESET}");
        println!();
        println!("{BLUE}Enter the secret_access_token to encrypt:{RESET}");
        let secret_access_token = secrecy::SecretString::from(inquire::Password::new("").without_confirmation().with_display_mode(inquire::PasswordDisplayMode::Masked).prompt()?);

        // prepare the random bytes, sign it with the private key, that is the true passcode used to encrypt the secret
        let (plain_seed_bytes_32bytes, plain_seed_string) = ende::random_seed_32bytes_and_string()?;
        // first try to use the private key from ssh-agent, else use the private file with user interaction
        let secret_passcode_32bytes: SecretBox<[u8; 32]> = ende::sign_seed_with_ssh_agent_or_private_key_file(&private_key_file_path, plain_seed_bytes_32bytes)?;
        let plain_encrypted_text = ende::encrypt_symmetric(secret_passcode_32bytes, secret_access_token)?;

        // prepare a struct to save as encoded string
        let encrypted_text_with_metadata = ende::EncryptedTextWithMetadata {
            private_key_file_path: private_key_file_path.to_string(),
            plain_seed_string,
            plain_encrypted_text,
            access_token_expiration: None,
            refresh_token_expiration: None,
            token_name: None,
        };
        let file_text = serde_json::to_string_pretty(&encrypted_text_with_metadata)?;
        // encode it just to obscure it a little bit
        let file_text = ende::encode64_from_string_to_string(&file_text);

        std::fs::write(&encrypted_file_name, file_text)?;
        println!("  {YELLOW}Encrypted text saved to file.{RESET}");
    }

    println!("  {YELLOW}Open and read the encrypted file.{RESET}");
    let encrypted_text_with_metadata: String = ende::open_file_b64_get_string(&encrypted_file_name)?;
    // parse json
    let encrypted_text_with_metadata: ende::EncryptedTextWithMetadata = serde_json::from_str(&encrypted_text_with_metadata)?;
    println!("  {YELLOW}Decrypt the file with ssh-agent or private key.{RESET}");
    let plain_seed_bytes_32bytes = ende::decode64_from_string_to_32bytes(&encrypted_text_with_metadata.plain_seed_string)?;
    let private_key_file_path = camino::Utf8PathBuf::from(&encrypted_text_with_metadata.private_key_file_path);
    let secret_passcode_32bytes: SecretBox<[u8; 32]> = ende::sign_seed_with_ssh_agent_or_private_key_file(&private_key_file_path, plain_seed_bytes_32bytes)?;

    // decrypt the secret access token string
    let secret_access_token: SecretString = ende::decrypt_symmetric(secret_passcode_32bytes, encrypted_text_with_metadata.plain_encrypted_text.clone())?;

    Ok(secret_access_token)
}
