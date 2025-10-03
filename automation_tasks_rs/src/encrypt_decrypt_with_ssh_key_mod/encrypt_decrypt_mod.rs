// encrypt_decrypt_with_ssh_key_mod.rs

//! Generic functions to encrypt and decrypt secrets using the ssh private key.
//!
//! Don't change this code, so it can be updated regularly with
//! cargo auto update_automation_tasks_rs
//! If you want to customize it, copy the code into main.rs and modify it there.

use crate::{BLUE, GREEN, RED, RESET, YELLOW};
use crossplatform_path::CrossPathBuf;
use secrecy::{ExposeSecret, ExposeSecretMut, SecretBox, SecretString};

pub struct PathStructInSshFolder {
    file_name: String,
    tilde_file_path: CrossPathBuf,
}

impl PathStructInSshFolder {
    /// Private key file path: CrossPathBuf have tilde. Later it transforms to full path.
    pub fn new(file_name: String) -> anyhow::Result<Self> {
        let tilde_file_path = CrossPathBuf::new(&format!("~/.ssh/{file_name}"))?;
        Ok(PathStructInSshFolder {
            file_name,
            tilde_file_path,
        })
    }

    /// Get reference to file name.
    pub fn get_file_name(&self) -> &str {
        &self.file_name
    }

    /// Get reference to tilde file path CrossPathBuf.
    pub fn get_cross_path(&self) -> &CrossPathBuf {
        &self.tilde_file_path
    }

    /// Get full file path.
    pub fn get_full_file_path(&self) -> std::path::PathBuf {
        self.tilde_file_path.to_path_buf_current_os()
    }

    /// Return true if file exists on path.
    pub fn exists(&self) -> bool {
        self.tilde_file_path.exists()
    }
}

impl std::fmt::Display for PathStructInSshFolder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.tilde_file_path)
    }
}

/// Struct that represents the json data saved in the '*.enc' file.
#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct EncryptedTextWithMetadata {
    pub(crate) private_key_file_name: String,
    pub(crate) plain_seed_string: String,
    pub(crate) plain_encrypted_text: String,
    pub(crate) token_name: Option<String>,
    pub(crate) access_token_expiration: Option<String>,
    pub(crate) refresh_token_expiration: Option<String>,
}

/// Generate a random seed.
///
/// This seed will be signed with the private key and
/// that will be the passcode for symmetric encryption
/// We will need the bytes and the string representation
pub(crate) fn random_seed_32bytes_and_string() -> anyhow::Result<([u8; 32], String)> {
    let mut seed_32bytes = [0_u8; 32];
    use aes_gcm::aead::rand_core::RngCore;
    aes_gcm::aead::OsRng.fill_bytes(&mut seed_32bytes);
    let plain_seed_string = encode64_from_32bytes_to_string(seed_32bytes)?;
    Ok((seed_32bytes, plain_seed_string))
}

/// Get the string from the file that is base64 encoded.
///
/// It is encoded just to obscure it a little.
pub(crate) fn open_file_b64_get_string(plain_file_b64_path: &CrossPathBuf) -> anyhow::Result<String> {
    if !plain_file_b64_path.exists() {
        anyhow::bail!("{RED}Error: File {plain_file_b64_path} does not exist! {RESET}");
    }

    let plain_file_text = plain_file_b64_path.read_to_string()?;
    // it is encoded just to obscure it a little
    let plain_file_text = decode64_from_string_to_string(&plain_file_text)?;

    Ok(plain_file_text)
}

/// Shorten the `Vec<u8> to [u8;32]`.  
pub(crate) fn shorten_vec_bytes_to_32bytes(vec_u8: Vec<u8>) -> anyhow::Result<[u8; 32]> {
    if vec_u8.len() < 32 {
        anyhow::bail!("The bytes must never be less then 32 bytes.");
    }
    let mut secret_passcode_32bytes = [0u8; 32];
    secret_passcode_32bytes.copy_from_slice(&vec_u8[0..32]);

    Ok(secret_passcode_32bytes)
}

// region: seed encode and decode - string and bytes

