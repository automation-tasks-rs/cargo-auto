// github_api_token_with_oauth2_mod.rs

//! # decrypt github api token from file or use the oauth2 device workflow to get the access token and encrypt it and save into file
//!
//! ## Secrets
//!
//! In this module there will be a lot of work with secrets.  
//! It is difficult to trust an external crate with your secrets.  
//! External crates can get updated unexpectedly and change to malicious code.  
//!
//! ## Copy code instead of dependency crate
//!
//! It is best to have the Rust code under your fingertips when dealing with secrets.  
//! Than you know, nobody will touch this code except of you.  
//! You can copy this code directly into your codebase as a module,
//! inspect and review it and know exactly what is going on.  
//! The code is as linear and readable with comments as possible.
//!
//! ## Store encrypted secret to file
//!
//! The secrets will be encrypted with an ssh private key and stored in the `~/.ssh` folder.  
//! This way the data is protected at rest in storage drive.  
//!
//! ## In memory protection
//!
//! This is a tough one! There is no 100% software protection of secrets in memory.  
//! Theoretically an attacker could dump the memory in any moment and read the secrets.  
//! There is always a moment when the secret is used in its plaintext form. This cannot be avoided.
//! All we can do now is to be alert what data is secret and take better care of it.  
//! Every variable that have secrets will have the word `secret` in it.
//! When a variable is confusing I will use the word `plain` to express it is `not a secret`.
//! To avoid leaking in logs I will use the `secrecy` crate. This is not 100% protection. It is important just to express intent when the secrets are really used.  
//! `Secrecy` needs the trait `zeroize` to empty the memory after use for better memory hygiene.
//! I will add the type names explicitly to emphasis the secrecy types used.
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
//! anyhow="1.0.95"
//! reqwest={version="0.12.12", features=["json","blocking"]}
//! serde ={ version= "1.0.217", features=["std","derive"]}
//! serde_json = "1.0.138"
//! ssh-key = { version = "0.6.7", features = [ "rsa", "encryption","ed25519"] }
//! ssh_agent_client_rs_git_bash = "0.0.11"
//! rsa = { version = "0.9.7", features = ["sha2","pem"] }
//! zeroize = {version="1.8.1", features=["derive"]}
//! aes-gcm = "0.10.3"
//! camino = "1.1.6"
//! base64ct = {version = "1.6.0", features = ["alloc"] }
//! secrecy = "0.10.3"
//! chrono ="0.4.39"
//! ```
//!

#![allow(dead_code)]

use anyhow::Context;
use secrecy::{ExposeSecret, SecretBox, SecretString};

use super::encrypt_decrypt_mod as ende;
use crate::{BLUE, GREEN, RED, RESET, YELLOW};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GithubApiConfig {
    pub github_app_name: String,
    pub client_id: String,
    pub github_api_private_key_file_name: String,
}

/// Application state (static) is initialized only once in the main() function.
///
/// And then is accessible all over the code.
pub static GITHUB_API_CONFIG: std::sync::OnceLock<GithubApiConfig> = std::sync::OnceLock::new();

#[derive(serde::Deserialize, serde::Serialize, zeroize::Zeroize, zeroize::ZeroizeOnDrop)]
struct SecretResponseAccessToken {
    access_token: String,
    expires_in: i64,
    refresh_token: String,
    refresh_token_expires_in: i64,
    scope: String,
    token_type: String,
}

/// Application state (static) is initialized only once in the main() function.
///
/// And then is accessible all over the code.
pub fn github_api_config_initialize() {
    if GITHUB_API_CONFIG.get().is_some() {
        return;
    }

    let github_api_config_json = std::fs::read_to_string("automation_tasks_rs/github_api_config.json").unwrap();
    let github_api_config: GithubApiConfig = serde_json::from_str(&github_api_config_json).unwrap();
    let _ = GITHUB_API_CONFIG.set(github_api_config);
}

