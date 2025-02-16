// secrets_always_local_mod.rs

/// Secrets like GitHub API secret_token, crates.io secret token, docker hub secret_token, SSH private key passphrase and similar
/// must never go out of this crate. Never pass any secret to an external crate library as much as possible.
/// The user has the source code under his fingers in this crate. So he knows nobody will mess with this code
/// once he inspected and reviewed it.
/// All the modules are in one file to avoid clutter in the automation_tasks_rs folder.
/// The simple program flow of functions that need secrets is butchered to avoid secrets leaving this crate.
/// Now it looks like a mess, but the goal is achieved. The secrets never leave this crate.

// region: Public API constants
// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
/// ANSI color
pub const RED: &str = "\x1b[31m";
/// ANSI color
pub const GREEN: &str = "\x1b[32m";
/// ANSI color
pub const YELLOW: &str = "\x1b[33m";
/// ANSI color
pub const BLUE: &str = "\x1b[34m";
/// ANSI color
pub const RESET: &str = "\x1b[0m";
// endregion: Public API constants

pub use cargo_auto_encrypt_secret_lib::EncryptedString;
pub use secrecy::ExposeSecret;

pub(crate) mod decrypt_mod {

    use crate::secrets_always_local_mod::*;

    /// The secrets must not leave this crate.
    /// They are never going into an external library crate.
    /// This crate is "user code" and is easy to review and inspect.
    pub(crate) struct Decryptor<'a> {
        secret_string: secrecy::SecretString,
        secret_passcode_bytes: &'a secrecy::SecretVec<u8>,
    }

    impl<'a> Decryptor<'a> {
        pub(crate) fn new_for_decrypt(secret_passcode_bytes: &'a secrecy::SecretVec<u8>) -> Self {
            Decryptor {
                secret_string: secrecy::SecretString::new("".to_string()),
                secret_passcode_bytes,
            }
        }
        pub(crate) fn return_secret_string(&self) -> &secrecy::SecretString {
            &self.secret_string
        }

        /// Decrypts encrypted_string with secret_passcode_bytes
        ///
        /// secret_passcode_bytes must be 32 bytes or more
        /// Returns the secret_string
        pub(crate) fn decrypt_symmetric(&mut self, encrypted_string: &cargo_auto_encrypt_secret_lib::EncryptedString) {
            let encrypted_bytes = <base64ct::Base64 as base64ct::Encoding>::decode_vec(&encrypted_string.0).unwrap();
            //only first 32 bytes
            let mut secret_passcode_32bytes = [0u8; 32];
            secret_passcode_32bytes.copy_from_slice(&self.secret_passcode_bytes.expose_secret()[0..32]);

            let cipher = <aes_gcm::Aes256Gcm as aes_gcm::KeyInit>::new(&secret_passcode_32bytes.into());
            // nonce is salt
            let nonce = rsa::sha2::digest::generic_array::GenericArray::from_slice(&encrypted_bytes[..12]);
            let cipher_text = &encrypted_bytes[12..];

            let Ok(decrypted_bytes) = aes_gcm::aead::Aead::decrypt(&cipher, nonce, cipher_text) else {
                panic!("{RED}Error: Decryption failed. {RESET}");
            };
            let decrypted_string = String::from_utf8(decrypted_bytes).unwrap();
            self.secret_string = secrecy::SecretString::new(decrypted_string)
        }
    }
}

pub(crate) mod encrypt_mod {
    use crate::secrets_always_local_mod::*;

    /// The secrets must not leave this crate.
    /// They are never going into an external library crate.
    /// This crate is "user code" and is easy to review and inspect.
    pub(crate) struct Encryptor<'a> {
        secret_string: secrecy::SecretString,
        secret_passcode_bytes: &'a secrecy::SecretVec<u8>,
    }

    impl<'a> Encryptor<'a> {
        pub(crate) fn new_for_encrypt(secret_string: secrecy::SecretString, secret_passcode_bytes: &'a secrecy::SecretVec<u8>) -> Self {
            Encryptor { secret_string, secret_passcode_bytes }
        }

        /// Encrypts secret_string with secret_passcode_bytes
        ///
        /// secret_passcode_bytes must be 32 bytes or more
        /// returns the encrypted_string
        pub(crate) fn encrypt_symmetric(&self) -> Option<cargo_auto_encrypt_secret_lib::EncryptedString> {
            //only first 32 bytes
            let mut secret_passcode_32bytes = [0u8; 32];
            secret_passcode_32bytes.copy_from_slice(&self.secret_passcode_bytes.expose_secret()[0..32]);

            let cipher = <aes_gcm::Aes256Gcm as aes_gcm::KeyInit>::new(&secret_passcode_32bytes.into());
            // nonce is salt
            let nonce = <aes_gcm::Aes256Gcm as aes_gcm::AeadCore>::generate_nonce(&mut aes_gcm::aead::OsRng);

            let Ok(cipher_text) = aes_gcm::aead::Aead::encrypt(&cipher, &nonce, self.secret_string.expose_secret().as_bytes()) else {
                panic!("{RED}Error: Encryption failed. {RESET}");
            };

            let mut encrypted_bytes = nonce.to_vec();
            encrypted_bytes.extend_from_slice(&cipher_text);
            let encrypted_string = <base64ct::Base64 as base64ct::Encoding>::encode_string(&encrypted_bytes);
            Some(cargo_auto_encrypt_secret_lib::EncryptedString(encrypted_string))
        }
    }
}