/// Decode base64 from string to 32bytes.
pub(crate) fn encode64_from_32bytes_to_string(bytes_32bytes: [u8; 32]) -> anyhow::Result<String> {
    Ok(<base64ct::Base64 as base64ct::Encoding>::encode_string(&bytes_32bytes))
}

/// Decode base64 from string to 32bytes.
pub(crate) fn decode64_from_string_to_32bytes(plain_seed_string: &str) -> anyhow::Result<[u8; 32]> {
    let plain_seed_bytes = <base64ct::Base64 as base64ct::Encoding>::decode_vec(plain_seed_string)?;
    let plain_seed_bytes_32bytes = shorten_vec_bytes_to_32bytes(plain_seed_bytes)?;
    Ok(plain_seed_bytes_32bytes)
}

/// Encode base64 from bytes to string.
pub(crate) fn encode64_from_bytes_to_string(plain_seed_bytes_32bytes: Vec<u8>) -> String {
    <base64ct::Base64 as base64ct::Encoding>::encode_string(&plain_seed_bytes_32bytes)
}

/// Encode base64 from string to string.
///
/// It is a silly little obfuscation just to avoid using plain text.
pub(crate) fn encode64_from_string_to_string(string_to_encode: &str) -> String {
    <base64ct::Base64 as base64ct::Encoding>::encode_string(string_to_encode.as_bytes())
}

/// Decode base64 from string to string.
///
/// It is a silly little obfuscation just to avoid using plain text.
pub(crate) fn decode64_from_string_to_string(string_to_decode: &str) -> anyhow::Result<String> {
    let decoded_string = String::from_utf8(<base64ct::Base64 as base64ct::Encoding>::decode_vec(string_to_decode)?)?;
    Ok(decoded_string)
}

// endregion: seed encode and decode - string and bytes

// region: sign the seed with ssh-agent or private key

/// Returns the secret signed seed.
///
/// First it tries to use the ssh-agent.  
/// Else it uses the private key and ask the user to input the passphrase.  
/// If the passphrase is 'empty string' it will try ssh-agent one more time.
/// The secret signed seed will be the actual password for symmetrical encryption.  
/// Returns secret_password_bytes.  
pub(crate) fn sign_seed_with_ssh_agent_or_private_key_file(
    private_key_path_struct: &PathStructInSshFolder,
    plain_seed_bytes_32bytes: [u8; 32],
) -> anyhow::Result<SecretBox<[u8; 32]>> {
    let secret_passcode_32bytes_maybe = sign_seed_with_ssh_agent(plain_seed_bytes_32bytes, private_key_path_struct);
    let secret_passcode_32bytes: SecretBox<[u8; 32]> = if secret_passcode_32bytes_maybe.is_ok() {
        secret_passcode_32bytes_maybe?
    } else {
        // ask user to think about adding key into ssh-agent with ssh-add
        println!("  {YELLOW}SSH key for encrypted secret_token is not found in the ssh-agent.{RESET}");
        println!("  {YELLOW}Without ssh-agent, you will have to type the private key passphrase every time.{RESET}");
        println!("  {YELLOW}This is more secure, but inconvenient.{RESET}");
        println!("  {YELLOW}WARNING: using ssh-agent is less secure, because there is no need for user interaction.{RESET}");
        println!("  {YELLOW}Knowing this, you can manually add the SSH private key to ssh-agent for 1 hour.{RESET}");
        println!("  {YELLOW}You can simply open a new bash terminal and add the key there right now:{RESET}");
        println!("{GREEN}ssh-add -t 1h {private_key_path_struct}{RESET}");
        println!("  {YELLOW}Unlock the private key to decrypt the saved file.{RESET}");

        match sign_seed_with_private_key_file(plain_seed_bytes_32bytes, &private_key_path_struct.tilde_file_path) {
            Ok(secret_passcode_32bytes) => secret_passcode_32bytes,
            Err(err) => {
                if err.to_string() == "Passphrase empty" {
                    // try with ssh-agent, because maybe the developer has ssh-add in another terminal right now
                    return sign_seed_with_ssh_agent(plain_seed_bytes_32bytes, private_key_path_struct);
                }
                anyhow::bail!(err)
            }
        }
    };
    Ok(secret_passcode_32bytes)
}