/// Start the github oauth2 device workflow
/// It will use the private key from the .ssh folder.
/// The encrypted file has the same file name with the ".enc" extension.
/// Returns access_token to use as bearer for api calls
pub fn get_github_secret_token() -> anyhow::Result<SecretString> {
    let client_id = GITHUB_API_CONFIG.get().unwrap().client_id.to_string();
    let private_key_file_name = GITHUB_API_CONFIG.get().unwrap().github_api_private_key_file_name.to_string();

    println!("  {YELLOW}Check if the ssh private key exists.{RESET}");
    let private_key_path_struct = ende::PathStructInSshFolder::new(private_key_file_name.clone())?;
    if !private_key_path_struct.exists() {
        eprintln!("{RED}Error: Private key {private_key_path_struct} does not exist.{RESET}");
        println!("  {YELLOW}Create the private key in bash terminal:{RESET}");
        println!(r#"{GREEN}ssh-keygen -t ed25519 -f "{private_key_path_struct}" -C "github api secret_token"{RESET}"#);
        anyhow::bail!("Private key file not found.");
    }

    println!("  {YELLOW}Check if the encrypted file exists.{RESET}");
    let encrypted_path_struct = ende::PathStructInSshFolder::new(format!("{private_key_file_name}.enc"))?;
    if !encrypted_path_struct.exists() {
        println!("  {YELLOW}Encrypted file {encrypted_path_struct} does not exist.{RESET}");
        println!("  {YELLOW}Continue to authentication with the browser{RESET}");
        let secret_access_token = authenticate_with_browser_and_save_file(&client_id, &private_key_path_struct, &encrypted_path_struct)?;
        Ok(secret_access_token)
    } else {
        println!("  {YELLOW}Encrypted file {encrypted_path_struct} exist.{RESET}");
        let plain_file_text = ende::open_file_b64_get_string(encrypted_path_struct.get_full_file_path())?;
        // deserialize json into struct
        let encrypted_text_with_metadata: ende::EncryptedTextWithMetadata = serde_json::from_str(&plain_file_text)?;

        // check the expiration
        let utc_now = chrono::Utc::now();
        if encrypted_text_with_metadata.refresh_token_expiration.is_none() {
            anyhow::bail!("refresh_token_expiration is None");
        }
        let refresh_token_expiration = chrono::DateTime::parse_from_rfc3339(
            encrypted_text_with_metadata
                .refresh_token_expiration
                .as_ref()
                .expect("The former line asserts this is never None"),
        )?;
        if refresh_token_expiration <= utc_now {
            eprintln!("{RED}Refresh token has expired, start authentication_with_browser{RESET}");
            let secret_access_token =
                authenticate_with_browser_and_save_file(&client_id, &private_key_path_struct, &encrypted_path_struct)?;
            return Ok(secret_access_token);
        }
        if encrypted_text_with_metadata.access_token_expiration.is_none() {
            anyhow::bail!("access_token_expiration is None");
        }
        let access_token_expiration = chrono::DateTime::parse_from_rfc3339(
            encrypted_text_with_metadata
                .access_token_expiration
                .as_ref()
                .expect("The former line asserts this is never None"),
        )?;
        if access_token_expiration <= utc_now {
            eprintln!("{RED}Access token has expired, use refresh token{RESET}");
            let secret_response_refresh_token = decrypt_text_with_metadata(encrypted_text_with_metadata)?;
            let secret_response_access_token: SecretBox<SecretResponseAccessToken> =
                refresh_tokens(&client_id, secret_response_refresh_token.expose_secret().refresh_token.clone())?;
            let secret_access_token = SecretString::from(secret_response_access_token.expose_secret().access_token.clone());
            println!("  {YELLOW}Encrypt data and save file{RESET}");
            encrypt_and_save_file(&private_key_path_struct, &encrypted_path_struct, secret_response_access_token)?;
            return Ok(secret_access_token);
        }
        println!("  {YELLOW}Decrypt the file with the private key.{RESET}");
        let secret_response_access_token = decrypt_text_with_metadata(encrypted_text_with_metadata)?;
        let secret_access_token = SecretString::from(secret_response_access_token.expose_secret().access_token.clone());
        Ok(secret_access_token)
    }
}

fn authenticate_with_browser_and_save_file(
    client_id: &str,
    private_key_path_struct: &ende::PathStructInSshFolder,
    encrypted_path_struct: &ende::PathStructInSshFolder,
) -> anyhow::Result<SecretString> {
    let secret_response_access_token: SecretBox<SecretResponseAccessToken> = authentication_with_browser(client_id)?;
    let secret_access_token = SecretString::from(secret_response_access_token.expose_secret().access_token.clone());
    println!("  {YELLOW}Encrypt data and save file{RESET}");

    encrypt_and_save_file(private_key_path_struct, encrypted_path_struct, secret_response_access_token)?;
    Ok(secret_access_token)
}

/// Oauth2 device workflow needs to be authenticated with a browser
fn authentication_with_browser(client_id: &str) -> anyhow::Result<SecretBox<SecretResponseAccessToken>> {
    // https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps#device-flow
    // https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app#using-the-device-flow-to-generate-a-user-access-token
    println!("  {YELLOW}Send request with client_id and retrieve device_code and user_code{RESET}");
    println!("  {YELLOW}wait...{RESET}");

    #[derive(serde::Serialize)]
    struct RequestDeviceCode {
        client_id: String,
    }

    #[derive(serde::Deserialize)]
    struct ResponseDeviceCode {
        device_code: String,
        user_code: String,
    }

    let response_device_code: ResponseDeviceCode = reqwest::blocking::Client::new()
        .post("https://github.com/login/device/code")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&RequestDeviceCode {
            client_id: client_id.to_owned(),
        })
        .send()?
        .json()?;

    println!("  {YELLOW}Copy this user_code:{RESET}");
    println!("{GREEN}{}{RESET}", response_device_code.user_code);
    println!("  {YELLOW}Open browser on and paste the user_code:{RESET}");
    println!("{GREEN}https://github.com/login/device?skip_account_picker=true{RESET}");
    println!("{BLUE}After the tokens are prepared on the server, press enter to continue...{RESET}");

    let _user_input_just_enter_to_continue: String = inquire::Text::new("").prompt()?;

    #[derive(serde::Serialize)]
    struct RequestAccessToken {
        client_id: String,
        device_code: String,
        grant_type: String,
    }

    println!("  {YELLOW}Send request with device_id and retrieve access tokens{RESET}");
    println!("  {YELLOW}wait...{RESET}");
    let secret_response_access_token: SecretBox<SecretResponseAccessToken> = SecretBox::new(
        reqwest::blocking::Client::new()
            .post(" https://github.com/login/oauth/access_token")
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&RequestAccessToken {
                client_id: client_id.to_string(),
                device_code: response_device_code.device_code.to_string(),
                grant_type: "urn:ietf:params:oauth:grant-type:device_code".to_string(),
            })
            .send()?
            .json()?,
    );

    Ok(secret_response_access_token)
}

