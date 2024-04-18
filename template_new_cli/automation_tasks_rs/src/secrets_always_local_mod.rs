// secrets_always_local_mod.rs

/// Secrets like GitHub API token, crates.io token, SSH private key passphrase and similar
/// must never go out of this crate. Never pass any secret to an external crate library as much as possible.
/// The user has the source code under his fingers in this crate. So he knows nobody will mess with this code
/// once he inspected and reviewed it.
/// All the modules are in one file to avoid clutter in the automation_tasks_rs folder.
/// The simple program flow of functions that need secrets is butchered to avoid secrets leaving this crate.
/// Now it looks like a mess, but the goal is achieved. The secrets never leave this crate.

pub(crate) mod decrypt_mod {

    use cargo_auto_lib::RED;
    use cargo_auto_lib::RESET;
    use secrecy::ExposeSecret;

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

    use cargo_auto_lib::RED;
    use cargo_auto_lib::RESET;

    // bring trait to scope
    use secrecy::ExposeSecret;

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

    use cargo_auto_encrypt_secret_lib::EncryptedString;

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

    #[allow(unused_imports)]
    use cargo_auto_lib::BLUE;
    use cargo_auto_lib::GREEN;
    use cargo_auto_lib::RED;
    use cargo_auto_lib::RESET;
    use cargo_auto_lib::YELLOW;

    use crate::secrets_always_local_mod::*;

    // bring trait into scope
    use secrecy::ExposeSecret;

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

