// cargo_auto_github_api_mod

#![allow(dead_code)]
use cargo_auto_lib as cl;
// traits must be in scope (Rust strangeness)
use cl::CargoTomlPublicApiMethods;

use cl::ShellCommandLimitedDoubleQuotesSanitizerTrait;
use cl::BLUE;
use cl::RED;
use cl::RESET;
use cl::YELLOW;

use crate::encrypt_decrypt_with_ssh_key_mod::github_api_token_with_oauth2_mod::send_to_github_api_with_secret_token;
use crate::encrypt_decrypt_with_ssh_key_mod::github_api_token_with_oauth2_mod::upload_to_github_with_secret_token;

/// Does git have settings for remote.
pub(crate) fn git_has_remote() -> bool {
    // git remote returns only "origin" if exists or nothing if it does not exist
    let output = std::process::Command::new("git").arg("remote").output().unwrap();
    // return
    String::from_utf8(output.stdout).unwrap() != ""
}

/// Has git upstream
pub(crate) fn git_has_upstream() -> bool {
    // git branch -vv returns upstream branches in angle brackets []
    let output = std::process::Command::new("git").arg("branch").arg("-vv").output().unwrap();
    // return
    String::from_utf8(output.stdout).unwrap().contains("[")
}

/// Interactive ask to create a new remote GitHub repository.
///
/// Use a function pointer to send_to_github_api_with_secret_token() to avoid passing the secret_token.
pub(crate) fn new_remote_github_repository() -> Option<()> {
    // early error if Repository contains the placeholder "github_owner" or does not contain the true github_owner
    let cargo_toml = cl::CargoToml::read();
    let package_name = cargo_toml.package_name();
    // the second fragment of URL can be the github_owner (authenticated_user) or organization
    let github_owner_or_organization = cargo_toml
        .github_owner()
        .unwrap_or_else(|| panic!("{RED}ERROR: Element Repository in Cargo.toml does not contain the github_owner!{RESET}"));
    if github_owner_or_organization == "github_owner" {
        panic!("{RED}Error: The placeholder 'github_owner' in Cargo.toml/repository is not changed to the real github_owner or GitHub Organization.{RESET}")
    }

    // get authenticated user from Github
    let json_value = send_to_github_api_with_secret_token(github_api_get_authenticated_user()).unwrap();
    let Some(authenticated_user_login) = json_value.get("login") else {
        panic!("{RED}ERROR: Unrecognized Authenticated on GitHub from secret_token.{RESET}");
    };
    let authenticated_user_login = authenticated_user_login.as_str().unwrap();

    if github_owner_or_organization == authenticated_user_login {
        // this repository is a User Repository
    } else {
        // check if it is a GitHub Organization
        let json_value = send_to_github_api_with_secret_token(github_api_get_organization(&github_owner_or_organization)).unwrap();
        let Some(_organization_login) = json_value.get("login") else {
            panic!("{RED}ERROR: Unrecognized Organization on GitHub: {github_owner_or_organization}.{RESET}");
        };
    }

    if !git_has_remote() {
        let description = cargo_toml
            .package_description()
            .unwrap_or_else(|| panic!("{RED}ERROR: Element Description in Cargo.toml does not exist!{RESET}"));

        // ask interactive
        println!("{BLUE}This project does not have a remote GitHub repository.{RESET}");
        let answer = cl::inquire::Text::new(&format!("{BLUE}Do you want to create a new remote GitHub repository? (y/n){RESET}"))
            .prompt()
            .unwrap();
        if answer.to_lowercase() != "y" {
            // early exit
            return None;
        }
        // continue if answer is "y"

        let json_value = if github_owner_or_organization == authenticated_user_login {
            // new User repository
            let json_value = send_to_github_api_with_secret_token(github_api_user_repository_new(
                &github_owner_or_organization,
                &package_name,
                &description,
            ))
            .unwrap();
            // early exit on error
            if let Some(error_message) = json_value.get("message") {
                eprintln!("{RED}{error_message}{RESET}");
                if let Some(errors) = json_value.get("errors") {
                    let errors = errors.as_array().unwrap();
                    for error in errors.iter() {
                        if let Some(code) = error.get("message") {
                            eprintln!("{RED}{code}{RESET}");
                        }
                    }
                }
                panic!("{RED}Call to GitHub API github_api_user_repository_new returned an error.{RESET}")
            }
            json_value
        } else {
            // new Organization repository
            let json_value = send_to_github_api_with_secret_token(github_api_organization_repository_new(
                &github_owner_or_organization,
                &package_name,
                &description,
            ))
            .unwrap();
            // early exit on error
            if let Some(error_message) = json_value.get("message") {
                eprintln!("{RED}{error_message}{RESET}");
                if let Some(errors) = json_value.get("errors") {
                    let errors = errors.as_array().unwrap();
                    for error in errors.iter() {
                        if let Some(code) = error.get("message") {
                            eprintln!("{RED}{code}{RESET}");
                        }
                    }
                }
                panic!("{RED}Call to GitHub API github_api_organization_repository_new returned an error.{RESET}")
            }
            json_value
        };

        // get just the name, description and html_url from json
        println!("  {YELLOW}name: {}{RESET}", json_value.get("name").unwrap().as_str().unwrap());
        println!(
            "  {YELLOW}description: {}{RESET}",
            json_value.get("description").unwrap().as_str().unwrap()
        );
        let repo_html_url = json_value.get("html_url").unwrap().as_str().unwrap().to_string();
        println!("  {YELLOW}url: {}{RESET}", &repo_html_url);

        // add this GitHub repository to origin remote over SSH (use sshadd for passphrase)
        cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
            r#"git remote add origin "git@github.com:{github_owner_or_organization}/{name}.git" "#,
        )
        .unwrap()
        .arg("{github_owner_or_organization}", &github_owner_or_organization)
        .unwrap()
        .arg("{name}", &package_name)
        .unwrap()
        .run()
        .unwrap();
    }

    if !git_has_upstream() {
        cl::run_shell_command("git push -u origin main").unwrap_or_else(|e| panic!("{e}"));

        // the docs pages are created with a GitHub action
        let _json =
            send_to_github_api_with_secret_token(github_api_create_a_github_pages_site(&github_owner_or_organization, &package_name));
    }

    Some(())
}