/// use refresh token to get new access_token and refresh_token
fn refresh_tokens(client_id: &str, refresh_token: String) -> anyhow::Result<SecretBox<SecretResponseAccessToken>> {
    // https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/refreshing-user-access-tokens

    #[derive(serde::Serialize)]
    struct RequestWithRefreshToken {
        client_id: String,
        grant_type: String,
        refresh_token: String,
    }

    println!("  {YELLOW}Send request with client_id and refresh_token and retrieve access tokens{RESET}");
    println!("  {YELLOW}wait...{RESET}");
    let secret_response_access_token: SecretBox<SecretResponseAccessToken> = SecretBox::new(Box::new(
        reqwest::blocking::Client::new()
            .post("https://github.com/login/oauth/access_token")
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&RequestWithRefreshToken {
                client_id: client_id.to_owned(),
                grant_type: "refresh_token".to_string(),
                refresh_token,
            })
            .send()?
            .json()?,
    ));

    Ok(secret_response_access_token)
}

/// encrypt and save file
///
/// The "seed" are just some random 32 bytes.
/// The "seed" will be "signed" with the private key.
/// Only the "owner" can unlock the private key and sign correctly.
/// This signature will be used as the true passcode for symmetrical encryption.
/// The "seed" and the private key path will be stored in plain text in the file
/// together with the encrypted data in json format.
/// To avoid plain text in the end encode in base64 just for obfuscate a little bit.
fn encrypt_and_save_file(
    private_key_path_struct: &ende::PathStructInSshFolder,
    encrypted_path_struct: &ende::PathStructInSshFolder,
    secret_response_access_token: SecretBox<SecretResponseAccessToken>,
) -> anyhow::Result<()> {
    let secret_string = SecretString::from(serde_json::to_string(&secret_response_access_token.expose_secret())?);

    let (plain_seed_bytes_32bytes, plain_seed_string) = ende::random_seed_32bytes_and_string()?;

    println!("  {YELLOW}Unlock private key to encrypt the secret symmetrically{RESET}");
    let secret_passcode_32bytes: SecretBox<[u8; 32]> =
        ende::sign_seed_with_ssh_agent_or_private_key_file(private_key_path_struct, plain_seed_bytes_32bytes)?;

    println!("  {YELLOW}Encrypt the secret symmetrically {RESET}");
    let encrypted_string = ende::encrypt_symmetric(secret_passcode_32bytes, secret_string)?;

    // the file will contain json with 3 plain text fields: fingerprint, seed, encrypted, expiration

    // calculate expiration minus 10 minutes or 600 seconds
    let utc_now = chrono::Utc::now();
    let access_token_expiration = utc_now
        .checked_add_signed(chrono::Duration::seconds(
            secret_response_access_token.expose_secret().expires_in - 600,
        ))
        .context("checked_add_signed")?
        .to_rfc3339();
    let refresh_token_expiration = utc_now
        .checked_add_signed(chrono::Duration::seconds(
            secret_response_access_token.expose_secret().refresh_token_expires_in - 600,
        ))
        .context("checked_add_signed")?
        .to_rfc3339();

    let encrypted_text_with_metadata = ende::EncryptedTextWithMetadata {
        private_key_file_name: private_key_path_struct.get_file_name().to_string(),
        plain_seed_string,
        plain_encrypted_text: encrypted_string,
        access_token_expiration: Some(access_token_expiration),
        refresh_token_expiration: Some(refresh_token_expiration),
        token_name: None,
    };
    let plain_file_text = serde_json::to_string_pretty(&encrypted_text_with_metadata)?;
    // encode it just to obscure it a little bit
    let file_text = ende::encode64_from_string_to_string(&plain_file_text);

    let mut file = std::fs::File::create(encrypted_path_struct.get_full_file_path())?;
    #[cfg(target_family = "unix")]
    {
        let metadata = file.metadata()?;
        let mut permissions = metadata.permissions();
        std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o600);
    }
    std::io::Write::write_all(&mut file, file_text.as_bytes())?;

    println!("  {YELLOW}Encrypted text saved to file.{RESET}");

    Ok(())
}