pub(crate) mod secrecy_mod {

    //! The crate secrecy is probably great.
    //! But I want to encrypt the content, so I will make a wrapper.
    //! The secrets must always be moved to secrecy types as soon as possible.

    use crate::secrets_always_local_mod::*;

    pub struct SecretEncryptedString {
        encrypted_string: EncryptedString,
    }

    impl SecretEncryptedString {
        pub fn new_with_secret_string(secret_string: secrecy::SecretString, session_passcode: &secrecy::SecretVec<u8>) -> Self {
            let encryptor = super::encrypt_mod::Encryptor::new_for_encrypt(secret_string, &session_passcode);
            let encrypted_string = encryptor.encrypt_symmetric().unwrap();

            SecretEncryptedString { encrypted_string }
        }

        pub fn new_with_string(secret_string: String, session_passcode: &secrecy::SecretVec<u8>) -> Self {
            let secret_string = secrecy::SecretString::new(secret_string);
            Self::new_with_secret_string(secret_string, session_passcode)
        }

        pub fn expose_decrypted_secret(&self, session_passcode: &secrecy::SecretVec<u8>) -> secrecy::SecretString {
            let mut decryptor = super::decrypt_mod::Decryptor::new_for_decrypt(&session_passcode);
            decryptor.decrypt_symmetric(&self.encrypted_string);
            decryptor.return_secret_string().clone()
        }
    }
}

pub(crate) mod ssh_mod {

    use crate::secrets_always_local_mod::*;

    pub struct SshContext {
        signed_passcode_is_a_secret: secrecy::SecretVec<u8>,
        decrypted_string: secrecy::SecretString,
    }

    impl SshContext {
        pub fn new() -> Self {
            SshContext {
                signed_passcode_is_a_secret: secrecy::SecretVec::new(vec![]),
                decrypted_string: secrecy::SecretString::new("".to_string()),
            }
        }
        pub fn get_decrypted_string(&self) -> secrecy::SecretString {
            self.decrypted_string.clone()
        }
    }

    impl cargo_auto_encrypt_secret_lib::SshContextTrait for SshContext {
        /// decrypt from file data and write the decrypted secret in private field for later use in this crate, not in external library crates
        fn decrypt_from_file_data(&mut self, encrypted_string: &cargo_auto_encrypt_secret_lib::EncryptedString) {
            let mut decryptor = decrypt_mod::Decryptor::new_for_decrypt(&self.signed_passcode_is_a_secret);
            decryptor.decrypt_symmetric(encrypted_string);
            self.decrypted_string = decryptor.return_secret_string().clone();
        }

        /// get secret_token and encrypt
        fn get_secret_token_and_encrypt(&self) -> cargo_auto_encrypt_secret_lib::EncryptedString {
            /// Internal function used only for test configuration
            ///
            /// It is not interactive, but reads from a env var.
            #[cfg(test)]
            fn get_secret_token() -> secrecy::SecretString {
                secrecy::SecretString::new(std::env::var("TEST_TOKEN").unwrap())
            }
            /// Internal function get_passphrase interactively ask user to type the passphrase
            ///
            /// This is used for normal code execution.
            #[cfg(not(test))]
            fn get_secret_token() -> secrecy::SecretString {
                eprintln!(" ");
                eprintln!("   {BLUE}Enter the secret_token to encrypt:{RESET}");
                secrecy::SecretString::new(
                    inquire::Password::new("")
                        .without_confirmation()
                        .with_display_mode(inquire::PasswordDisplayMode::Masked)
                        .prompt()
                        .unwrap(),
                )
            }
            let secret_token = get_secret_token();
            // use this signed as password for symmetric encryption
            let encryptor = encrypt_mod::Encryptor::new_for_encrypt(secret_token, &self.signed_passcode_is_a_secret);

            let encrypted_token = encryptor.encrypt_symmetric().unwrap();
            // return
            encrypted_token
        }