/// Sign seed with ssh-agent into 32 bytes secret.
///
/// This will be the true passcode for symmetrical encryption and decryption.  
/// Returns secret_password_bytes.  
fn sign_seed_with_ssh_agent(
    plain_seed_bytes_32bytes: [u8; 32],
    private_key_path_struct: &PathStructInSshFolder,
) -> anyhow::Result<SecretBox<[u8; 32]>> {
    /// Internal function returns the public_key inside ssh-add
    fn public_key_from_ssh_agent(
        client: &mut ssh_agent_client_rs_git_bash::Client,
        fingerprint_from_file: &str,
    ) -> anyhow::Result<ssh_key::PublicKey> {
        let vec_identities = client.list_all_identities()?;
        for identity in vec_identities.iter() {
            if let ssh_agent_client_rs_git_bash::Identity::PublicKey(public_key) = identity {
                let fingerprint_from_agent = public_key.key_data().fingerprint(Default::default()).to_string();

                if fingerprint_from_agent == fingerprint_from_file {
                    return Ok(public_key.clone().into_owned());
                }
            }
        }
        anyhow::bail!("This private key is not added to ssh-agent.")
    }
    let public_key_path_struct = PathStructInSshFolder::new(format!("{}.pub", private_key_path_struct.get_file_name()))?;
    let public_key = ssh_key::PublicKey::read_openssh_file(&public_key_path_struct.get_full_file_path())?;
    let fingerprint_from_file = public_key.fingerprint(Default::default()).to_string();

    println!("  {YELLOW}Connect to ssh-agent on SSH_AUTH_SOCK{RESET}");
    let var_ssh_auth_sock = std::env::var("SSH_AUTH_SOCK")?;
    let path_ssh_auth_sock = CrossPathBuf::new(&var_ssh_auth_sock)?;
    // import trait into scope
    use ssh_agent_client_rs_git_bash::GitBash;
    let mut ssh_agent_client =
        ssh_agent_client_rs_git_bash::Client::connect_to_git_bash_or_linux(&path_ssh_auth_sock.to_path_buf_current_os())?;

    let public_key = public_key_from_ssh_agent(&mut ssh_agent_client, &fingerprint_from_file)?;

    let mut secret_passcode_32bytes = SecretBox::new(Box::new([0u8; 32]));
    // sign with public key from ssh-agent
    // only the data part of the signature goes into as_bytes.
    secret_passcode_32bytes
        .expose_secret_mut()
        .copy_from_slice(&ssh_agent_client.sign(&public_key, &plain_seed_bytes_32bytes)?.as_bytes()[0..32]);

    Ok(secret_passcode_32bytes)
}

/// Sign the seed with the private key into 32 bytes secret.
///
/// User must input the passphrase to unlock the private key file.  
/// This will be the true passcode for symmetrical encryption and decryption.  
/// Returns secret_password_bytes.  
fn sign_seed_with_private_key_file(
    plain_seed_bytes_32bytes: [u8; 32],
    private_key_file_path: &CrossPathBuf,
) -> anyhow::Result<SecretBox<[u8; 32]>> {
    /// Internal function for user input passphrase
    fn user_input_secret_passphrase() -> anyhow::Result<SecretString> {
        println!();
        println!("{BLUE}Enter the passphrase for the SSH private key:{RESET}");

        let secret_passphrase = SecretString::from(
            inquire::Password::new("")
                .without_confirmation()
                .with_display_mode(inquire::PasswordDisplayMode::Masked)
                .prompt()?,
        );
        if secret_passphrase.expose_secret().is_empty() {
            anyhow::bail!("Passphrase empty");
        }
        Ok(secret_passphrase)
    }
    // the user is the only one that knows the passphrase to unlock the private key
    let secret_user_passphrase: SecretString = user_input_secret_passphrase()?;

    // sign_with_ssh_private_key_file
    println!("  {YELLOW}Use ssh private key from file {RESET}");
    let private_key = ssh_key::PrivateKey::read_openssh_file(&private_key_file_path.to_path_buf_current_os())?;
    println!("  {YELLOW}Unlock the private key {RESET}");

    // cannot use secrecy: PrivateKey does not have trait Zeroize
    let mut secret_private_key = private_key.decrypt(secret_user_passphrase.expose_secret())?;

    // FYI: this type of signature is compatible with ssh-agent because it does not involve namespace
    println!("  {YELLOW}Sign the seed {RESET}");

    let mut secret_passcode_32bytes = SecretBox::new(Box::new([0u8; 32]));
    // only the data part of the signature goes into as_bytes.
    // only the first 32 bytes
    secret_passcode_32bytes
        .expose_secret_mut()
        .copy_from_slice(&rsa::signature::SignerMut::try_sign(&mut secret_private_key, &plain_seed_bytes_32bytes)?.as_bytes()[0..32]);

    Ok(secret_passcode_32bytes)
}

