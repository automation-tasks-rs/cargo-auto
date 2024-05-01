// template_new_pwa_wasm_mod.rs

//! template for new_pwa_wasm
//!
//! The template is downloaded from github
//! <https://github.com/automation-tasks-rs/cargo_auto_template_new_pwa_wasm/releases/download/v0.0.5/template.tar.gz>


use crate::{GREEN, RESET, YELLOW,RED};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PwaJson5 {
    rust_project_name: String,
    github_owner_or_organization: String,
    pwa_description: String,
    web_server_domain: String,
    server_username: String,
}

pub fn new_pwa_wasm() {
    if std::path::Path::new("pwa.json5").exists() && std::path::Path::new("icon512x512.png").exists() {
        // both exist, read the json5 and run the generator
        let pwa_json5: PwaJson5 = json5::from_str(&std::fs::read_to_string("pwa.json5").unwrap()).unwrap();
        copy_to_files(&pwa_json5);

        // region: png with various sizes for: favicon png, pwa Android and pwa iOS
        // 32, 72, 96, 120, 128, 144, 152, 167, 180, 192, 196, 512
        let img = std::fs::read("icon512x512.png").unwrap();
        let img = decode_png(img);

        resize_image(&img, 32, "icon-032.png", &pwa_json5.rust_project_name, );
        resize_image(&img, 72, "icon-072.png", &pwa_json5.rust_project_name, );
        resize_image(&img, 96, "icon-096.png", &pwa_json5.rust_project_name, );
        resize_image(&img, 120, "icon-120.png", &pwa_json5.rust_project_name, );
        resize_image(&img, 128, "icon-128.png", &pwa_json5.rust_project_name, );
        resize_image(&img, 144, "icon-144.png", &pwa_json5.rust_project_name, );
        resize_image(&img, 152, "icon-152.png", &pwa_json5.rust_project_name, );
        resize_image(&img, 167, "icon-167.png", &pwa_json5.rust_project_name, );
        resize_image(&img, 180, "icon-180.png", &pwa_json5.rust_project_name, );
        resize_image(&img, 192, "icon-192.png", &pwa_json5.rust_project_name, );
        resize_image(&img, 196, "icon-196.png", &pwa_json5.rust_project_name, );
        // overwrite the default with the new
        resize_image(&img, 512, "icon-512.png", &pwa_json5.rust_project_name, );

        // maskable icon 192
        resize_image(&img, 192, "icon-maskable.png", &pwa_json5.rust_project_name, );

        // favicon.ico with 16, 32 and 48 icons
        encode_to_favicon_ico(&img, &pwa_json5.rust_project_name, );

        // endregion
        println!(
            r#"
    {YELLOW}On second run, the command `crate auto new_pwa_wasm` generated the project directory `{package_name}`{RESET}
    {YELLOW}You can open this new Rust project `{package_name}` in VSCode:{RESET}
{GREEN}code {package_name}{RESET}
    {YELLOW}Then build with:{RESET}
{GREEN}cargo auto build{RESET}
    {YELLOW}and follow the detailed instructions.{RESET}
"#,
            package_name = &pwa_json5.rust_project_name
        );
    } else {
        // On first run
        // They don't exist, create the default ones and return instructions to the user.
        if !std::path::Path::new("pwa.json5").exists() {
            std::fs::write(
                "pwa.json5",
                r#"
{
    // modify the values in this json5 file accordingly to your new project

    rust_project_name: "hello_world",
    github_owner_or_organization: "automation-tasks-rs",
    pwa_description: "Template for a minimal pwa wasm project for browser",
    web_server_domain: "bestia.dev",
    server_username: "luciano_bestia",
}
"#,
            )
            .unwrap();
        }
        if !std::path::Path::new("icon512x512.png").exists() {
            // TODO: download icon 512 from github
            // https://raw.githubusercontent.com/automation-tasks-rs/cargo_auto_template_new_pwa_wasm/main/web_server_folder/cargo_auto_template_new_pwa_wasm/icons/icon-512.png
            // decode the icon-512
            for file_item in get_vec_file() {
                if file_item.file_name.ends_with("icon-512.png") {
                    let file_content = file_item.file_content;
                    let file_content = <base64ct::Base64 as base64ct::Encoding>::decode_vec(&file_content).unwrap();
                    std::fs::write("icon512x512.png", file_content).unwrap();
                    break;
                }
            }
        }
        println!(
            r#"
    {YELLOW}On first run, the command `crate auto new_pwa_wasm` generated the files `pwa.json5` and `icon512x512.png`.{RESET}
    {YELLOW}Modify these files accordingly. This step is very important:{RESET}
{GREEN}code pwa.json5{RESET}
    {YELLOW}Finally, repeat the same command to generate the project directory.{RESET}
{GREEN}crate auto new_pwa_wasm {RESET}
"#
        );
    }
}