/// decrypt text with metadata
///
/// The encrypted file is encoded in base64 just to obfuscate it a little bit.  
/// In json format in plain text there is the "seed", the private key path and the encrypted secret.  
/// The "seed" will be "signed" with the private key.  
/// Only the "owner" can unlock the private key and sign correctly.  
/// This signature will be used as the true passcode for symmetrical decryption.  
fn decrypt_text_with_metadata(
    encrypted_text_with_metadata: ende::EncryptedTextWithMetadata,
) -> anyhow::Result<SecretBox<SecretResponseAccessToken>> {
    // the private key file is written inside the file
    let private_key_path_struct = ende::PathStructInSshFolder::new(encrypted_text_with_metadata.private_key_file_name.clone())?;
    if !camino::Utf8Path::new(private_key_path_struct.get_full_file_path()).exists() {
        anyhow::bail!("{RED}Error: File {private_key_path_struct} does not exist! {RESET}");
    }

    let plain_seed_bytes_32bytes = ende::decode64_from_string_to_32bytes(&encrypted_text_with_metadata.plain_seed_string)?;
    // first try to use the private key from ssh-agent, else use the private file with user interaction
    let secret_passcode_32bytes: SecretBox<[u8; 32]> =
        ende::sign_seed_with_ssh_agent_or_private_key_file(&private_key_path_struct, plain_seed_bytes_32bytes)?;
    // decrypt the data
    let decrypted_string = ende::decrypt_symmetric(secret_passcode_32bytes, encrypted_text_with_metadata.plain_encrypted_text)?;
    // parse json to struct
    let secret_response_access_token: SecretBox<SecretResponseAccessToken> =
        SecretBox::new(Box::new(serde_json::from_str(decrypted_string.expose_secret())?));
    Ok(secret_response_access_token)
}