// endregion: sign the seed with ssh-agent or private key

// region: symmetrical encrypt and decrypt

/// Encrypts symmetrically secret_string_to_encrypt with secret_passcode_32bytes.
///
/// Consumes the secret_passcode_32bytes and secret_string_to_encrypt.  
/// Returns the plain_encrypted_string, it is not a secret anymore.  
pub(crate) fn encrypt_symmetric(
    secret_passcode_32bytes: SecretBox<[u8; 32]>,
    secret_string_to_encrypt: SecretString,
) -> anyhow::Result<String> {
    // nonce is salt
    let nonce = <aes_gcm::Aes256Gcm as aes_gcm::AeadCore>::generate_nonce(&mut aes_gcm::aead::OsRng);
    let Ok(cipher_text_encrypted) = aes_gcm::aead::Aead::encrypt(
        // cipher_secret is the true passcode, here I don't know how to use secrecy, because the type has not the trait Zeroize
        &<aes_gcm::Aes256Gcm as aes_gcm::KeyInit>::new(secret_passcode_32bytes.expose_secret().into()),
        &nonce,
        secret_string_to_encrypt.expose_secret().as_bytes(),
    ) else {
        anyhow::bail!("{RED}Error: Encryption failed. {RESET}");
    };

    let mut encrypted_bytes = nonce.to_vec();
    encrypted_bytes.extend_from_slice(&cipher_text_encrypted);
    // plain encrypted string is not a secret anymore
    let plain_encrypted_string = encode64_from_bytes_to_string(encrypted_bytes);

    Ok(plain_encrypted_string)
}

/// Decrypts plain_encrypted_string with secret_passcode_32bytes.
///
/// Consumes secret_passcode_32bytes and encrypted_string.  
/// Returns the secret_decrypted_string.  
pub(crate) fn decrypt_symmetric(
    secret_passcode_32bytes: SecretBox<[u8; 32]>,
    plain_encrypted_string: String,
) -> anyhow::Result<SecretString> {
    let encrypted_bytes = <base64ct::Base64 as base64ct::Encoding>::decode_vec(&plain_encrypted_string)?;
    // nonce is salt
    let nonce = rsa::sha2::digest::generic_array::GenericArray::from_slice(&encrypted_bytes[..12]);
    let cipher_text = &encrypted_bytes[12..];

    let Ok(secret_decrypted_bytes) = aes_gcm::aead::Aead::decrypt(
        // cipher_secret is the true passcode, here I don't know how to use secrecy, because the type has not the trait Zeroize
        &<aes_gcm::Aes256Gcm as aes_gcm::KeyInit>::new(secret_passcode_32bytes.expose_secret().into()),
        nonce,
        cipher_text,
    ) else {
        anyhow::bail!("{RED}Error: Decryption failed. {RESET}");
    };
    let secret_decrypted_string = SecretString::from(String::from_utf8(secret_decrypted_bytes)?);

    Ok(secret_decrypted_string)
}
// endregion: symmetrical encrypt and decrypt