        /// Sign with ssh-agent or with identity_file
        ///
        /// get passphrase interactively
        /// returns secret_password_bytes:Vec u8
        fn sign_with_ssh_agent_or_identity_file(&mut self, identity_private_file_path: &camino::Utf8Path, seed_bytes_not_a_secret: &[u8; 32]) {
            /// Internal function used only for test configuration
            ///
            /// It is not interactive, but reads from a env var.
            #[cfg(test)]
            fn get_passphrase() -> secrecy::SecretString {
                secrecy::SecretString::new(std::env::var("TEST_PASSPHRASE").unwrap())
            }
            /// Internal function get_passphrase interactively ask user to type the passphrase
            ///
            /// This is used for normal code execution.
            #[cfg(not(test))]
            fn get_passphrase() -> secrecy::SecretString {
                eprintln!(" ");
                eprintln!("   {BLUE}Enter the passphrase for the SSH private key:{RESET}");
                secrecy::SecretString::new(
                    inquire::Password::new("")
                        .without_confirmation()
                        .with_display_mode(inquire::PasswordDisplayMode::Masked)
                        .prompt()
                        .unwrap(),
                )
            }

            let identity_private_file_path_expanded = expand_path_check_private_key_exists(identity_private_file_path);

            let fingerprint_from_file = cargo_auto_encrypt_secret_lib::get_fingerprint_from_file(&identity_private_file_path_expanded);

            let mut ssh_agent_client = cargo_auto_encrypt_secret_lib::crate_ssh_agent_client();
            match cargo_auto_encrypt_secret_lib::ssh_add_list_contains_fingerprint(&mut ssh_agent_client, &fingerprint_from_file) {
                Some(public_key) => {
                    // sign with public key from ssh-agent
                    let signature_is_the_new_secret_password = ssh_agent_client.sign(&public_key, seed_bytes_not_a_secret).unwrap();
                    // only the data part of the signature goes into as_bytes.
                    self.signed_passcode_is_a_secret = secrecy::SecretVec::new(signature_is_the_new_secret_password.as_bytes().to_owned());
                }
                None => {
                    // ask user to think about adding with ssh-add
                    eprintln!("   {YELLOW}SSH key for encrypted secret_token is not found in the ssh-agent.{RESET}");
                    eprintln!("   {YELLOW}Without ssh-agent, you will have to type the private key passphrase every time. This is more secure, but inconvenient.{RESET}");
                    eprintln!("   {YELLOW}You can manually add the SSH identity to ssh-agent for 1 hour:{RESET}");
                    eprintln!("   {YELLOW}WARNING: using ssh-agent is less secure, because there is no need for user interaction.{RESET}");
                    eprintln!("{GREEN}ssh-add -t 1h {identity_private_file_path_expanded}{RESET}");

                    // just for test purpose I will use env var to read this passphrase. Don't use it in production.

                    let passphrase_is_a_secret = get_passphrase();
                    let private_key = ssh_key::PrivateKey::read_openssh_file(identity_private_file_path_expanded.as_std_path()).unwrap();
                    let mut private_key = private_key.decrypt(passphrase_is_a_secret.expose_secret()).unwrap();

                    // FYI: this type of signature is compatible with ssh-agent because it does not involve namespace
                    let signature_is_the_new_secret_password = rsa::signature::SignerMut::try_sign(&mut private_key, seed_bytes_not_a_secret).unwrap();

                    // only the data part of the signature goes into as_bytes.
                    self.signed_passcode_is_a_secret = secrecy::SecretVec::new(signature_is_the_new_secret_password.as_bytes().to_owned());
                }
            }
        }
    }