        /// get token and encrypt
        fn get_token_and_encrypt(&self) -> cargo_auto_encrypt_secret_lib::EncryptedString {
            /// Internal function used only for test configuration
            ///
            /// It is not interactive, but reads from a env var.
            #[cfg(test)]
            fn get_token() -> secrecy::SecretString {
                secrecy::SecretString::new(std::env::var("TEST_TOKEN").unwrap())
            }
            /// Internal function get_passphrase interactively ask user to type the passphrase
            ///
            /// This is used for normal code execution.
            #[cfg(not(test))]
            fn get_token() -> secrecy::SecretString {
                eprintln!(" ");
                eprintln!("   {BLUE}Enter the API token to encrypt:{RESET}");
                secrecy::SecretString::new(
                    inquire::Password::new("")
                        .without_confirmation()
                        .with_display_mode(inquire::PasswordDisplayMode::Masked)
                        .prompt()
                        .unwrap(),
                )
            }
            let token_is_a_secret = get_token();
            // use this signed as password for symmetric encryption
            let encryptor = encrypt_mod::Encryptor::new_for_encrypt(token_is_a_secret, &self.signed_passcode_is_a_secret);

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

            let identity_private_file_path_expanded = cargo_auto_encrypt_secret_lib::file_path_home_expand(identity_private_file_path);
            if !camino::Utf8Path::new(&identity_private_file_path_expanded).exists() {
                eprintln!("{RED}Identity file {identity_private_file_path_expanded} that contains the SSH private key does not exist! {RESET}");
                eprintln!("    {YELLOW}Create the SSH key manually in bash with this command:{RESET}");
                if identity_private_file_path_expanded.as_str().contains("github_api") {
                    eprintln!(r#"{GREEN}ssh-keygen -t ed25519 -f \"{identity_private_file_path_expanded}\" -C \"github api token\"{RESET}"#);
                } else if identity_private_file_path_expanded.as_str().contains("github_api") {
                    eprintln!(r#"{GREEN}ssh-keygen -t ed25519 -f \"{identity_private_file_path_expanded}\" -C \"crates io token\"{RESET}"#);
                }
                eprintln!(" ");
                panic!("{RED}Error: File {identity_private_file_path_expanded} does not exist! {RESET}");
            }

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
                    eprintln!("   {YELLOW}SSH key for encrypted token is not found in the ssh-agent.{RESET}");
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
}

pub(crate) mod github_mod {

    //! Every API call needs the GitHub API token. This is a secret important just like a password.
    //! I don't want to pass this secret to an "obscure" library crate that is difficult to review.
    //! This secret will stay here in this codebase that every developer can easily inspect.
    //! Instead of the token, I will pass the struct GitHubClient with the trait SendToGitHubApi.
    //! This way, the secret token will be encapsulated.

    use cargo_auto_github_lib as cgl;

    use cargo_auto_lib::BLUE;
    use cargo_auto_lib::RED;
    use cargo_auto_lib::RESET;

    use reqwest::Client;
    // bring trait into scope
    use secrecy::ExposeSecret;

    /// Struct GitHubClient contains only private fields
    /// This fields are accessible only to methods in implementation of traits.
    pub struct GitHubClient {
        /// Passcode for encrypt the token_is_a_secret to encrypted_token in memory.
        /// So that the secret is in memory as little as possible as plain text.
        /// For every session (program start) a new random passcode is created.
        session_passcode: secrecy::SecretVec<u8>,

        /// private field is set only once in the new() constructor
        encrypted_token: super::secrecy_mod::SecretEncryptedString,
    }

    impl GitHubClient {
        /// Create new GitHub client
        ///
        /// Interactively ask the user to input the GitHub token.
        pub fn new_interactive_input_token() -> Self {
            let mut github_client = Self::new_wo_token();

            println!("{BLUE}Enter the GitHub API token:{RESET}");
            github_client.encrypted_token =
                super::secrecy_mod::SecretEncryptedString::new_with_string(inquire::Password::new("").without_confirmation().prompt().unwrap(), &github_client.session_passcode);

            // return
            github_client
        }

        /// Create new GitHub client without token
        fn new_wo_token() -> Self {
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

        /// Use the stored API token
        ///
        /// If the token not exists ask user to interactively input the token.
        /// To decrypt it, use the SSH passphrase. That is much easier to type than typing the token.
        /// it is then possible also to have the ssh key in ssh-agent and write the passphrase only once.
        /// But this great user experience comes with security concerns. The token is accessible if the attacker is very dedicated.
        pub fn new_with_stored_token() -> Self {
            /// Internal function for DRY Don't Repeat Yourself
            fn read_token_and_decrypt_return_github_client(mut ssh_context: super::ssh_mod::SshContext, encrypted_string_file_path: &camino::Utf8Path) -> GitHubClient {
                // read the token and decrypt
                cargo_auto_encrypt_secret_lib::decrypt_with_ssh_interactive_from_file(&mut ssh_context, encrypted_string_file_path);
                let token_is_a_secret = ssh_context.get_decrypted_string();
                let mut github_client = GitHubClient::new_wo_token();
                github_client.encrypted_token = super::secrecy_mod::SecretEncryptedString::new_with_secret_string(token_is_a_secret, &github_client.session_passcode);
                github_client
            }

            let encrypted_string_file_path = camino::Utf8Path::new("~/.ssh/github_api_token_encrypted.txt");
            let encrypted_string_file_path_expanded = cargo_auto_encrypt_secret_lib::file_path_home_expand(encrypted_string_file_path);

            let identity_file_path = camino::Utf8Path::new("~/.ssh/github_api_token_ssh_1");
            if !encrypted_string_file_path_expanded.exists() {
                // ask interactive
                println!("    {BLUE}Do you want to store the GitHub API token encrypted with an SSH key? (y/n){RESET}");
                let answer = inquire::Text::new("").prompt().unwrap();
                if answer.to_lowercase() != "y" {
                    // enter the token manually, not storing
                    return Self::new_interactive_input_token();
                } else {
                    // get the passphrase and token interactively
                    let mut ssh_context = super::ssh_mod::SshContext::new();
                    // encrypt and save the encrypted token
                    cargo_auto_encrypt_secret_lib::encrypt_with_ssh_interactive_save_file(&mut ssh_context, identity_file_path, encrypted_string_file_path);
                    // read the token and decrypt, return GitHubClient
                    read_token_and_decrypt_return_github_client(ssh_context, encrypted_string_file_path)
                }
            } else {
                // file exists
                let ssh_context = super::ssh_mod::SshContext::new();
                // read the token and decrypt, return GitHubClient
                read_token_and_decrypt_return_github_client(ssh_context, encrypted_string_file_path)
            }
        }

        /// decrypts the secret token in memory
        pub fn decrypt_token_in_memory(&self) -> secrecy::SecretString {
            self.encrypted_token.expose_decrypted_secret(&self.session_passcode)
        }
    }

    /// trait from the crate library, so the 2 crates can share a function
    impl cgl::SendToGitHubApi for GitHubClient {
        /// Send GitHub API request
        ///
        /// This function encapsulates the secret API token.
        /// The RequestBuilder is created somewhere in the library crate.
        /// The client can be passed to the library. It will not reveal the secret token.
        fn send_to_github_api(&self, req: reqwest::blocking::RequestBuilder) -> serde_json::Value {
            // I must build the request to be able then to inspect it.
            let req = req.bearer_auth(self.decrypt_token_in_memory().expose_secret()).build().unwrap();

            // region: Assert the correct url and https
            // It is important that the request coming from a external crate/library
            // is only sent always and only to GitHub API and not some other malicious url,
            // because the request contains the secret GitHub API token.
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
        /// This function encapsulates the secret API token.
        /// The RequestBuilder is created somewhere in the library crate.
        /// The client can be passed to the library. It will not reveal the secret token.
        /// This is basically an async fn, but use of `async fn` in public traits is discouraged...
        async fn upload_to_github(&self, req: reqwest::RequestBuilder) -> serde_json::Value {
            // I must build the request to be able then to inspect it.
            let req = req.bearer_auth(self.decrypt_token_in_memory().expose_secret()).build().unwrap();

            // region: Assert the correct url and https
            // It is important that the request coming from a external crate/library
            // is only sent always and only to GitHub uploads and not some other malicious url,
            // because the request contains the secret GitHub API token.
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

pub(crate) mod crate_io_mod {

    //! Publish to crates.io needs the crates.io token. This is a secret important just like a password.
    //! I don't want to pass this secret to an "obscure" library crate that is difficult to review.
    //! This secret will stay here in this codebase that every developer can easily inspect.
    //! Instead of the token, I will pass the struct CratesIoClient with the trait SendToCratesIo.
    //! This way, the secret token will be encapsulated.

    use cargo_auto_lib::BLUE;
    use cargo_auto_lib::RED;
    use cargo_auto_lib::RESET;
    use cargo_auto_lib::YELLOW;

    // bring trait into scope
    use secrecy::ExposeSecret;

    /// Struct CratesIoClient contains only private fields
    /// This fields are accessible only to methods in implementation of traits.
    pub struct CratesIoClient {
        /// Passcode for encrypt the token_is_a_secret to encrypted_token in memory.
        /// So that the secret is in memory as little as possible as plain text.
        /// For every session (program start) a new random passcode is created.
        session_passcode: secrecy::SecretVec<u8>,

        /// private field is set only once in the new() constructor
        encrypted_token: super::secrecy_mod::SecretEncryptedString,
    }

    impl CratesIoClient {
        /// Create new CratesIo client
        ///
        /// Interactively ask the user to input the crates.io token.
        #[allow(dead_code)]
        pub fn new_interactive_input_token() -> Self {
            let mut crates_io_client = Self::new_wo_token();

            println!("{BLUE}Enter the crates.io token:{RESET}");
            crates_io_client.encrypted_token =
                super::secrecy_mod::SecretEncryptedString::new_with_string(inquire::Password::new("").without_confirmation().prompt().unwrap(), &crates_io_client.session_passcode);

            // return
            crates_io_client
        }

        /// Create new CratesIo client without token
        #[allow(dead_code)]
        fn new_wo_token() -> Self {
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

        /// Use the stored crates.io token
        ///
        /// If the token not exists ask user to interactively input the token.
        /// To decrypt it, use the SSH passphrase. That is much easier to type than typing the token.
        /// It is then possible also to have the ssh key in ssh-agent and write the passphrase only once.
        /// But this great user experience comes with security concerns. The token is accessible if the attacker is very dedicated.
        #[allow(dead_code)]
        pub fn new_with_stored_token() -> Self {
            /// Internal function for DRY Don't Repeat Yourself
            fn read_token_and_decrypt_return_crate_io_client(mut ssh_context: super::ssh_mod::SshContext, encrypted_string_file_path: &camino::Utf8Path) -> CratesIoClient {
                cargo_auto_encrypt_secret_lib::decrypt_with_ssh_interactive_from_file(&mut ssh_context, encrypted_string_file_path);
                let token_is_a_secret = ssh_context.get_decrypted_string();
                let mut crates_io_client = CratesIoClient::new_wo_token();
                crates_io_client.encrypted_token = super::secrecy_mod::SecretEncryptedString::new_with_secret_string(token_is_a_secret, &crates_io_client.session_passcode);
                crates_io_client
            }

            let encrypted_string_file_path = camino::Utf8Path::new("~/.ssh/crates_io_token_encrypted.txt");
            let encrypted_string_file_path_expanded = cargo_auto_encrypt_secret_lib::file_path_home_expand(encrypted_string_file_path);

            let identity_file_path = camino::Utf8Path::new("~/.ssh/crates_io_token_ssh_1");
            if !encrypted_string_file_path_expanded.exists() {
                // ask interactive
                println!("    {BLUE}Do you want to store the crates.io token encrypted with an SSH key? (y/n){RESET}");
                let answer = inquire::Text::new("").prompt().unwrap();
                if answer.to_lowercase() != "y" {
                    // enter the token manually, not storing
                    return Self::new_interactive_input_token();
                } else {
                    // get the passphrase and token interactively
                    let mut ssh_context = super::ssh_mod::SshContext::new();
                    // encrypt and save the encrypted token
                    cargo_auto_encrypt_secret_lib::encrypt_with_ssh_interactive_save_file(&mut ssh_context, identity_file_path, encrypted_string_file_path);
                    // read the token and decrypt, return CratesIoClient
                    read_token_and_decrypt_return_crate_io_client(ssh_context, encrypted_string_file_path)
                }
            } else {
                // file exists
                let ssh_context = super::ssh_mod::SshContext::new();
                // read the token and decrypt, return CratesIoClient
                read_token_and_decrypt_return_crate_io_client(ssh_context, encrypted_string_file_path)
            }
        }

        /// decrypts the secret token in memory
        pub fn decrypt_token_in_memory(&self) -> secrecy::SecretString {
            self.encrypted_token.expose_decrypted_secret(&self.session_passcode)
        }

        /// Publish to crates.io
        ///
        /// This function encapsulates the secret crates.io token.
        /// The client can be passed to the library. It will not reveal the secret token.
        pub fn publish_to_crates_io(&self) {
            // print command without the token
            println!("{YELLOW}cargo publish --token [REDACTED]{RESET}");
            let shell_command = format!("cargo publish --token {}", self.decrypt_token_in_memory().expose_secret());
            let status = std::process::Command::new("sh").arg("-c").arg(shell_command).spawn().unwrap().wait().unwrap();
            let exit_code = status.code().expect(&format!("{RED}Error: publish to crates.io error. {RESET}"));
            if exit_code != 0 {
                panic!("{RED}Error: publish to crates.io error {exit_code}. {RESET}");
            }
        }
    }
}