/// Check and modify the description and topics on Github
///
/// The words topics, keywords and tags all mean the same concept.
/// In cargo.toml we have keywords.
/// In README.md I want to have badges for tags
/// In GitHub they are topics.
/// Topic must be only one word: lowercase letters, hyphens(-) or numbers, less then 35 characters.
/// I want to avoid GitHub API at every git push. I will store the old description and topics
/// in the file automation_tasks_rs/.old_metadata.json
/// So I can compare first locally and only when they differ call the Github API.
pub(crate) fn description_and_topics_to_github() {
    let cargo_toml = cl::CargoToml::read();
    let repo_name = cargo_toml.package_name();
    let github_owner_or_organization = cargo_toml.github_owner().unwrap();
    let description = cargo_toml.package_description().unwrap();
    let keywords = cargo_toml.package_keywords();

    #[derive(serde::Serialize, serde::Deserialize)]
    struct OldMetadata {
        old_description: String,
        old_keywords: Vec<String>,
    }

    // read data from automation_tasks_rs/.old_metadata.json
    let mut is_old_metadata_different = true;
    if let Ok(old_metadata) = std::fs::read_to_string("automation_tasks_rs/.old_metadata.json") {
        if let Ok(old_metadata) = serde_json::from_str::<OldMetadata>(&old_metadata) {
            if old_metadata.old_description == description && old_metadata.old_keywords == keywords {
                is_old_metadata_different = false;
            }
        }
    }

    if is_old_metadata_different {
        // get data from GitHub
        let json = send_to_github_api_with_secret_token(github_api_get_repository(&github_owner_or_organization, &repo_name)).unwrap();

        // get just the description and topis from json
        let gh_description = json.get("description").unwrap().as_str().unwrap();
        let gh_topics = json.get("topics").unwrap().as_array().unwrap();
        let gh_topics: Vec<String> = gh_topics.iter().map(|value| value.as_str().unwrap().to_string()).collect();

        // are description and topics both equal?
        if gh_description != description {
            let _json = send_to_github_api_with_secret_token(github_api_update_description(
                &github_owner_or_organization,
                &repo_name,
                &description,
            ));
        }

        // all elements must be equal, but not necessary in the same order
        let topics_is_equal = if gh_topics.len() == keywords.len() {
            let mut elements_is_equal = true;
            'outer: for x in gh_topics.iter() {
                let mut has_element = false;
                'inner: for y in keywords.iter() {
                    if y == x {
                        has_element = true;
                        break 'inner;
                    }
                }
                if !has_element {
                    elements_is_equal = false;
                    break 'outer;
                }
            }
            elements_is_equal
        } else {
            false
        };

        if !topics_is_equal {
            let _json =
                send_to_github_api_with_secret_token(github_api_replace_all_topics(&github_owner_or_organization, &repo_name, &keywords));
            // write into automation_tasks_rs/.old_metadata.json file
            let old_metadata = OldMetadata {
                old_description: description,
                old_keywords: keywords,
            };
            std::fs::write(
                "automation_tasks_rs/.old_metadata.json",
                serde_json::to_string_pretty(&old_metadata).unwrap(),
            )
            .unwrap();
        }
    }
}