pub(crate) fn send_to_github_api_with_secret_token(req: reqwest::blocking::RequestBuilder) -> anyhow::Result<serde_json::Value> {
    // I must build the request to be able then to inspect it.
    let req = req.bearer_auth(get_github_secret_token()?.expose_secret()).build()?;

    // region: Assert the correct url and https
    // It is important that the request coming from a external crate/library
    // is only sent always and only to GitHub API and not some other malicious url,
    // because the request contains the secret GitHub API secret_token.
    // And it must always use https
    let host_str = req.url().host_str().context("host_str")?;
    assert!(
        host_str == "api.github.com",
        "{RED}Error: Url is not correct: {host_str}. It must be always api.github.com.{RESET}"
    );
    let scheme = req.url().scheme();
    assert!(
        scheme == "https",
        "{RED}Error: Scheme is not correct: {scheme}. It must be always https.{RESET}"
    );
    // endregion: Assert the correct url and https

    let reqwest_client = reqwest::blocking::Client::new();
    let response_text = reqwest_client.execute(req)?.text()?;

    let json_value: serde_json::Value = serde_json::from_str(&response_text)?;

    // panic if "message": String("Bad credentials"),
    if let Some(m) = json_value.get("message") {
        if m == "Bad credentials" {
            panic!("{RED}Error: Bad credentials for GitHub API. {RESET}");
        }
    }

    // return
    Ok(json_value)
}

/// Upload to GitHub
///
/// This function encapsulates the secret API secret_token.
/// The RequestBuilder is created somewhere in the library crate.
/// The client can be passed to the library. It will not reveal the secret_token.
/// This is basically an async fn, but use of `async fn` in public traits is discouraged...
pub(crate) async fn upload_to_github_with_secret_token(req: reqwest::RequestBuilder) -> anyhow::Result<serde_json::Value> {
    // I must build the request to be able then to inspect it.
    let req = req.bearer_auth(get_github_secret_token()?.expose_secret()).build()?;

    // region: Assert the correct url and https
    // It is important that the request coming from a external crate/library
    // is only sent always and only to GitHub uploads and not some other malicious url,
    // because the request contains the secret GitHub API secret_token.
    // And it must always use https
    let host_str = req.url().host_str().context("host_str")?;
    assert!(
        host_str == "uploads.github.com",
        "{RED}Error: Url is not correct: {host_str}. It must be always api.github.com.{RESET}"
    );
    let scheme = req.url().scheme();
    assert!(
        scheme == "https",
        "{RED}Error: Scheme is not correct: {scheme}. It must be always https.{RESET}"
    );
    // endregion: Assert the correct url and https

    let reqwest_client = reqwest::Client::new();
    let response_text = reqwest_client.execute(req).await?.text().await?;

    let json_value: serde_json::Value = serde_json::from_str(&response_text)?;

    // panic if "message": String("Bad credentials"),
    if let Some(m) = json_value.get("message") {
        if m == "Bad credentials" {
            panic!("{RED}Error: Bad credentials for GitHub API. {RESET}");
        }
    }

    // return
    Ok(json_value)
}
