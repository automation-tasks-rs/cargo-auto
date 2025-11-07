// template_new_pwa_wasm_mod.rs

//! Template for new_pwa_wasm.
//!
//! The template is downloaded from github:  
//! <https://github.com/automation-tasks-rs/cargo_auto_template_new_pwa_wasm/releases/latest/download/template.tar.gz>

use crate::{pos, ResultLogError, GREEN, RED, RESET, YELLOW};

/// Creates a new Rust project from template.
pub fn new_pwa_wasm(
    rust_project_name: Option<String>,
    github_owner_or_organization: Option<String>,
    web_server_domain: Option<String>,
    server_username: Option<String>,
) -> anyhow::Result<()> {
    /// internal function: favicon.ico with 16 and 32 icons.
    fn encode_to_favicon_ico(img: &image::DynamicImage, rust_project_name: &str) -> anyhow::Result<()> {
        /// internal function: favicon
        fn favicon_add_entry(img: &image::DynamicImage, size: u32, icon_dir: &mut ico::IconDir) -> anyhow::Result<()> {
            // icons need smaller images 48, 32 and 16
            let img_rgba_vec = img
                .resize(size, size, image::imageops::FilterType::Lanczos3)
                .into_rgba8()
                .into_raw();
            // create an IconImage from raw RGBA pixel data from another image library
            let icon_image = ico::IconImage::from_rgba_data(size, size, img_rgba_vec);
            icon_dir.add_entry(ico::IconDirEntry::encode(&icon_image)?);
            Ok(())
        }
        // Create a new, empty icon collection:
        let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
        favicon_add_entry(img, 16, &mut icon_dir).log(pos!())?;
        favicon_add_entry(img, 32, &mut icon_dir).log(pos!())?;
        favicon_add_entry(img, 48, &mut icon_dir).log(pos!())?;
        let file_name = format!("{rust_project_name}/web_server_folder/{rust_project_name}/favicon.ico");
        let buffer = std::fs::File::create(file_name).log(pos!())?;
        icon_dir.write(buffer).log(pos!())?;
        Ok(())
    }

    /// Internal function: decode png.
    fn decode_png(vec: Vec<u8>) -> anyhow::Result<image::DynamicImage> {
        let img = image::ImageReader::new(std::io::Cursor::new(vec));
        let img = img.with_guessed_format().log(pos!())?;
        // return img
        Ok(img.decode()?)
    }

    /// Internal function: resize img.
    fn resize_image(img: &image::DynamicImage, img_size: u32, file_name: &str, rust_project_name: &str) -> anyhow::Result<()> {
        /// internal function: encode to png
        fn encode_to_png(new_img: image::DynamicImage) -> anyhow::Result<Vec<u8>> {
            let vec_u8: Vec<u8> = Vec::new();
            let mut cursor_1 = std::io::Cursor::new(vec_u8);
            new_img.write_to(&mut cursor_1, image::ImageFormat::Png).log(pos!())?;
            // return
            Ok(cursor_1.get_ref().to_owned())
        }
        let new_img = img.resize(img_size, img_size, image::imageops::FilterType::Lanczos3);
        let vec_u8 = encode_to_png(new_img).log(pos!())?;

        let file_name = format!("{rust_project_name}/web_server_folder/{rust_project_name}/icons/{file_name}");
        std::fs::write(file_name, vec_u8).log(pos!())?;
        Ok(())
    }

    if !std::path::Path::new("icon512x512.png").exists() {
        println!(
            r#"
{RED}The mandatory file `icon512x512.png` is not found in this folder.{RESET}
  {YELLOW}If you don't have your icon, you can download and use this default:{RESET}
{GREEN}curl -L https://github.com/automation-tasks-rs/cargo_auto_template_new_pwa_wasm/raw/main/icon512x512.png --output icon512x512.png{RESET}
"#
        );
        return Ok(());
    }

    if rust_project_name.is_none() {
        println!("{RED}Error: Project name argument is missing: `cargo auto new_pwa_wasm project_name github_owner_or_organization web_server server_username`{RESET}");
        return Ok(());
    }
    if github_owner_or_organization.is_none() {
        println!("{RED}Error: github_owner or Organization argument is missing: `cargo auto new_pwa_wasm project_name github_owner_or_organization web_server server_username`{RESET}");
        return Ok(());
    }
    if web_server_domain.is_none() {
        println!("{RED}Error: Web server argument is missing: `cargo auto new_pwa_wasm project_name github_owner_or_organization web_server server_username`{RESET}");
        return Ok(());
    }
    if server_username.is_none() {
        println!("{RED}Error: Server username argument is missing: `cargo auto new_pwa_wasm project_name github_owner_or_organization web_server server_username`{RESET}");
        return Ok(());
    }
    use anyhow::Context;
    let rust_project_name = rust_project_name.context("rust_project_name is None").log(pos!())?;
    let github_owner_or_organization = github_owner_or_organization
        .context("github_owner_or_organization is None")
        .log(pos!())?;
    let web_server_domain = web_server_domain.context("web_server_domain is None").log(pos!())?;
    let server_username = server_username.context("server_username is None").log(pos!())?;

    // the icon exist, already checked
    copy_to_files(
        &rust_project_name,
        &github_owner_or_organization,
        &web_server_domain,
        &server_username,
    )
    .log(pos!())?;

    // region: png with various sizes for: favicon png, pwa Android and pwa iOS
    // 32, 72, 96, 120, 128, 144, 152, 167, 180, 192, 196, 512
    let img = std::fs::read("icon512x512.png").log(pos!())?;
    let img = decode_png(img).log(pos!())?;

    resize_image(&img, 32, "icon-032.png", &rust_project_name).log(pos!())?;
    resize_image(&img, 72, "icon-072.png", &rust_project_name).log(pos!())?;
    resize_image(&img, 96, "icon-096.png", &rust_project_name).log(pos!())?;
    resize_image(&img, 120, "icon-120.png", &rust_project_name).log(pos!())?;
    resize_image(&img, 128, "icon-128.png", &rust_project_name).log(pos!())?;
    resize_image(&img, 144, "icon-144.png", &rust_project_name).log(pos!())?;
    resize_image(&img, 152, "icon-152.png", &rust_project_name).log(pos!())?;
    resize_image(&img, 167, "icon-167.png", &rust_project_name).log(pos!())?;
    resize_image(&img, 180, "icon-180.png", &rust_project_name).log(pos!())?;
    resize_image(&img, 192, "icon-192.png", &rust_project_name).log(pos!())?;
    resize_image(&img, 196, "icon-196.png", &rust_project_name).log(pos!())?;
    // overwrite the default with the new
    resize_image(&img, 512, "icon-512.png", &rust_project_name).log(pos!())?;

    // maskable icon 192
    resize_image(&img, 192, "icon-maskable.png", &rust_project_name).log(pos!())?;

    // favicon.ico with 16, 32 and 48 icons
    encode_to_favicon_ico(&img, &rust_project_name).log(pos!())?;

    // endregion
    println!(
        r#"
  {YELLOW}You can open this new Rust project in VSCode:{RESET}
{GREEN}code {package_name}{RESET}
  {YELLOW}Then build it with:{RESET}
{GREEN}cargo auto build{RESET}
  {YELLOW}Then follow the detailed instructions.{RESET}
"#,
        package_name = rust_project_name
    );
    Ok(())
}