/// GitHub api get authenticated user
pub(crate) fn github_api_get_authenticated_user() -> reqwest::blocking::RequestBuilder {
    /*
        https://docs.github.com/en/rest/users/users?apiVersion=2022-11-28#get-the-authenticated-user

        curl -L \
        -H "Accept: application/vnd.github+json" \
        -H "Authorization: Bearer <YOUR-TOKEN>" \
        -H "X-GitHub-Api-Version: 2022-11-28" \
        https://api.github.com/user

        {
        "login": "bestia-dev",
        "id": 1,
        }
    */
    let repos_url = "https://api.github.com/user".to_string();
    // return
    reqwest::blocking::Client::new()
        .get(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
}

/// GitHub api get organization
pub(crate) fn github_api_get_organization(organization: &str) -> reqwest::blocking::RequestBuilder {
    /*
        https://docs.github.com/en/rest/orgs/orgs?apiVersion=2022-11-28#get-an-organization

        curl -L \
        -H "Accept: application/vnd.github+json" \
        -H "Authorization: Bearer <YOUR-TOKEN>" \
        -H "X-GitHub-Api-Version: 2022-11-28" \
        https://api.github.com/orgs/ORG

        {
        "login": "github",
        "id": 1,
        }
    */
    let repos_url = format!("https://api.github.com/orgs/{organization}");
    // return
    reqwest::blocking::Client::new()
        .get(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
}

/// GitHub api get repository
pub(crate) fn github_api_get_repository(github_owner_or_organization: &str, repo_name: &str) -> reqwest::blocking::RequestBuilder {
    /*
        https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#get-a-repository

        curl -L \
        -H "Accept: application/vnd.github+json" \
        -H "Authorization: Bearer <YOUR-TOKEN>" \
        -H "X-GitHub-Api-Version: 2022-11-28" \
        https://api.github.com/repos/github_owner/REPO
    */
    let repos_url = format!("https://api.github.com/repos/{github_owner_or_organization}/{repo_name}");
    // return
    reqwest::blocking::Client::new()
        .get(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
}

/// Create a new github User repository
/// TODO: slightly different API call for organization repository. How to distinguish user and organization?
pub(crate) fn github_api_user_repository_new(github_owner: &str, name: &str, description: &str) -> reqwest::blocking::RequestBuilder {
    /*
    https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#create-a-repository-for-the-authenticated-user

    Request like :
    curl -L \
    -X POST \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer <YOUR-TOKEN>" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    https://api.github.com/user/repos \
    -d '{
        "name":"Hello-World",
        "description":"This is your first repo!",
        "homepage":"https://github.com",
        "private":false,
        "is_template":true
    }'

    Response (short)
    {
    "id": 1296269,
    ...
    }
    */
    let repos_url = "https://api.github.com/user/repos".to_string();
    let body = serde_json::json!({
        "name": name,
        "description": description,
        "homepage": format!("https://{github_owner}.github.io/{name}"),
        "private":false,
        "has_issues":true,
        "has_projects":false,
        "has_wiki":false,
        // more settings...
        "has_discussions" :true
    });
    // Sadly there is no way in the API to set the settings: releases, packages and deployments
    let body = body.to_string();

    reqwest::blocking::Client::new()
        .post(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
        .body(body)
}

/// Create a new github organization repository
pub(crate) fn github_api_organization_repository_new(
    organization: &str,
    name: &str,
    description: &str,
) -> reqwest::blocking::RequestBuilder {
    /*
    https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#create-a-repository-for-the-authenticated-user

    Request like :
    curl -L \
    -X POST \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer <YOUR-TOKEN>" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    https://api.github.com/orgs/ORG/repos \
    -d '{
        "name":"Hello-World",
        "description":"This is your first repository",
        "homepage":"https://github.com",
        "private":false,
        "has_issues":true,
        "has_projects":true,
        "has_wiki":true
    }'

    Response (short)
    {
    "id": 1296269,
    ...
    }
    */
    let repos_url = format!("https://api.github.com/orgs/{organization}/repos");
    let body = serde_json::json!({
        "name": name,
        "description": description,
        "homepage": format!("https://{organization}.github.io/{name}"),
        "private":false,
        "has_issues":true,
        "has_projects":false,
        "has_wiki":false,
        // more settings...
        "has_discussions" :true
    });
    // Sadly there is no way in the API to set the settings: releases, packages and deployments
    let body = body.to_string();

    reqwest::blocking::Client::new()
        .post(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
        .body(body)
}

/// GitHub api update description
pub(crate) fn github_api_update_description(
    github_owner_or_organization: &str,
    repo_name: &str,
    description: &str,
) -> reqwest::blocking::RequestBuilder {
    /*
    https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#update-a-repository

    curl -L \
    -X PATCH \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer <YOUR-TOKEN>" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    https://api.github.com/repos/github_owner/REPO \
    -d '{
        "name":"Hello-World",
        "description":"This is your first repository",
        "homepage":"https://github.com",
        "private":true,
        "has_issues":true,
        "topics": [
            "cat",
            "atom",
            "electron",
            "api"
            ],
        "has_projects":true,
        "has_wiki":true}'

    Response (short)
    {
    "id": 1296269,
    ...
    }
    */
    let repos_url = format!("https://api.github.com/repos/{github_owner_or_organization}/{repo_name}");
    let body = serde_json::json!({
        "description": description,
    });
    let body = body.to_string();

    reqwest::blocking::Client::new()
        .patch(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
        .body(body)
}

/// GitHub API replace all topics
pub(crate) fn github_api_replace_all_topics(
    github_owner_or_organization: &str,
    repo_name: &str,
    topics: &Vec<String>,
) -> reqwest::blocking::RequestBuilder {
    /*
    https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#replace-all-repository-topics
    curl -L \
      -X PUT \
      -H "Accept: application/vnd.github+json" \
      -H "Authorization: Bearer <YOUR-TOKEN>" \
      -H "X-GitHub-Api-Version: 2022-11-28" \
      https://api.github.com/repos/github_owner/REPO/topics \
      -d '{"names":["cat","atom","electron","api"]}'
     */
    let repos_url = format!("https://api.github.com/repos/{github_owner_or_organization}/{repo_name}/topics");
    let body = serde_json::json!({
        "names": topics,
    });
    let body = body.to_string();

    reqwest::blocking::Client::new()
        .put(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
        .body(body)
}

/// GitHub API create-a-github-pages-site
pub(crate) fn github_api_create_a_github_pages_site(
    github_owner_or_organization: &str,
    repo_name: &str,
) -> reqwest::blocking::RequestBuilder {
    /*
        https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#create-a-github-pages-site
        curl -L \
        -X POST \
        -H "Accept: application/vnd.github+json" \
        -H "Authorization: Bearer <YOUR-TOKEN>" \
        -H "X-GitHub-Api-Version: 2022-11-28" \
        https://api.github.com/repos/github_owner/REPO/pages \
        -d '
    {
        "source": {
            "branch": "main",
            "path": "/docs",
            "build_type": "workflow"
        }
    }'
         */
    let repos_url = format!("https://api.github.com/repos/{github_owner_or_organization}/{repo_name}/pages");
    let body = serde_json::json!({
        "build_type": "workflow",
        "source": {
            "branch": "main",
            "path": "/docs"
        }
    });
    let body = body.to_string();

    reqwest::blocking::Client::new()
        .post(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
        .body(body)
}

/// Upload asset to github release  
pub(crate) fn github_api_upload_asset_to_release(github_owner_or_organization: &str, repo: &str, release_id: &str, path_to_file: &str) {
    println!("  {YELLOW}Uploading file to GitHub release: {path_to_file}{RESET}");
    let file = camino::Utf8Path::new(&path_to_file);
    let file_name = file.file_name().unwrap();

    let release_upload_url = format!("https://uploads.github.com/repos/{github_owner_or_organization}/{repo}/releases/{release_id}/assets");
    let mut release_upload_url = <url::Url as std::str::FromStr>::from_str(&release_upload_url).unwrap();
    release_upload_url.set_query(Some(format!("{}={}", "name", file_name).as_str()));
    let file_size = std::fs::metadata(file).unwrap().len();
    println!("  {YELLOW}It can take some time to upload. File size: {file_size}. Wait...{RESET}");
    // region: async code made sync locally
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        let file = tokio::fs::File::open(file).await.unwrap();
        let stream = tokio_util::codec::FramedRead::new(file, tokio_util::codec::BytesCodec::new());
        let body = reqwest::Body::wrap_stream(stream);

        let req = reqwest::Client::new()
            .post(release_upload_url.as_str())
            .header("Content-Type", "application/octet-stream")
            .header("Content-Length", file_size.to_string())
            .body(body);

        let _ = upload_to_github_with_secret_token(req).await;
    });
    // endregion: async code made sync locally
}

/// Create new release on Github
pub(crate) fn github_api_create_new_release(
    github_owner_or_organization: &str,
    repo: &str,
    tag_name_version: &str,
    name: &str,
    branch: &str,
    body_md_text: &str,
) -> reqwest::blocking::RequestBuilder {
    /*
    https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#create-a-release
    Request like :
    curl -L \
    -X POST \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer <YOUR-TOKEN>"\
    -H "X-GitHub-Api-Version: 2022-11-28" \
    https://api.github.com/repos/github_owner/REPO/releases \
    -d '
    {
        "tag_name":"v1.0.0",
        "target_commitish":"master",
        "name":"v1.0.0",
        "body":"Description of the release",
        "draft":false,
        "prerelease":false,
        "generate_release_notes":false
    }'

    Response (short)
    {
    "id": 1,
    ...
    }
    */
    let releases_url = format!("https://api.github.com/repos/{github_owner_or_organization}/{repo}/releases");
    let body = serde_json::json!({
        "tag_name": tag_name_version,
        "target_commitish":branch,
        "name":name,
        "body":body_md_text,
        "draft":false,
        "prerelease":false,
        "generate_release_notes":false,
    });
    let body = serde_json::to_string_pretty(&body).unwrap();
    reqwest::blocking::Client::new()
        .post(releases_url.as_str())
        .header("Content-Type", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
        .body(body)
}