// favicon.ico with 16 and 32 icons
pub fn encode_to_favicon_ico(img: &image::DynamicImage, rust_project_name: &str) {
    // Create a new, empty icon collection:
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    favicon_add_entry(img, 16, &mut icon_dir);
    favicon_add_entry(img, 32, &mut icon_dir);
    favicon_add_entry(img, 48, &mut icon_dir);
    let file_name = format!("{rust_project_name}/web_server_folder/{rust_project_name}/favicon.ico");
    let buffer = std::fs::File::create(file_name).unwrap();
    icon_dir.write(buffer).unwrap();
}

pub fn favicon_add_entry(img: &image::DynamicImage, size: u32, icon_dir: &mut ico::IconDir) {
    // icons need smaller images 48, 32 and 16
    let img_rgba_vec = img.resize(size, size, image::imageops::FilterType::Lanczos3).into_rgba8().into_raw();
    // create an IconImage from raw RGBA pixel data from another image library
    let icon_image = ico::IconImage::from_rgba_data(size, size, img_rgba_vec);
    icon_dir.add_entry(ico::IconDirEntry::encode(&icon_image).unwrap());
}

/// decode png
pub fn decode_png(vec: Vec<u8>) -> image::DynamicImage {
    let img = image::io::Reader::new(std::io::Cursor::new(vec));
    let img = img.with_guessed_format().unwrap();
    let img = img.decode().unwrap();
    // return
    img
}

/// encode to png
pub fn encode_to_png(new_img: image::DynamicImage) -> Vec<u8> {
    //dbg!("encode new_img");
    let vec_u8: Vec<u8> = Vec::new();
    let mut cursor_1 = std::io::Cursor::new(vec_u8);
    let _x = new_img.write_to(&mut cursor_1, image::ImageOutputFormat::Png).unwrap();
    // return
    cursor_1.get_ref().to_owned()
}

/// resize img
pub fn resize_image(img: &image::DynamicImage, img_size: u32, file_name: &str, rust_project_name: &str) {
    //dbg!("resize_image {img_size}");
    let new_img = img.resize(img_size, img_size, image::imageops::FilterType::Lanczos3);
    let vec_u8 = encode_to_png(new_img);

    let file_name = format!("{rust_project_name}/web_server_folder/{rust_project_name}/icons/{file_name}");
    std::fs::write(file_name, vec_u8).unwrap();
}

pub fn copy_to_files(pwa_json5: &PwaJson5) {
    let project_name = &pwa_json5.rust_project_name;


    let folder_path = std::path::Path::new(project_name);
    if folder_path.exists() {
        panic!("{RED}Error: Folder {project_name} already exists! {RESET}");
    }
    std::fs::create_dir_all(folder_path).unwrap();

    // download latest template.tar.gz
    println!("{YELLOW}Downloading template.tar.gz...{RESET}");
    let file_name = "template.tar.gz";
    let path = "./template.tar.gz";
    let url = "https://github.com/automation-tasks-rs/cargo_auto_template_new_pwa_wasm/releases/download/v0.0.5/template.tar.gz";
    let reqwest_client = reqwest::blocking::Client::new();
    let http_response = reqwest_client.get(url).send();
    if http_response.is_err() {
        panic!("Error while retrieving data: {:#?}", http_response.err());
    } else {
        let body = http_response.unwrap().bytes().unwrap();
        // Get the content of the response
        std::fs::write(path, &body).expect(&format!("Download failed for {file_name}"));
    }

    // decompress into folder_path
    let tar_gz = std::fs::File::open(path).unwrap();
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(folder_path).unwrap();
    std::fs::remove_file(path).unwrap();

    // replace placeholders inside text files
    for entry in walkdir::WalkDir::new(folder_path).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            if entry.file_name().to_string_lossy().ends_with(".ico") || entry.file_name().to_string_lossy().ends_with(".png") || entry.file_name().to_string_lossy().ends_with(".woff2") {
                // cannot replace text in binary files 
            }else{
                // template has only valid utf8 files
                println!("replace: {}", entry.path().to_string_lossy());
                let content = std::fs::read_to_string(entry.path()).unwrap();
                let content = content.replace("cargo_auto_template_new_pwa_wasm", project_name);
                let content = content.replace("automation-tasks-rs", &pwa_json5.github_owner_or_organization);
                let content = content.replace("automation--tasks--rs", "automation-tasks-rs");
                let content = content.replace("pwa_description", &pwa_json5.pwa_description);
                let content = content.replace("web_server_domain",&pwa_json5.web_server_domain);
                let content = content.replace("server_username", &pwa_json5.server_username);
                std::fs::write(entry.path(), content).unwrap();
            }
        }
    }
    // renaming files is tricky and must be traverse  in reverse.
    let mut traverse_reverse: Vec<walkdir::DirEntry> = walkdir::WalkDir::new(folder_path).into_iter().filter_map(Result::ok).collect();
    traverse_reverse.reverse();
    for entry in traverse_reverse.iter() {
        if entry.file_name() == "cargo_auto_template_new_pwa_wasm" {
            println!("rename: {}", entry.path().to_string_lossy());
            std::fs::rename(entry.path(), entry.path().parent().unwrap().join(project_name)).unwrap();
        }
    }
}