    /// Expand path and check if identity file exists
    ///
    /// Inform the user how to generate identity file.
    pub fn expand_path_check_private_key_exists(identity_private_file_path: &camino::Utf8Path) -> camino::Utf8PathBuf {
        let identity_private_file_path_expanded = cargo_auto_encrypt_secret_lib::file_path_home_expand(identity_private_file_path);
        if !camino::Utf8Path::new(&identity_private_file_path_expanded).exists() {
            eprintln!("{RED}Identity file {identity_private_file_path_expanded} that contains the SSH private key does not exist! {RESET}");
            eprintln!("    {YELLOW}Create the SSH key manually in bash with this command:{RESET}");
            if identity_private_file_path_expanded.as_str().contains("github_api") {
                eprintln!(r#"{GREEN}ssh-keygen -t ed25519 -f "{identity_private_file_path_expanded}" -C "github api secret_token"{RESET}"#);
            } else if identity_private_file_path_expanded.as_str().contains("crates_io") {
                eprintln!(r#"{GREEN}ssh-keygen -t ed25519 -f "{identity_private_file_path_expanded}" -C "crates io secret_token"{RESET}"#);
            } else if identity_private_file_path_expanded.as_str().contains("docker_hub") {
                eprintln!(r#"{GREEN}ssh-keygen -t ed25519 -f "{identity_private_file_path_expanded}" -C "docker hub secret_token"{RESET}"#);
            }
            eprintln!(" ");
            panic!("{RED}Error: File {identity_private_file_path_expanded} does not exist! {RESET}");
        }
        identity_private_file_path_expanded
    }
}

pub(crate) mod github_mod {

    //! Every API call needs the GitHub API secret_token. This is a secret important just like a password.
    //! I don't want to pass this secret to an "obscure" library crate that is difficult to review.
    //! This secret will stay here in this codebase that every developer can easily inspect.
    //! Instead of the secret_token, I will pass the struct GitHubClient with the trait SendToGitHubApi.
    //! This way, the secret_token will be encapsulated.

    use crate::secrets_always_local_mod::*;
    use cargo_auto_github_lib as cgl;
    use reqwest::Client;

    /// Struct GitHubClient contains only private fields
    /// This fields are accessible only to methods in implementation of traits.
    pub struct GitHubClient {
        /// Passcode for encrypt the secret_token to encrypted_token in memory.
        /// So that the secret is in memory as little as possible as plain text.
        /// For every session (program start) a new random passcode is created.
        session_passcode: secrecy::SecretVec<u8>,

        /// private field is set only once in the new() constructor
        encrypted_token: super::secrecy_mod::SecretEncryptedString,
    }

    impl GitHubClient {
        /// Create new GitHub client
        ///
        /// Interactively ask the user to input the GitHub secret_token.
        pub fn new_interactive_input_secret_token() -> Self {
            let mut github_client = Self::new_wo_secret_token();

            println!("{BLUE}Enter the GitHub API secret_token:{RESET}");
            github_client.encrypted_token =
                super::secrecy_mod::SecretEncryptedString::new_with_string(inquire::Password::new("").without_confirmation().prompt().unwrap(), &github_client.session_passcode);

            // return
            github_client
        }

        /// Create new GitHub client without secret_token
        fn new_wo_secret_token() -> Self {
            /// Internal function Generate a random password
            fn random_byte_passcode() -> [u8; 32] {
                let mut password = [0_u8; 32];
                use aes_gcm::aead::rand_core::RngCore;
                aes_gcm::aead::OsRng.fill_bytes(&mut password);
                password
            }

            let session_passcode = secrecy::SecretVec::new(random_byte_passcode().to_vec());
            let encrypted_token = super::secrecy_mod::SecretEncryptedString::new_with_string("".to_string(), &session_passcode);

            GitHubClient { session_passcode, encrypted_token }
        }

        /// Use the stored API secret_token
        ///
        /// If the secret_token not exists ask user to interactively input the secret_token.
        /// To decrypt it, use the SSH passphrase. That is much easier to type than typing the secret_token.
        /// it is then possible also to have the ssh key in ssh-agent and write the passphrase only once.
        /// But this great user experience comes with security concerns. The secret_token is accessible if the attacker is very dedicated.
        pub fn new_with_stored_secret_token() -> Self {
            /// Internal function for DRY Don't Repeat Yourself
            fn read_secret_token_and_decrypt_return_github_client(mut ssh_context: super::ssh_mod::SshContext, encrypted_string_file_path: &camino::Utf8Path) -> GitHubClient {
                // read the secret_token and decrypt
                cargo_auto_encrypt_secret_lib::decrypt_with_ssh_interactive_from_file(&mut ssh_context, encrypted_string_file_path);
                let secret_token = ssh_context.get_decrypted_string();
                let mut github_client = GitHubClient::new_wo_secret_token();
                github_client.encrypted_token = super::secrecy_mod::SecretEncryptedString::new_with_secret_string(secret_token, &github_client.session_passcode);
                github_client
            }

            let encrypted_string_file_path = camino::Utf8Path::new("~/.ssh/github_api_secret_token_ssh_1.enc");
            let encrypted_string_file_path_expanded = cargo_auto_encrypt_secret_lib::file_path_home_expand(encrypted_string_file_path);

            let identity_private_file_path = camino::Utf8Path::new("~/.ssh/github_api_secret_token_ssh_1");
            let _identity_private_file_path_expanded = crate::secrets_always_local_mod::ssh_mod::expand_path_check_private_key_exists(identity_private_file_path);

            if !encrypted_string_file_path_expanded.exists() {
                // ask interactive
                println!("    {BLUE}Do you want to store the GitHub API secret_token encrypted with an SSH key? (y/n){RESET}");
                let answer = inquire::Text::new("").prompt().unwrap();
                if answer.to_lowercase() != "y" {
                    // enter the secret_token manually, not storing
                    return Self::new_interactive_input_secret_token();
                } else {
                    // get the passphrase and secret_token interactively
                    let mut ssh_context = super::ssh_mod::SshContext::new();
                    // encrypt and save the encrypted secret_token
                    cargo_auto_encrypt_secret_lib::encrypt_with_ssh_interactive_save_file(&mut ssh_context, identity_private_file_path, encrypted_string_file_path);
                    // read the secret_token and decrypt, return GitHubClient
                    read_secret_token_and_decrypt_return_github_client(ssh_context, encrypted_string_file_path)
                }
            } else {
                // file exists
                let ssh_context = super::ssh_mod::SshContext::new();
                // read the secret_token and decrypt, return GitHubClient
                read_secret_token_and_decrypt_return_github_client(ssh_context, encrypted_string_file_path)
            }
        }

        /// decrypts the secret_token in memory
        #[allow(dead_code)]
        pub fn decrypt_secret_token_in_memory(&self) -> secrecy::SecretString {
            self.encrypted_token.expose_decrypted_secret(&self.session_passcode)
        }
    }

    /// trait from the crate library, so the 2 crates can share a function
    impl cgl::SendToGitHubApi for GitHubClient {
        /// Send GitHub API request
        ///
        /// This function encapsulates the secret API secret_token.
        /// The RequestBuilder is created somewhere in the library crate.
        /// The client can be passed to the library. It will not reveal the secret_token.
        fn send_to_github_api(&self, req: reqwest::blocking::RequestBuilder) -> serde_json::Value {
            // I must build the request to be able then to inspect it.
            let req = req.bearer_auth(self.decrypt_secret_token_in_memory().expose_secret()).build().unwrap();

            // region: Assert the correct url and https
            // It is important that the request coming from a external crate/library
            // is only sent always and only to GitHub API and not some other malicious url,
            // because the request contains the secret GitHub API secret_token.
            // And it must always use https
            let host_str = req.url().host_str().unwrap();
            assert!(host_str == "api.github.com", "{RED}Error: Url is not correct: {host_str}. It must be always api.github.com.{RESET}");
            let scheme = req.url().scheme();
            assert!(scheme == "https", "{RED}Error: Scheme is not correct: {scheme}. It must be always https.{RESET}");
            // endregion: Assert the correct url and https

            let reqwest_client = reqwest::blocking::Client::new();
            let response_text = reqwest_client.execute(req).unwrap().text().unwrap();

            let json_value: serde_json::Value = serde_json::from_str(&response_text).unwrap();

            // panic if "message": String("Bad credentials"),
            if let Some(m) = json_value.get("message") {
                if m == "Bad credentials" {
                    panic!("{RED}Error: Bad credentials for GitHub API. {RESET}");
                }
            }

            // return
            json_value
        }

        /// Upload to GitHub
        ///
        /// This function encapsulates the secret API secret_token.
        /// The RequestBuilder is created somewhere in the library crate.
        /// The client can be passed to the library. It will not reveal the secret_token.
        /// This is basically an async fn, but use of `async fn` in public traits is discouraged...
        async fn upload_to_github(&self, req: reqwest::RequestBuilder) -> serde_json::Value {
            // I must build the request to be able then to inspect it.
            let req = req.bearer_auth(self.decrypt_secret_token_in_memory().expose_secret()).build().unwrap();

            // region: Assert the correct url and https
            // It is important that the request coming from a external crate/library
            // is only sent always and only to GitHub uploads and not some other malicious url,
            // because the request contains the secret GitHub API secret_token.
            // And it must always use https
            let host_str = req.url().host_str().unwrap();
            assert!(host_str == "uploads.github.com", "{RED}Error: Url is not correct: {host_str}. It must be always api.github.com.{RESET}");
            let scheme = req.url().scheme();
            assert!(scheme == "https", "{RED}Error: Scheme is not correct: {scheme}. It must be always https.{RESET}");
            // endregion: Assert the correct url and https

            let reqwest_client = Client::new();
            let response_text = reqwest_client.execute(req).await.unwrap().text().await.unwrap();

            let json_value: serde_json::Value = serde_json::from_str(&response_text).unwrap();

            // panic if "message": String("Bad credentials"),
            if let Some(m) = json_value.get("message") {
                if m == "Bad credentials" {
                    panic!("{RED}Error: Bad credentials for GitHub API. {RESET}");
                }
            }

            // return
            json_value
        }
    }
}

pub(crate) mod crates_io_mod {

    //! Publish to crates.io needs the crates.io secret_token. This is a secret important just like a password.
    //! I don't want to pass this secret to an "obscure" library crate that is difficult to review.
    //! This secret will stay here in this codebase that every developer can easily inspect.
    //! Instead of the secret_token, I will pass the struct CratesIoClient with the trait SendToCratesIo.
    //! This way, the secret_token will be encapsulated.

    use crate::secrets_always_local_mod::*;
    use cargo_auto_lib::ShellCommandLimitedDoubleQuotesSanitizer;
    use cargo_auto_lib::ShellCommandLimitedDoubleQuotesSanitizerTrait;

    /// Struct CratesIoClient contains only private fields
    /// This fields are accessible only to methods in implementation of traits.
    pub struct CratesIoClient {
        /// Passcode for encrypt the secret_token to encrypted_token in memory.
        /// So that the secret is in memory as little as possible as plain text.
        /// For every session (program start) a new random passcode is created.
        session_passcode: secrecy::SecretVec<u8>,

        /// private field is set only once in the new() constructor
        encrypted_token: super::secrecy_mod::SecretEncryptedString,
    }

    impl CratesIoClient {
        /// Create new CratesIo client
        ///
        /// Interactively ask the user to input the crates.io secret_token.
        #[allow(dead_code)]
        pub fn new_interactive_input_secret_token() -> Self {
            let mut crates_io_client = Self::new_wo_secret_token();

            println!("{BLUE}Enter the crates.io secret_token:{RESET}");
            crates_io_client.encrypted_token =
                super::secrecy_mod::SecretEncryptedString::new_with_string(inquire::Password::new("").without_confirmation().prompt().unwrap(), &crates_io_client.session_passcode);

            // return
            crates_io_client
        }

        /// Create new CratesIo client without secret_token
        #[allow(dead_code)]
        fn new_wo_secret_token() -> Self {
            /// Internal function Generate a random password
            fn random_byte_passcode() -> [u8; 32] {
                let mut password = [0_u8; 32];
                use aes_gcm::aead::rand_core::RngCore;
                aes_gcm::aead::OsRng.fill_bytes(&mut password);
                password
            }

            let session_passcode = secrecy::SecretVec::new(random_byte_passcode().to_vec());
            let encrypted_token = super::secrecy_mod::SecretEncryptedString::new_with_string("".to_string(), &session_passcode);

            CratesIoClient { session_passcode, encrypted_token }
        }

        /// Use the stored crates.io secret_token
        ///
        /// If the secret_token not exists ask user to interactively input the secret_token.
        /// To decrypt it, use the SSH passphrase. That is much easier to type than typing the secret_token.
        /// It is then possible also to have the ssh key in ssh-agent and write the passphrase only once.
        /// But this great user experience comes with security concerns. The secret_token is accessible if the attacker is very dedicated.
        #[allow(dead_code)]
        pub fn new_with_stored_secret_token() -> Self {
            /// Internal function for DRY Don't Repeat Yourself
            fn read_secret_token_and_decrypt_return_crates_io_client(mut ssh_context: super::ssh_mod::SshContext, encrypted_string_file_path: &camino::Utf8Path) -> CratesIoClient {
                cargo_auto_encrypt_secret_lib::decrypt_with_ssh_interactive_from_file(&mut ssh_context, encrypted_string_file_path);
                let secret_token = ssh_context.get_decrypted_string();
                let mut crates_io_client = CratesIoClient::new_wo_secret_token();
                crates_io_client.encrypted_token = super::secrecy_mod::SecretEncryptedString::new_with_secret_string(secret_token, &crates_io_client.session_passcode);
                crates_io_client
            }

            // check if the plain-text file from `podman login` exists and warn the user because it is a security vulnerability.
            let file_auth = "~/.cargo/credentials.toml";
            let file_auth = camino::Utf8Path::new(file_auth);
            // TODO: check for env variable also?
            let file_auth_expanded = cargo_auto_encrypt_secret_lib::file_path_home_expand(file_auth);
            let file_auth_expanded = camino::Utf8Path::new(&file_auth_expanded);
            if file_auth_expanded.exists() {
                eprintln!("{RED}Security vulnerability: Found the cargo file with plain-text secret_token: {file_auth_expanded}. It would be better to inspect and remove it. {RESET}")
            }

            let encrypted_string_file_path = camino::Utf8Path::new("~/.ssh/crates_io_secret_token_ssh_1.enc");
            let encrypted_string_file_path_expanded = cargo_auto_encrypt_secret_lib::file_path_home_expand(encrypted_string_file_path);

            let identity_private_file_path = camino::Utf8Path::new("~/.ssh/crates_io_secret_token_ssh_1");
            let _identity_private_file_path_expanded = crate::secrets_always_local_mod::ssh_mod::expand_path_check_private_key_exists(identity_private_file_path);

            if !encrypted_string_file_path_expanded.exists() {
                // ask interactive
                println!("    {BLUE}Do you want to store the crates.io secret_token encrypted with an SSH key? (y/n){RESET}");
                let answer = inquire::Text::new("").prompt().unwrap();
                if answer.to_lowercase() != "y" {
                    // enter the secret_token manually, not storing
                    return Self::new_interactive_input_secret_token();
                } else {
                    // get the passphrase and secret_token interactively
                    let mut ssh_context = super::ssh_mod::SshContext::new();
                    // encrypt and save the encrypted secret_token
                    cargo_auto_encrypt_secret_lib::encrypt_with_ssh_interactive_save_file(&mut ssh_context, identity_private_file_path, encrypted_string_file_path);
                    // read the secret_token and decrypt, return CratesIoClient
                    read_secret_token_and_decrypt_return_crates_io_client(ssh_context, encrypted_string_file_path)
                }
            } else {
                // file exists
                let ssh_context = super::ssh_mod::SshContext::new();
                // read the secret_token and decrypt, return CratesIoClient
                read_secret_token_and_decrypt_return_crates_io_client(ssh_context, encrypted_string_file_path)
            }
        }

        /// decrypts the secret_token in memory
        #[allow(dead_code)]
        pub fn decrypt_secret_token_in_memory(&self) -> secrecy::SecretString {
            self.encrypted_token.expose_decrypted_secret(&self.session_passcode)
        }

        /// Publish to crates.io
        ///
        /// This function encapsulates the secret crates.io secret_token.
        /// The client can be passed to the library. It will not reveal the secret_token.
        #[allow(dead_code)]
        pub fn publish_to_crates_io(&self) {
            // the secret_token is redacted when print on screen
            ShellCommandLimitedDoubleQuotesSanitizer::new(r#"cargo publish --token "{secret_token}" "#)
                .unwrap_or_else(|e| panic!("{e}"))
                .arg_secret("{secret_token}", &self.decrypt_secret_token_in_memory())
                .unwrap_or_else(|e| panic!("{e}"))
                .run()
                .unwrap_or_else(|e| panic!("{e}"));
        }
    }
}

pub(crate) mod docker_hub_mod {

    //! Push to docker-hub needs the docker hub secret_token. This is a secret important just like a password.
    //! I don't want to pass this secret to an "obscure" library crate that is difficult to review.
    //! This secret will stay here in this codebase that every developer can easily inspect.
    //! Instead of the secret_token, I will pass the struct DockerHubClient with the trait SendToDockerHub.
    //! This way, the secret_token will be encapsulated.

    use crate::secrets_always_local_mod::*;
    use cargo_auto_lib::ShellCommandLimitedDoubleQuotesSanitizer;
    use cargo_auto_lib::ShellCommandLimitedDoubleQuotesSanitizerTrait;

    /// Struct DockerHubClient contains only private fields
    /// This fields are accessible only to methods in implementation of traits.
    pub struct DockerHubClient {
        /// Passcode for encrypt the secret_token to encrypted_token in memory.
        /// So that the secret is in memory as little as possible as plain text.
        /// For every session (program start) a new random passcode is created.
        session_passcode: secrecy::SecretVec<u8>,

        /// private field is set only once in the new() constructor
        encrypted_token: super::secrecy_mod::SecretEncryptedString,
    }

    impl DockerHubClient {
        /// Create new DockerHub client
        ///
        /// Interactively ask the user to input the docker hub secret_token.
        #[allow(dead_code)]
        pub fn new_interactive_input_secret_token() -> Self {
            let mut docker_hub_client = Self::new_wo_secret_token();

            println!("{BLUE}Enter the docker hub secret_token:{RESET}");
            docker_hub_client.encrypted_token =
                super::secrecy_mod::SecretEncryptedString::new_with_string(inquire::Password::new("").without_confirmation().prompt().unwrap(), &docker_hub_client.session_passcode);

            // return
            docker_hub_client
        }

        /// Create new DockerHub client without secret_token
        #[allow(dead_code)]
        fn new_wo_secret_token() -> Self {
            /// Internal function Generate a random password
            fn random_byte_passcode() -> [u8; 32] {
                let mut password = [0_u8; 32];
                use aes_gcm::aead::rand_core::RngCore;
                aes_gcm::aead::OsRng.fill_bytes(&mut password);
                password
            }

            let session_passcode = secrecy::SecretVec::new(random_byte_passcode().to_vec());
            let encrypted_token = super::secrecy_mod::SecretEncryptedString::new_with_string("".to_string(), &session_passcode);

            DockerHubClient { session_passcode, encrypted_token }
        }

        /// Use the stored docker hub secret_token
        ///
        /// If the secret_token not exists ask user to interactively input the secret_token.
        /// To decrypt it, use the SSH passphrase. That is much easier to type than typing the secret_token.
        /// It is then possible also to have the ssh key in ssh-agent and write the passphrase only once.
        /// But this great user experience comes with security concerns. The secret_token is accessible if the attacker is very dedicated.
        #[allow(dead_code)]
        pub fn new_with_stored_secret_token(user_name: &str, registry: &str) -> Self {
            /// Internal function for DRY Don't Repeat Yourself
            fn read_secret_token_and_decrypt_return_docker_hub_client(mut ssh_context: super::ssh_mod::SshContext, encrypted_string_file_path: &camino::Utf8Path) -> DockerHubClient {
                cargo_auto_encrypt_secret_lib::decrypt_with_ssh_interactive_from_file(&mut ssh_context, encrypted_string_file_path);
                let secret_token = ssh_context.get_decrypted_string();
                let mut docker_hub_client = DockerHubClient::new_wo_secret_token();
                docker_hub_client.encrypted_token = super::secrecy_mod::SecretEncryptedString::new_with_secret_string(secret_token, &docker_hub_client.session_passcode);
                docker_hub_client
            }

            // check if the plain-text file from `podman login` exists and warn the user because it is a security vulnerability.
            let file_auth = "${XDG_RUNTIME_DIR}/containers/auth.json";
            // TODO: check for env variable also?
            if let Some(xdg_runtime_dir) = std::env::var_os("XDG_RUNTIME_DIR"){
                let xdg_runtime_dir=xdg_runtime_dir.to_string_lossy().to_string();
                let file_auth_expanded = file_auth.replace("${XDG_RUNTIME_DIR}", &xdg_runtime_dir);
                let file_auth_expanded = camino::Utf8Path::new(&file_auth_expanded);
                if file_auth_expanded.exists() {
                    eprintln!("{RED}Security vulnerability: Found the docker hub file with plain-text secret_token: {file_auth_expanded}. It would be better to inspect and remove it. {RESET}")
                }
            }

            // registry: docker.io -> replace dot into "--""
            // username: bestiadev
            let registry_escaped = registry.replace(".", "--");
            let encrypted_string_file_path = format!("~/.ssh/docker_hub_{registry_escaped}_{user_name}.enc");
            let encrypted_string_file_path = camino::Utf8Path::new(&encrypted_string_file_path);
            let encrypted_string_file_path_expanded = cargo_auto_encrypt_secret_lib::file_path_home_expand(encrypted_string_file_path);

            let identity_private_file_path = camino::Utf8Path::new("~/.ssh/docker_hub_secret_token_ssh_1");
            let _identity_private_file_path_expanded = crate::secrets_always_local_mod::ssh_mod::expand_path_check_private_key_exists(identity_private_file_path);

            if !encrypted_string_file_path_expanded.exists() {
                // ask interactive
                println!("    {BLUE}Do you want to store the docker hub secret_token encrypted with an SSH key? (y/n){RESET}");
                let answer = inquire::Text::new("").prompt().unwrap();
                if answer.to_lowercase() != "y" {
                    // enter the secret_token manually, not storing
                    return Self::new_interactive_input_secret_token();
                } else {
                    // get the passphrase and secret_token interactively
                    let mut ssh_context = super::ssh_mod::SshContext::new();
                    // encrypt and save the encrypted secret_token
                    cargo_auto_encrypt_secret_lib::encrypt_with_ssh_interactive_save_file(&mut ssh_context, identity_private_file_path, encrypted_string_file_path);
                    // read the secret_token and decrypt, return DockerHubClient
                    read_secret_token_and_decrypt_return_docker_hub_client(ssh_context, encrypted_string_file_path)
                }
            } else {
                // file exists
                let ssh_context = super::ssh_mod::SshContext::new();
                // read the secret_token and decrypt, return DockerHubClient
                read_secret_token_and_decrypt_return_docker_hub_client(ssh_context, encrypted_string_file_path)
            }
        }

        /// decrypts the secret_token in memory
        #[allow(dead_code)]
        pub fn decrypt_secret_token_in_memory(&self) -> secrecy::SecretString {
            self.encrypted_token.expose_decrypted_secret(&self.session_passcode)
        }

        /// Push to docker hub
        ///
        /// This function encapsulates the secret docker hub secret_token.
        /// The client can be passed to the library. It will not reveal the secret_token.
        #[allow(dead_code)]
        pub fn push_to_docker_hub(&self, image_url: &str, user_name: &str) {
            // the secret_token can be used in place of the password in --cred
            ShellCommandLimitedDoubleQuotesSanitizer::new(r#"podman push --creds "{user_name}:{secret_token}" "{image_url}" "#)
                .unwrap_or_else(|e| panic!("{e}"))
                .arg("{user_name}", user_name)
                .unwrap_or_else(|e| panic!("{e}"))
                .arg_secret("{secret_token}", &self.decrypt_secret_token_in_memory())
                .unwrap_or_else(|e| panic!("{e}"))
                .arg("{image_url}", image_url)
                .unwrap_or_else(|e| panic!("{e}"))
                .run()
                .unwrap_or_else(|e| panic!("{e}"));
        }
    }
}