/// Copy the Rust project into a compressed file.  
fn copy_to_files(
    rust_project_name: &str,
    github_owner_or_organization: &str,
    web_server_domain: &str,
    server_username: &str,
) -> anyhow::Result<()> {
    let folder_path = std::path::Path::new(rust_project_name);
    if folder_path.exists() {
        anyhow::bail!("{RED}Error: Folder {rust_project_name} already exists! {RESET}");
    }
    std::fs::create_dir_all(folder_path).log(pos!())?;

    // download latest template.tar.gz
    println!("  {YELLOW}Downloading template.tar.gz...{RESET}");
    let file_name = "template.tar.gz";
    let path = "./template.tar.gz";
    let url = "https://github.com/automation-tasks-rs/cargo_auto_template_new_pwa_wasm/releases/latest/download/template.tar.gz";
    let reqwest_client = reqwest::blocking::Client::new();
    let http_response = reqwest_client.get(url).send();

    if let Ok(body) = http_response {
        let body = body.bytes().log(pos!())?;
        // Get the content of the response
        std::fs::write(path, &body)
            .or_else(|err| anyhow::bail!("Download failed for {file_name} {err}"))
            .log(pos!())?;
    } else {
        anyhow::bail!("Error while retrieving data: {:#?}", http_response.err());
    }

    // decompress into folder_path
    let tar_gz = std::fs::File::open(path).log(pos!())?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(folder_path).log(pos!())?;
    std::fs::remove_file(path).log(pos!())?;

    // replace placeholders inside text files
    for entry in walkdir::WalkDir::new(folder_path).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            if entry.file_name().to_string_lossy().ends_with(".ico")
                || entry.file_name().to_string_lossy().ends_with(".png")
                || entry.file_name().to_string_lossy().ends_with(".woff2")
            {
                // cannot replace text in binary files
            } else {
                // template has only valid utf8 files
                println!("replace: {}", entry.path().to_string_lossy());
                let content = std::fs::read_to_string(entry.path()).log(pos!())?;
                let content = content.replace("cargo_auto_template_new_pwa_wasm", rust_project_name);
                let content = content.replace(
                    &"cargo_auto_template_new_pwa_wasm".to_uppercase(),
                    &rust_project_name.to_uppercase(),
                );
                let content = content.replace("automation-tasks-rs", github_owner_or_organization);
                let content = content.replace("automation--tasks--rs", "automation-tasks-rs");
                let content = content.replace("web_server_domain", web_server_domain);
                let content = content.replace("server_username", server_username);
                std::fs::write(entry.path(), content).log(pos!())?;
            }
        }
    }
    // renaming files is tricky and must be traverse in reverse.
    let mut traverse_reverse: Vec<walkdir::DirEntry> = walkdir::WalkDir::new(folder_path).into_iter().filter_map(Result::ok).collect();
    traverse_reverse.reverse();
    for entry in traverse_reverse.iter() {
        if entry.file_name().to_string_lossy().contains("cargo_auto_template_new_pwa_wasm") {
            println!("rename: {}", entry.path().to_string_lossy());
            std::fs::rename(
                entry.path(),
                entry
                    .path()
                    .to_string_lossy()
                    .replace("cargo_auto_template_new_pwa_wasm", rust_project_name),
            )
            .log(pos!())?;
        }
    }
    Ok(())
}
