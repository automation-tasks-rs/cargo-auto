//! this strings are copied from the template_new_pwa_wasm folder
//! because when publishing to crates.io, only the main bin-executable is transferred

use crate::{GREEN, RED, RESET, YELLOW};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PwaJson5 {
    rust_project_name: String,
    pwa_short_name: String,
    pwa_name: String,
    pwa_description: String,
    project_author: String,
    project_homepage: String,
    project_repository: String,
}

pub fn new_pwa_wasm(arg_2: Option<String>) {
    match arg_2 {
        None => println!("{RED}Error: Project name argument is missing: `cargo auto new_pwa_wasm project_name`{RESET}"),
        Some(rust_project_name) => {
            if std::path::Path::new("pwa.json5").exists() && std::path::Path::new("icon512x512.png").exists() {
                // both exist, read the json5 and run the generator
                let pwa_json5: PwaJson5 = json5::from_str(&std::fs::read_to_string("pwa.json5").unwrap()).unwrap();
                // the rust_project_name must be equal
                if rust_project_name != pwa_json5.rust_project_name {
                    panic!("Error: rust_project_name in pwa.json5 is different!")
                }
                copy_to_files(&pwa_json5);

                // region: png with various sizes for: favicon png, pwa Android and pwa iOS
                // 32, 72, 96, 120, 128, 144, 152, 167, 180, 192, 196, 512
                let img = std::fs::read("icon512x512.png").unwrap();
                let img = decode_png(img);

                resize_image(&img, 32, "icon-032.png", &pwa_json5.rust_project_name, &pwa_json5.pwa_short_name);
                resize_image(&img, 72, "icon-072.png", &pwa_json5.rust_project_name, &pwa_json5.pwa_short_name);
                resize_image(&img, 96, "icon-096.png", &pwa_json5.rust_project_name, &pwa_json5.pwa_short_name);
                resize_image(&img, 120, "icon-120.png", &pwa_json5.rust_project_name, &pwa_json5.pwa_short_name);
                resize_image(&img, 128, "icon-128.png", &pwa_json5.rust_project_name, &pwa_json5.pwa_short_name);
                resize_image(&img, 144, "icon-144.png", &pwa_json5.rust_project_name, &pwa_json5.pwa_short_name);
                resize_image(&img, 152, "icon-152.png", &pwa_json5.rust_project_name, &pwa_json5.pwa_short_name);
                resize_image(&img, 167, "icon-167.png", &pwa_json5.rust_project_name, &pwa_json5.pwa_short_name);
                resize_image(&img, 180, "icon-180.png", &pwa_json5.rust_project_name, &pwa_json5.pwa_short_name);
                resize_image(&img, 192, "icon-192.png", &pwa_json5.rust_project_name, &pwa_json5.pwa_short_name);
                resize_image(&img, 196, "icon-196.png", &pwa_json5.rust_project_name, &pwa_json5.pwa_short_name);
                // overwrite the default with the new
                resize_image(&img, 512, "icon-512.png", &pwa_json5.rust_project_name, &pwa_json5.pwa_short_name);

                // maskable icon 192
                resize_image(&img, 192, "icon-maskable.png", &pwa_json5.rust_project_name, &pwa_json5.pwa_short_name);

                // favicon.ico with 16, 32 and 48 icons
                encode_to_favicon_ico(&img, &pwa_json5.rust_project_name, &pwa_json5.pwa_short_name);

                // endregion

                println!("");
                println!("    {YELLOW}On second call, the command `crate auto new_pwa_wasm` generated the directory `{rust_project_name}`{RESET}");
                println!("    {YELLOW}You can open this new Rust project `{rust_project_name}` in VSCode.{RESET}",);
                println!("{GREEN}code {rust_project_name}{RESET}");
                println!("    {YELLOW}Then build with:{RESET}");
                println!("{GREEN}cargo auto build{RESET}");
                println!("    {YELLOW}and follow the detailed instructions.{RESET}");
            } else {
                // They don't exist, create the default ones and return instructions to the user.
                if !std::path::Path::new("pwa.json5").exists() {
                    std::fs::write(
                        "pwa.json5",
                        r#"
{
    // modify the values in this json5 file accordingly to your new project

    rust_project_name: "rust_project_name",
    pwa_short_name: "pwa_short_name",
    pwa_name: "pwa_name",
    pwa_description: "pwa_description",
    project_author: "project_author",
    project_homepage: "project_homepage",
    project_repository: "project_repository",
}
"#,
                    )
                    .unwrap();
                }
                if !std::path::Path::new("icon512x512.png").exists() {
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
                println!("");
                println!("    {YELLOW}On first call, the command `crate auto new_pwa_wasm` generated the files `pwa.json5` and `icon512x512.png`.{RESET}");
                println!("    {YELLOW}Modify these files accordingly and repeat the same command to generate the project.{RESET}");
            }
        }
    }
}

// favicon.ico with 16 and 32 icons
pub fn encode_to_favicon_ico(img: &image::DynamicImage, rust_project_name: &str, pwa_short_name: &str) {
    // Create a new, empty icon collection:
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    favicon_add_entry(img, 16, &mut icon_dir);
    favicon_add_entry(img, 32, &mut icon_dir);
    favicon_add_entry(img, 48, &mut icon_dir);
    let file_name = format!("{rust_project_name}/web_server_folder/{pwa_short_name}/favicon.ico");
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
pub fn resize_image(img: &image::DynamicImage, img_size: u32, file_name: &str, rust_project_name: &str, pwa_short_name: &str) {
    //dbg!("resize_image {img_size}");
    let new_img = img.resize(img_size, img_size, image::imageops::FilterType::Lanczos3);
    let vec_u8 = encode_to_png(new_img);

    let file_name = format!("{rust_project_name}/web_server_folder/{pwa_short_name}/icons/{file_name}");
    std::fs::write(file_name, vec_u8).unwrap();
}

fn copy_to_files(pwa_json5: &PwaJson5) {
    let folder_path = std::path::Path::new(&pwa_json5.rust_project_name);
    std::fs::create_dir_all(folder_path).unwrap();
    for file_item in get_vec_file() {
        // rename/replace the project_name
        let file_name = file_item
            .file_name
            .replace("rust_project_name", &pwa_json5.rust_project_name)
            .replace("pwa_short_name", &pwa_json5.pwa_short_name);
        // create directory if needed
        std::fs::create_dir_all(folder_path.join(&file_name).parent().unwrap()).unwrap();

        if file_name.ends_with(".ico") || file_name.ends_with(".png") || file_name.ends_with(".woff2") {
            // binary files bas64
            let file_content = file_item.file_content;
            let file_content = <base64ct::Base64 as base64ct::Encoding>::decode_vec(&file_content).unwrap();
            std::fs::write(folder_path.join(&file_name), file_content).unwrap();
        } else {
            // text files
            let file_content = file_item
                .file_content
                .replace("rust_project_name", &pwa_json5.rust_project_name)
                .replace("pwa_short_name", &pwa_json5.pwa_short_name)
                .replace("pwa_name", &pwa_json5.pwa_name)
                .replace("pwa_description", &pwa_json5.pwa_description)
                .replace("project_author", &pwa_json5.project_author)
                .replace("project_homepage", &pwa_json5.project_homepage)
                .replace("project_repository", &pwa_json5.project_repository);
            std::fs::write(folder_path.join(&file_name), file_content.as_bytes()).unwrap();
        }
    }
}

pub fn get_vec_file() -> Vec<crate::FileItem> {
    let mut vec_file = vec![];

    // region: files copied into strings by automation tasks
    vec_file.push(crate::FileItem {
        file_name: "RELEASES.md",
        file_content: r###"# Releases changelog of rust_project_name

All notable changes to this project will be documented in this file.  
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).  
The library releases will be published on crates.io.  
The cargo-auto automation task will use the content of the section `## Unreleased` to create
the GitHub release consistently with this file.  
The ongoing changes that are not released, are visible in the git commits and github pull requests.  
The TODO section is part of the [README.md](https://github.com/bestia-dev/rust_project_name).  

## Unreleased

## Version 0.0.1

- Rust project created with `cargo auto new_pwa_wasm`
"###,
    });
    vec_file.push(crate::FileItem{
            file_name :"README.md",
            file_content : r###"# rust_project_name

[//]: # (auto_cargo_toml_to_md start)

**pwa_description**  
***version: 0.0.1 date: 2024-02-20 author: [project_author](project_homepage) repository: [GitHub](project_repository)***  

[//]: # (auto_cargo_toml_to_md end)


 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](project_repository/blob/master/LICENSE)
  [![GitHubAction](https://github.com/bestia-dev/# rust_project_name
/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/# rust_project_name
/)

[//]: # (auto_lines_of_code start)

[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-361-green.svg)](https://github.com/bestia-dev/rust_project_name/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-70-blue.svg)](https://github.com/bestia-dev/rust_project_name/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-68-purple.svg)](https://github.com/bestia-dev/rust_project_name/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/rust_project_name/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-16-orange.svg)](https://github.com/bestia-dev/rust_project_name/)

[//]: # (auto_lines_of_code end)

Hashtags: #rustlang #tutorial #pwa #wasm #webassembly  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## This template

Just like `cargo new` makes a soft and gentle introduction to Rust projects and development, I want to make the same for an in-browser WASM project with 

```bash
cargo auto new_pwa_wasm
```

Extremely simple, just the basic moving parts and use-cases.  
This simplest template does not have a PWA implementation or dedicated web server app.

## Development details

Read the development details in a separate md file:
[DEVELOPMENT.md](DEVELOPMENT.md)

## Releases changelog

Read the releases changelog in a separate md file:
[RELEASES.md](RELEASES.md)

## TODO

And code happily ever after...

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
"###,
});
    vec_file.push(crate::FileItem {
        file_name: "web_server_folder/pwa_short_name/service_worker.js",
        file_content: r###"
'use strict';

// Incrementing VERSION in CACHE_NAME will kick off the 
// install event and force previously cached
// resources to be cached again.
// but the new service worker will not be activated until all 
// tabs with this webapp are closed.

const CACHE_NAME = '2024.225.1914';

self.addEventListener('install', event => {
    console.log('event install ', CACHE_NAME);
    // the ugly trick of avoiding the waiting phase
    self.skipWaiting();

    event.waitUntil(
        caches.open(CACHE_NAME).then(function (cache) {
            return cache.addAll(
                [
                    '/pwa_short_name/',
                    'index.html',
                    'favicon.ico',
                    'manifest.json',
                    'start_service_worker.js',
                    'css/basic_style.css',
                    'css/fa-solid-900.woff2',
                    'css/fontawesome.css',
                    'css/normalize.css',
                    'css/Roboto-Medium.woff2',
                    'icons/icon-032.png',
                    'icons/icon-072.png',
                    'icons/icon-096.png',
                    'icons/icon-120.png',
                    'icons/icon-128.png',
                    'icons/icon-144.png',
                    'icons/icon-152.png',
                    'icons/icon-167.png',
                    'icons/icon-180.png',
                    'icons/icon-192.png',
                    'icons/icon-196.png',
                    'icons/icon-512.png',
                    'icons/icon-maskable.png',
                    'pkg/rust_project_name_bg.wasm',
                    'pkg/rust_project_name.js'
                ]
            );
        })
    );
});

self.addEventListener('activate', event => {
    console.log('event activate');
    // Delete all caches that aren't CACHE_NAME.
    event.waitUntil(
        caches.keys().then(cacheNames => {
            return Promise.all(
                cacheNames.map(cacheName => {
                    if (CACHE_NAME.indexOf(cacheName) === -1) {
                        // If this cache name isn't right, then delete it.
                        console.log('Deleting out of date cache:', cacheName);
                        return caches.delete(cacheName);
                    }
                })
            );
        })
    );
});

self.addEventListener('fetch', event => {
    // console.log('event fetch');
    // Let the browser do its default thing
    // for non-GET requests.
    if (event.request.method != 'GET') return;

    // Prevent the default, and handle the request ourselves.
    event.respondWith(async function () {
        // Try to get the response from a cache.
        const cache = await caches.open(CACHE_NAME);
        const cachedResponse = await cache.match(event.request);

        if (cachedResponse) {
            // console.log('from cache');
            // If we found a match in the cache, return it, but also
            // update the entry in the cache in the background.
            event.waitUntil(cache.add(event.request));
            return cachedResponse;
        }

        // If we didn't find a match in the cache, use the network and cache it for later.
        const response = await fetch(event.request);
        cache.put(event.request, response.clone());
        return response;
    }());
});
"###,
    });
    vec_file.push(crate::FileItem{
            file_name :"web_server_folder/pwa_short_name/icons/icon-512.png",
            file_content : r###"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"###,
});
    vec_file.push(crate::FileItem {
        file_name: "web_server_folder/pwa_short_name/index.html",
        file_content: r###"<!DOCTYPE html>
<html lang="en">
<head>
    <!-- classic header for a web page -->
    <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
    <title>pwa_name</title>
    <meta name="Description" content="pwa_description">
    <meta name="author" content="project_author">    
    <meta name="viewport" content="width = device-width,initial-scale = 1.0" />
    <link rel="stylesheet" href="css/normalize.css" />
    <link rel="stylesheet" href="css/basic_style.css" />
    <link rel="stylesheet" href="css/fontawesome.css" />    

    <!-- favicons generic-->
    <link rel="icon" type="image/png" href="icons/icon-032.png" sizes="32x32">
    <link rel="icon" type="image/png" href="icons/icon-128.png" sizes="128x128">
    <link rel="icon" type="image/png" href="icons/icon-192.png" sizes="192x192">
    <!-- favicons Android -->
    <link rel="shortcut icon" href="icons/icon-196.png" sizes="196x196">
    <!-- favicons iOS -->
    <link rel="apple-touch-icon" href="icons/icon-152.png" sizes="152x152">
    <link rel="apple-touch-icon" href="icons/icon-167.png" sizes="167x167">
    <link rel="apple-touch-icon" href="icons/icon-180.png" sizes="180x180">

    <!-- Metadata for PWA -->
    <link rel="manifest" href="manifest.json">
    <meta name="mobile-web-app-capable" content="yes">
    <meta name="apple-mobile-web-app-capable" content="yes" />
    <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent" />
    <meta name="theme-color" content="#000000">
    <link rel="apple-touch-icon" sizes="120x120" href="icons/icon-120.png">
    <link rel="apple-touch-icon" sizes="180x180" href="icons/icon-180.png">
</head>
<body>
    <!-- a standard service worker is a must for PWA -->
    <script src="start_service_worker.js"></script>
    <!-- warning if javascript is not enabled -->
    <noscript>
        <h2>
            !!!???!!!<br>
            This web app <br>
            cannot work <br>
            without javascript<br>
            enabled<br>
            !!!???!!!
        </h2>
    </noscript>

    <!-- display a text while waiting for wasm download. 
    This content will change from the wasm code.-->
    <div id="div_for_wasm_html_injecting">
        <h2>
            Waiting to<br>
            download <br>
            the web app...<br>
            This is <br>
            very quick on fast<br>
            networks...<br>
        </h2>
    </div>
    <div class="fc_red" id="div_for_errors"></div>
    <!-- import and init the wasm code -->
    <script type="module">
        import init from "./pkg/rust_project_name.js";
        async function run() {
            await init();
        }
        run();
    </script>
</body>
</html>"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "web_server_folder/pwa_short_name/manifest.json",
        file_content: r###"{
"short_name": "pwa_short_name",
"name": "pwa_name",
"icons": [
    {
        "src": "icons/icon-072.png",
        "sizes": "72x72",
        "type": "image/png",
        "density": "1.5"
    },
    {
        "src": "icons/icon-096.png",
        "sizes": "96x96",
        "type": "image/png",
        "density": "2.0"
    },
    {
        "src": "icons/icon-128.png",
        "sizes": "128x128",
        "type": "image/png",
        "density": "2.5"
    },
    {
        "src": "icons/icon-144.png",
        "sizes": "144x144",
        "type": "image/png",
        "density": "3.0"
    },
    {
        "src": "icons/icon-152.png",
        "sizes": "152x152",
        "type": "image/png",
        "density": "3.2"
    },
    {
        "src": "icons/icon-192.png",
        "sizes": "192x192",
        "type": "image/png",
        "density": "4.0"
    },
    {
        "src": "icons/icon-512.png",
        "sizes": "512x512",
        "type": "image/png"            
    },
    {
        "src": "icons/icon-maskable.png",
        "sizes": "192x192",
        "type": "image/png",
        "density": "4.0",
        "purpose": "any maskable"
    }
],
"start_url": "/pwa_short_name/index.html",
"background_color": "#000000",
"display": "standalone",
"orientation": "portrait",
"theme_color": "#000000"
}"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "web_server_folder/pwa_short_name/css/basic_style.css",
        file_content: r###"/* region: css variables */
:root {
    /* color palette */
    /* use of variables: var(--color_tooltip_1); */
    /* users can simply change the colors with the chrome extension: user CSS
    https://chrome.google.com/webstore/detail/user-css/okpjlejfhacmgjkmknjhadmkdbcldfcb
    */
    /* background color */
    --b_color_body: #24292E;
    --b_color_button: rgb(78, 78, 78);
    --b_color_header:rgb(78, 78, 78);
    /* front color */
    --f_color_body: #c4cce0;
    --f_color_button: white;
    --f_color_header:white;
    --f_color_link: white;
}

/* endregion: css variables */

/* region: media dependent on screen size */
/* less then 590px*/

@media (max-width: 590px) {
   

}

/* larger then 590px */

@media (min-width: 590px) {
   

}

@font-face {
    font-family: "Roboto";
    /* fonts are inside the css folder */
    src: url("Roboto-Medium.woff2") format("woff2");
}

@font-face {
    font-family: 'Font Awesome 5 Free';
    font-style: normal;
    font-weight: 900;
    font-display: block;
    src: url("fa-solid-900.woff2") format("woff2");
}

.fa,
.fas, .fa-solid {
    font-family: 'Font Awesome 5 Free';
    font-weight: 900;
}

/* endregion: media dependent on screen size */

/* region: basics */

html {
    font-family: 'Roboto', sans-serif;
    max-width: 1200px;
    min-width: 300px;
    width: 100%;
    /*margin auto means centered horizontally*/
    margin: auto;
    padding-right: 0px;
    overflow-y: auto;
    overflow-x: hidden;
    word-wrap: break-word;
    overflow-wrap: break-word;
    box-sizing: border-box;
    background-color: var(--b_color_body);
    line-height: 1.5;
    color: var(--f_color_body);
    /*This is the base font-size. All other font-size 
use rem units that are
relative to this font-size.*/
    /*width greater than 600 px*/
    font-size: 34px;
    -webkit-font-smoothing: antialiased;
    text-shadow: 1px 1px 1px rgba(0, 0, 0, 0.004);
}

body {
    margin: 0;
    padding: 2%;
    font-size: 60%;
    line-height: 1.5;
    background-color: var(--b_color_body);
    color: var(--f_color_body);
}

a:link,
a:visited,
a:hover,
a:active {
    color: inherit;
    text-decoration: none;
}

h1{
    padding:0px;
}
/* endregion: basics */

/* region: css classes */

.div_header{
    background-color: rgb(78, 78, 78);
    width: 100%;
}

.bold {
    font-weight: bold;
}

.fc_red{
    color: red;
}

.center {
    display: block;
    margin-left: auto;
    margin-right: auto;
}

.right {
    text-align: right;
}

.left {
    text-align: left;
}

.big {
    font-size: 140%;
}

.small {
    font-size: 80%;
}

.button {
    display: inline-block;
    padding: 12px 18px;
    cursor: pointer;
    border-radius: 6px;
    background: var(--b_color_button);
    font-size: 120%;
    font-weight: bold;
    color: var(--f_color_button);
}

/* endregion: css classes */

/* region: modal window */

.w3_modal {
    /* grey opacity over the whole display */
    z-index: 3;
    display: block;
    position: fixed;
    left: 0;
    top: 0;
    width: 100vw;
    height: 100vh;
    overflow: auto;
    /* opacity:80% would be inherited by the child. defined inside a rgba is not inherited. Trick! */
    background-color: rgba(0, 0, 0, 0.8);
    color: var(--f_color_body);
}

.w3_modal_content {
    top: 20%;
    width: 50%;
    margin: auto;
    background-color: var(--b_color_body);
    color: var(--f_color_body);
    border: 2px solid #ffffff;
    position: relative;
    padding: 5%;
    outline: 0;
}

/* endregion: modal window */"###,
    });
    vec_file.push(crate::FileItem{
            file_name :"web_server_folder/pwa_short_name/css/Roboto-Medium.woff2",
            file_content : r###"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"###,
});
    vec_file.push(crate::FileItem {
        file_name: "web_server_folder/pwa_short_name/css/fontawesome.css",
        file_content: r###"/*!
 * Font Awesome Free 5.15.3 by @fontawesome
 * https://fontawesome.com/v5.15/icons?d=gallery&p=2&s=solid&m=free
 * License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License)
 */

.fa,
.fas,
.far,
.fal,
.fad,
.fab {
    -moz-osx-font-smoothing: grayscale;
    -webkit-font-smoothing: antialiased;
    display: inline-block;
    font-style: normal;
    font-variant: normal;
    text-rendering: auto;
    line-height: 1;
}

.fa-lg {
    font-size: 1.33333em;
    line-height: 0.75em;
    vertical-align: -.0667em;
}

.fa-xs {
    font-size: .75em;
}

.fa-sm {
    font-size: .875em;
}

.fa-1x {
    font-size: 1em;
}

.fa-2x {
    font-size: 2em;
}

.fa-3x {
    font-size: 3em;
}

.fa-4x {
    font-size: 4em;
}

.fa-5x {
    font-size: 5em;
}

.fa-6x {
    font-size: 6em;
}

.fa-7x {
    font-size: 7em;
}

.fa-8x {
    font-size: 8em;
}

.fa-9x {
    font-size: 9em;
}

.fa-10x {
    font-size: 10em;
}

.fa-fw {
    text-align: center;
    width: 1.25em;
}

.fa-ul {
    list-style-type: none;
    margin-left: 2.5em;
    padding-left: 0;
}

.fa-ul>li {
    position: relative;
}

.fa-li {
    left: -2em;
    position: absolute;
    text-align: center;
    width: 2em;
    line-height: inherit;
}

.fa-border {
    border: solid 0.08em #eee;
    border-radius: .1em;
    padding: .2em .25em .15em;
}

.fa-pull-left {
    float: left;
}

.fa-pull-right {
    float: right;
}

.fa.fa-pull-left,
.fas.fa-pull-left,
.far.fa-pull-left,
.fal.fa-pull-left,
.fab.fa-pull-left {
    margin-right: .3em;
}

.fa.fa-pull-right,
.fas.fa-pull-right,
.far.fa-pull-right,
.fal.fa-pull-right,
.fab.fa-pull-right {
    margin-left: .3em;
}

.fa-spin {
    -webkit-animation: fa-spin 2s infinite linear;
    animation: fa-spin 2s infinite linear;
}

.fa-pulse {
    -webkit-animation: fa-spin 1s infinite steps(8);
    animation: fa-spin 1s infinite steps(8);
}

@-webkit-keyframes fa-spin {
    0% {
        -webkit-transform: rotate(0deg);
        transform: rotate(0deg);
    }
    100% {
        -webkit-transform: rotate(360deg);
        transform: rotate(360deg);
    }
}

@keyframes fa-spin {
    0% {
        -webkit-transform: rotate(0deg);
        transform: rotate(0deg);
    }
    100% {
        -webkit-transform: rotate(360deg);
        transform: rotate(360deg);
    }
}

.fa-rotate-90 {
    -ms-filter: "progid:DXImageTransform.Microsoft.BasicImage(rotation=1)";
    -webkit-transform: rotate(90deg);
    transform: rotate(90deg);
}

.fa-rotate-180 {
    -ms-filter: "progid:DXImageTransform.Microsoft.BasicImage(rotation=2)";
    -webkit-transform: rotate(180deg);
    transform: rotate(180deg);
}

.fa-rotate-270 {
    -ms-filter: "progid:DXImageTransform.Microsoft.BasicImage(rotation=3)";
    -webkit-transform: rotate(270deg);
    transform: rotate(270deg);
}

.fa-flip-horizontal {
    -ms-filter: "progid:DXImageTransform.Microsoft.BasicImage(rotation=0, mirror=1)";
    -webkit-transform: scale(-1, 1);
    transform: scale(-1, 1);
}

.fa-flip-vertical {
    -ms-filter: "progid:DXImageTransform.Microsoft.BasicImage(rotation=2, mirror=1)";
    -webkit-transform: scale(1, -1);
    transform: scale(1, -1);
}

.fa-flip-both,
.fa-flip-horizontal.fa-flip-vertical {
    -ms-filter: "progid:DXImageTransform.Microsoft.BasicImage(rotation=2, mirror=1)";
    -webkit-transform: scale(-1, -1);
    transform: scale(-1, -1);
}

:root .fa-rotate-90,
:root .fa-rotate-180,
:root .fa-rotate-270,
:root .fa-flip-horizontal,
:root .fa-flip-vertical,
:root .fa-flip-both {
    -webkit-filter: none;
    filter: none;
}

.fa-stack {
    display: inline-block;
    height: 2em;
    line-height: 2em;
    position: relative;
    vertical-align: middle;
    width: 2.5em;
}

.fa-stack-1x,
.fa-stack-2x {
    left: 0;
    position: absolute;
    text-align: center;
    width: 100%;
}

.fa-stack-1x {
    line-height: inherit;
}

.fa-stack-2x {
    font-size: 2em;
}

.fa-inverse {
    color: #fff;
}


/* Font Awesome uses the Unicode Private Use Area (PUA) to ensure screen
readers do not read off random characters that represent icons */

.fa-500px:before {
    content: "\f26e";
}

.fa-accessible-icon:before {
    content: "\f368";
}

.fa-accusoft:before {
    content: "\f369";
}

.fa-acquisitions-incorporated:before {
    content: "\f6af";
}

.fa-ad:before {
    content: "\f641";
}

.fa-address-book:before {
    content: "\f2b9";
}

.fa-address-card:before {
    content: "\f2bb";
}

.fa-adjust:before {
    content: "\f042";
}

.fa-adn:before {
    content: "\f170";
}

.fa-adversal:before {
    content: "\f36a";
}

.fa-affiliatetheme:before {
    content: "\f36b";
}

.fa-air-freshener:before {
    content: "\f5d0";
}

.fa-airbnb:before {
    content: "\f834";
}

.fa-algolia:before {
    content: "\f36c";
}

.fa-align-center:before {
    content: "\f037";
}

.fa-align-justify:before {
    content: "\f039";
}

.fa-align-left:before {
    content: "\f036";
}

.fa-align-right:before {
    content: "\f038";
}

.fa-alipay:before {
    content: "\f642";
}

.fa-allergies:before {
    content: "\f461";
}

.fa-amazon:before {
    content: "\f270";
}

.fa-amazon-pay:before {
    content: "\f42c";
}

.fa-ambulance:before {
    content: "\f0f9";
}

.fa-american-sign-language-interpreting:before {
    content: "\f2a3";
}

.fa-amilia:before {
    content: "\f36d";
}

.fa-anchor:before {
    content: "\f13d";
}

.fa-android:before {
    content: "\f17b";
}

.fa-angellist:before {
    content: "\f209";
}

.fa-angle-double-down:before {
    content: "\f103";
}

.fa-angle-double-left:before {
    content: "\f100";
}

.fa-angle-double-right:before {
    content: "\f101";
}

.fa-angle-double-up:before {
    content: "\f102";
}

.fa-angle-down:before {
    content: "\f107";
}

.fa-angle-left:before {
    content: "\f104";
}

.fa-angle-right:before {
    content: "\f105";
}

.fa-angle-up:before {
    content: "\f106";
}

.fa-angry:before {
    content: "\f556";
}

.fa-angrycreative:before {
    content: "\f36e";
}

.fa-angular:before {
    content: "\f420";
}

.fa-ankh:before {
    content: "\f644";
}

.fa-app-store:before {
    content: "\f36f";
}

.fa-app-store-ios:before {
    content: "\f370";
}

.fa-apper:before {
    content: "\f371";
}

.fa-apple:before {
    content: "\f179";
}

.fa-apple-alt:before {
    content: "\f5d1";
}

.fa-apple-pay:before {
    content: "\f415";
}

.fa-archive:before {
    content: "\f187";
}

.fa-archway:before {
    content: "\f557";
}

.fa-arrow-alt-circle-down:before {
    content: "\f358";
}

.fa-arrow-alt-circle-left:before {
    content: "\f359";
}

.fa-arrow-alt-circle-right:before {
    content: "\f35a";
}

.fa-arrow-alt-circle-up:before {
    content: "\f35b";
}

.fa-arrow-circle-down:before {
    content: "\f0ab";
}

.fa-arrow-circle-left:before {
    content: "\f0a8";
}

.fa-arrow-circle-right:before {
    content: "\f0a9";
}

.fa-arrow-circle-up:before {
    content: "\f0aa";
}

.fa-arrow-down:before {
    content: "\f063";
}

.fa-arrow-left:before {
    content: "\f060";
}

.fa-arrow-right:before {
    content: "\f061";
}

.fa-arrow-up:before {
    content: "\f062";
}

.fa-arrows-alt:before {
    content: "\f0b2";
}

.fa-arrows-alt-h:before {
    content: "\f337";
}

.fa-arrows-alt-v:before {
    content: "\f338";
}

.fa-artstation:before {
    content: "\f77a";
}

.fa-assistive-listening-systems:before {
    content: "\f2a2";
}

.fa-asterisk:before {
    content: "\f069";
}

.fa-asymmetrik:before {
    content: "\f372";
}

.fa-at:before {
    content: "\f1fa";
}

.fa-atlas:before {
    content: "\f558";
}

.fa-atlassian:before {
    content: "\f77b";
}

.fa-atom:before {
    content: "\f5d2";
}

.fa-audible:before {
    content: "\f373";
}

.fa-audio-description:before {
    content: "\f29e";
}

.fa-autoprefixer:before {
    content: "\f41c";
}

.fa-avianex:before {
    content: "\f374";
}

.fa-aviato:before {
    content: "\f421";
}

.fa-award:before {
    content: "\f559";
}

.fa-aws:before {
    content: "\f375";
}

.fa-baby:before {
    content: "\f77c";
}

.fa-baby-carriage:before {
    content: "\f77d";
}

.fa-backspace:before {
    content: "\f55a";
}

.fa-backward:before {
    content: "\f04a";
}

.fa-bacon:before {
    content: "\f7e5";
}

.fa-bacteria:before {
    content: "\e059";
}

.fa-bacterium:before {
    content: "\e05a";
}

.fa-bahai:before {
    content: "\f666";
}

.fa-balance-scale:before {
    content: "\f24e";
}

.fa-balance-scale-left:before {
    content: "\f515";
}

.fa-balance-scale-right:before {
    content: "\f516";
}

.fa-ban:before {
    content: "\f05e";
}

.fa-band-aid:before {
    content: "\f462";
}

.fa-bandcamp:before {
    content: "\f2d5";
}

.fa-barcode:before {
    content: "\f02a";
}

.fa-bars:before {
    content: "\f0c9";
}

.fa-baseball-ball:before {
    content: "\f433";
}

.fa-basketball-ball:before {
    content: "\f434";
}

.fa-bath:before {
    content: "\f2cd";
}

.fa-battery-empty:before {
    content: "\f244";
}

.fa-battery-full:before {
    content: "\f240";
}

.fa-battery-half:before {
    content: "\f242";
}

.fa-battery-quarter:before {
    content: "\f243";
}

.fa-battery-three-quarters:before {
    content: "\f241";
}

.fa-battle-net:before {
    content: "\f835";
}

.fa-bed:before {
    content: "\f236";
}

.fa-beer:before {
    content: "\f0fc";
}

.fa-behance:before {
    content: "\f1b4";
}

.fa-behance-square:before {
    content: "\f1b5";
}

.fa-bell:before {
    content: "\f0f3";
}

.fa-bell-slash:before {
    content: "\f1f6";
}

.fa-bezier-curve:before {
    content: "\f55b";
}

.fa-bible:before {
    content: "\f647";
}

.fa-bicycle:before {
    content: "\f206";
}

.fa-biking:before {
    content: "\f84a";
}

.fa-bimobject:before {
    content: "\f378";
}

.fa-binoculars:before {
    content: "\f1e5";
}

.fa-biohazard:before {
    content: "\f780";
}

.fa-birthday-cake:before {
    content: "\f1fd";
}

.fa-bitbucket:before {
    content: "\f171";
}

.fa-bitcoin:before {
    content: "\f379";
}

.fa-bity:before {
    content: "\f37a";
}

.fa-black-tie:before {
    content: "\f27e";
}

.fa-blackberry:before {
    content: "\f37b";
}

.fa-blender:before {
    content: "\f517";
}

.fa-blender-phone:before {
    content: "\f6b6";
}

.fa-blind:before {
    content: "\f29d";
}

.fa-blog:before {
    content: "\f781";
}

.fa-blogger:before {
    content: "\f37c";
}

.fa-blogger-b:before {
    content: "\f37d";
}

.fa-bluetooth:before {
    content: "\f293";
}

.fa-bluetooth-b:before {
    content: "\f294";
}

.fa-bold:before {
    content: "\f032";
}

.fa-bolt:before {
    content: "\f0e7";
}

.fa-bomb:before {
    content: "\f1e2";
}

.fa-bone:before {
    content: "\f5d7";
}

.fa-bong:before {
    content: "\f55c";
}

.fa-book:before {
    content: "\f02d";
}

.fa-book-dead:before {
    content: "\f6b7";
}

.fa-book-medical:before {
    content: "\f7e6";
}

.fa-book-open:before {
    content: "\f518";
}

.fa-book-reader:before {
    content: "\f5da";
}

.fa-bookmark:before {
    content: "\f02e";
}

.fa-bootstrap:before {
    content: "\f836";
}

.fa-border-all:before {
    content: "\f84c";
}

.fa-border-none:before {
    content: "\f850";
}

.fa-border-style:before {
    content: "\f853";
}

.fa-bowling-ball:before {
    content: "\f436";
}

.fa-box:before {
    content: "\f466";
}

.fa-box-open:before {
    content: "\f49e";
}

.fa-box-tissue:before {
    content: "\e05b";
}

.fa-boxes:before {
    content: "\f468";
}

.fa-braille:before {
    content: "\f2a1";
}

.fa-brain:before {
    content: "\f5dc";
}

.fa-bread-slice:before {
    content: "\f7ec";
}

.fa-briefcase:before {
    content: "\f0b1";
}

.fa-briefcase-medical:before {
    content: "\f469";
}

.fa-broadcast-tower:before {
    content: "\f519";
}

.fa-broom:before {
    content: "\f51a";
}

.fa-brush:before {
    content: "\f55d";
}

.fa-btc:before {
    content: "\f15a";
}

.fa-buffer:before {
    content: "\f837";
}

.fa-bug:before {
    content: "\f188";
}

.fa-building:before {
    content: "\f1ad";
}

.fa-bullhorn:before {
    content: "\f0a1";
}

.fa-bullseye:before {
    content: "\f140";
}

.fa-burn:before {
    content: "\f46a";
}

.fa-buromobelexperte:before {
    content: "\f37f";
}

.fa-bus:before {
    content: "\f207";
}

.fa-bus-alt:before {
    content: "\f55e";
}

.fa-business-time:before {
    content: "\f64a";
}

.fa-buy-n-large:before {
    content: "\f8a6";
}

.fa-buysellads:before {
    content: "\f20d";
}

.fa-calculator:before {
    content: "\f1ec";
}

.fa-calendar:before {
    content: "\f133";
}

.fa-calendar-alt:before {
    content: "\f073";
}

.fa-calendar-check:before {
    content: "\f274";
}

.fa-calendar-day:before {
    content: "\f783";
}

.fa-calendar-minus:before {
    content: "\f272";
}

.fa-calendar-plus:before {
    content: "\f271";
}

.fa-calendar-times:before {
    content: "\f273";
}

.fa-calendar-week:before {
    content: "\f784";
}

.fa-camera:before {
    content: "\f030";
}

.fa-camera-retro:before {
    content: "\f083";
}

.fa-campground:before {
    content: "\f6bb";
}

.fa-canadian-maple-leaf:before {
    content: "\f785";
}

.fa-candy-cane:before {
    content: "\f786";
}

.fa-cannabis:before {
    content: "\f55f";
}

.fa-capsules:before {
    content: "\f46b";
}

.fa-car:before {
    content: "\f1b9";
}

.fa-car-alt:before {
    content: "\f5de";
}

.fa-car-battery:before {
    content: "\f5df";
}

.fa-car-crash:before {
    content: "\f5e1";
}

.fa-car-side:before {
    content: "\f5e4";
}

.fa-caravan:before {
    content: "\f8ff";
}

.fa-caret-down:before {
    content: "\f0d7";
}

.fa-caret-left:before {
    content: "\f0d9";
}

.fa-caret-right:before {
    content: "\f0da";
}

.fa-caret-square-down:before {
    content: "\f150";
}

.fa-caret-square-left:before {
    content: "\f191";
}

.fa-caret-square-right:before {
    content: "\f152";
}

.fa-caret-square-up:before {
    content: "\f151";
}

.fa-caret-up:before {
    content: "\f0d8";
}

.fa-carrot:before {
    content: "\f787";
}

.fa-cart-arrow-down:before {
    content: "\f218";
}

.fa-cart-plus:before {
    content: "\f217";
}

.fa-cash-register:before {
    content: "\f788";
}

.fa-cat:before {
    content: "\f6be";
}

.fa-cc-amazon-pay:before {
    content: "\f42d";
}

.fa-cc-amex:before {
    content: "\f1f3";
}

.fa-cc-apple-pay:before {
    content: "\f416";
}

.fa-cc-diners-club:before {
    content: "\f24c";
}

.fa-cc-discover:before {
    content: "\f1f2";
}

.fa-cc-jcb:before {
    content: "\f24b";
}

.fa-cc-mastercard:before {
    content: "\f1f1";
}

.fa-cc-paypal:before {
    content: "\f1f4";
}

.fa-cc-stripe:before {
    content: "\f1f5";
}

.fa-cc-visa:before {
    content: "\f1f0";
}

.fa-centercode:before {
    content: "\f380";
}

.fa-centos:before {
    content: "\f789";
}

.fa-certificate:before {
    content: "\f0a3";
}

.fa-chair:before {
    content: "\f6c0";
}

.fa-chalkboard:before {
    content: "\f51b";
}

.fa-chalkboard-teacher:before {
    content: "\f51c";
}

.fa-charging-station:before {
    content: "\f5e7";
}

.fa-chart-area:before {
    content: "\f1fe";
}

.fa-chart-bar:before {
    content: "\f080";
}

.fa-chart-line:before {
    content: "\f201";
}

.fa-chart-pie:before {
    content: "\f200";
}

.fa-check:before {
    content: "\f00c";
}

.fa-check-circle:before {
    content: "\f058";
}

.fa-check-double:before {
    content: "\f560";
}

.fa-check-square:before {
    content: "\f14a";
}

.fa-cheese:before {
    content: "\f7ef";
}

.fa-chess:before {
    content: "\f439";
}

.fa-chess-bishop:before {
    content: "\f43a";
}

.fa-chess-board:before {
    content: "\f43c";
}

.fa-chess-king:before {
    content: "\f43f";
}

.fa-chess-knight:before {
    content: "\f441";
}

.fa-chess-pawn:before {
    content: "\f443";
}

.fa-chess-queen:before {
    content: "\f445";
}

.fa-chess-rook:before {
    content: "\f447";
}

.fa-chevron-circle-down:before {
    content: "\f13a";
}

.fa-chevron-circle-left:before {
    content: "\f137";
}

.fa-chevron-circle-right:before {
    content: "\f138";
}

.fa-chevron-circle-up:before {
    content: "\f139";
}

.fa-chevron-down:before {
    content: "\f078";
}

.fa-chevron-left:before {
    content: "\f053";
}

.fa-chevron-right:before {
    content: "\f054";
}

.fa-chevron-up:before {
    content: "\f077";
}

.fa-child:before {
    content: "\f1ae";
}

.fa-chrome:before {
    content: "\f268";
}

.fa-chromecast:before {
    content: "\f838";
}

.fa-church:before {
    content: "\f51d";
}

.fa-circle:before {
    content: "\f111";
}

.fa-circle-notch:before {
    content: "\f1ce";
}

.fa-city:before {
    content: "\f64f";
}

.fa-clinic-medical:before {
    content: "\f7f2";
}

.fa-clipboard:before {
    content: "\f328";
}

.fa-clipboard-check:before {
    content: "\f46c";
}

.fa-clipboard-list:before {
    content: "\f46d";
}

.fa-clock:before {
    content: "\f017";
}

.fa-clone:before {
    content: "\f24d";
}

.fa-closed-captioning:before {
    content: "\f20a";
}

.fa-cloud:before {
    content: "\f0c2";
}

.fa-cloud-download-alt:before {
    content: "\f381";
}

.fa-cloud-meatball:before {
    content: "\f73b";
}

.fa-cloud-moon:before {
    content: "\f6c3";
}

.fa-cloud-moon-rain:before {
    content: "\f73c";
}

.fa-cloud-rain:before {
    content: "\f73d";
}

.fa-cloud-showers-heavy:before {
    content: "\f740";
}

.fa-cloud-sun:before {
    content: "\f6c4";
}

.fa-cloud-sun-rain:before {
    content: "\f743";
}

.fa-cloud-upload-alt:before {
    content: "\f382";
}

.fa-cloudflare:before {
    content: "\e07d";
}

.fa-cloudscale:before {
    content: "\f383";
}

.fa-cloudsmith:before {
    content: "\f384";
}

.fa-cloudversify:before {
    content: "\f385";
}

.fa-cocktail:before {
    content: "\f561";
}

.fa-code:before {
    content: "\f121";
}

.fa-code-branch:before {
    content: "\f126";
}

.fa-codepen:before {
    content: "\f1cb";
}

.fa-codiepie:before {
    content: "\f284";
}

.fa-coffee:before {
    content: "\f0f4";
}

.fa-cog:before {
    content: "\f013";
}

.fa-cogs:before {
    content: "\f085";
}

.fa-coins:before {
    content: "\f51e";
}

.fa-columns:before {
    content: "\f0db";
}

.fa-comment:before {
    content: "\f075";
}

.fa-comment-alt:before {
    content: "\f27a";
}

.fa-comment-dollar:before {
    content: "\f651";
}

.fa-comment-dots:before {
    content: "\f4ad";
}

.fa-comment-medical:before {
    content: "\f7f5";
}

.fa-comment-slash:before {
    content: "\f4b3";
}

.fa-comments:before {
    content: "\f086";
}

.fa-comments-dollar:before {
    content: "\f653";
}

.fa-compact-disc:before {
    content: "\f51f";
}

.fa-compass:before {
    content: "\f14e";
}

.fa-compress:before {
    content: "\f066";
}

.fa-compress-alt:before {
    content: "\f422";
}

.fa-compress-arrows-alt:before {
    content: "\f78c";
}

.fa-concierge-bell:before {
    content: "\f562";
}

.fa-confluence:before {
    content: "\f78d";
}

.fa-connectdevelop:before {
    content: "\f20e";
}

.fa-contao:before {
    content: "\f26d";
}

.fa-cookie:before {
    content: "\f563";
}

.fa-cookie-bite:before {
    content: "\f564";
}

.fa-copy:before {
    content: "\f0c5";
}

.fa-copyright:before {
    content: "\f1f9";
}

.fa-cotton-bureau:before {
    content: "\f89e";
}

.fa-couch:before {
    content: "\f4b8";
}

.fa-cpanel:before {
    content: "\f388";
}

.fa-creative-commons:before {
    content: "\f25e";
}

.fa-creative-commons-by:before {
    content: "\f4e7";
}

.fa-creative-commons-nc:before {
    content: "\f4e8";
}

.fa-creative-commons-nc-eu:before {
    content: "\f4e9";
}

.fa-creative-commons-nc-jp:before {
    content: "\f4ea";
}

.fa-creative-commons-nd:before {
    content: "\f4eb";
}

.fa-creative-commons-pd:before {
    content: "\f4ec";
}

.fa-creative-commons-pd-alt:before {
    content: "\f4ed";
}

.fa-creative-commons-remix:before {
    content: "\f4ee";
}

.fa-creative-commons-sa:before {
    content: "\f4ef";
}

.fa-creative-commons-sampling:before {
    content: "\f4f0";
}

.fa-creative-commons-sampling-plus:before {
    content: "\f4f1";
}

.fa-creative-commons-share:before {
    content: "\f4f2";
}

.fa-creative-commons-zero:before {
    content: "\f4f3";
}

.fa-credit-card:before {
    content: "\f09d";
}

.fa-critical-role:before {
    content: "\f6c9";
}

.fa-crop:before {
    content: "\f125";
}

.fa-crop-alt:before {
    content: "\f565";
}

.fa-cross:before {
    content: "\f654";
}

.fa-crosshairs:before {
    content: "\f05b";
}

.fa-crow:before {
    content: "\f520";
}

.fa-crown:before {
    content: "\f521";
}

.fa-crutch:before {
    content: "\f7f7";
}

.fa-css3:before {
    content: "\f13c";
}

.fa-css3-alt:before {
    content: "\f38b";
}

.fa-cube:before {
    content: "\f1b2";
}

.fa-cubes:before {
    content: "\f1b3";
}

.fa-cut:before {
    content: "\f0c4";
}

.fa-cuttlefish:before {
    content: "\f38c";
}

.fa-d-and-d:before {
    content: "\f38d";
}

.fa-d-and-d-beyond:before {
    content: "\f6ca";
}

.fa-dailymotion:before {
    content: "\e052";
}

.fa-dashcube:before {
    content: "\f210";
}

.fa-database:before {
    content: "\f1c0";
}

.fa-deaf:before {
    content: "\f2a4";
}

.fa-deezer:before {
    content: "\e077";
}

.fa-delicious:before {
    content: "\f1a5";
}

.fa-democrat:before {
    content: "\f747";
}

.fa-deploydog:before {
    content: "\f38e";
}

.fa-deskpro:before {
    content: "\f38f";
}

.fa-desktop:before {
    content: "\f108";
}

.fa-dev:before {
    content: "\f6cc";
}

.fa-deviantart:before {
    content: "\f1bd";
}

.fa-dharmachakra:before {
    content: "\f655";
}

.fa-dhl:before {
    content: "\f790";
}

.fa-diagnoses:before {
    content: "\f470";
}

.fa-diaspora:before {
    content: "\f791";
}

.fa-dice:before {
    content: "\f522";
}

.fa-dice-d20:before {
    content: "\f6cf";
}

.fa-dice-d6:before {
    content: "\f6d1";
}

.fa-dice-five:before {
    content: "\f523";
}

.fa-dice-four:before {
    content: "\f524";
}

.fa-dice-one:before {
    content: "\f525";
}

.fa-dice-six:before {
    content: "\f526";
}

.fa-dice-three:before {
    content: "\f527";
}

.fa-dice-two:before {
    content: "\f528";
}

.fa-digg:before {
    content: "\f1a6";
}

.fa-digital-ocean:before {
    content: "\f391";
}

.fa-digital-tachograph:before {
    content: "\f566";
}

.fa-directions:before {
    content: "\f5eb";
}

.fa-discord:before {
    content: "\f392";
}

.fa-discourse:before {
    content: "\f393";
}

.fa-disease:before {
    content: "\f7fa";
}

.fa-divide:before {
    content: "\f529";
}

.fa-dizzy:before {
    content: "\f567";
}

.fa-dna:before {
    content: "\f471";
}

.fa-dochub:before {
    content: "\f394";
}

.fa-docker:before {
    content: "\f395";
}

.fa-dog:before {
    content: "\f6d3";
}

.fa-dollar-sign:before {
    content: "\f155";
}

.fa-dolly:before {
    content: "\f472";
}

.fa-dolly-flatbed:before {
    content: "\f474";
}

.fa-donate:before {
    content: "\f4b9";
}

.fa-door-closed:before {
    content: "\f52a";
}

.fa-door-open:before {
    content: "\f52b";
}

.fa-dot-circle:before {
    content: "\f192";
}

.fa-dove:before {
    content: "\f4ba";
}

.fa-download:before {
    content: "\f019";
}

.fa-draft2digital:before {
    content: "\f396";
}

.fa-drafting-compass:before {
    content: "\f568";
}

.fa-dragon:before {
    content: "\f6d5";
}

.fa-draw-polygon:before {
    content: "\f5ee";
}

.fa-dribbble:before {
    content: "\f17d";
}

.fa-dribbble-square:before {
    content: "\f397";
}

.fa-dropbox:before {
    content: "\f16b";
}

.fa-drum:before {
    content: "\f569";
}

.fa-drum-steelpan:before {
    content: "\f56a";
}

.fa-drumstick-bite:before {
    content: "\f6d7";
}

.fa-drupal:before {
    content: "\f1a9";
}

.fa-dumbbell:before {
    content: "\f44b";
}

.fa-dumpster:before {
    content: "\f793";
}

.fa-dumpster-fire:before {
    content: "\f794";
}

.fa-dungeon:before {
    content: "\f6d9";
}

.fa-dyalog:before {
    content: "\f399";
}

.fa-earlybirds:before {
    content: "\f39a";
}

.fa-ebay:before {
    content: "\f4f4";
}

.fa-edge:before {
    content: "\f282";
}

.fa-edge-legacy:before {
    content: "\e078";
}

.fa-edit:before {
    content: "\f044";
}

.fa-egg:before {
    content: "\f7fb";
}

.fa-eject:before {
    content: "\f052";
}

.fa-elementor:before {
    content: "\f430";
}

.fa-ellipsis-h:before {
    content: "\f141";
}

.fa-ellipsis-v:before {
    content: "\f142";
}

.fa-ello:before {
    content: "\f5f1";
}

.fa-ember:before {
    content: "\f423";
}

.fa-empire:before {
    content: "\f1d1";
}

.fa-envelope:before {
    content: "\f0e0";
}

.fa-envelope-open:before {
    content: "\f2b6";
}

.fa-envelope-open-text:before {
    content: "\f658";
}

.fa-envelope-square:before {
    content: "\f199";
}

.fa-envira:before {
    content: "\f299";
}

.fa-equals:before {
    content: "\f52c";
}

.fa-eraser:before {
    content: "\f12d";
}

.fa-erlang:before {
    content: "\f39d";
}

.fa-ethereum:before {
    content: "\f42e";
}

.fa-ethernet:before {
    content: "\f796";
}

.fa-etsy:before {
    content: "\f2d7";
}

.fa-euro-sign:before {
    content: "\f153";
}

.fa-evernote:before {
    content: "\f839";
}

.fa-exchange-alt:before {
    content: "\f362";
}

.fa-exclamation:before {
    content: "\f12a";
}

.fa-exclamation-circle:before {
    content: "\f06a";
}

.fa-exclamation-triangle:before {
    content: "\f071";
}

.fa-expand:before {
    content: "\f065";
}

.fa-expand-alt:before {
    content: "\f424";
}

.fa-expand-arrows-alt:before {
    content: "\f31e";
}

.fa-expeditedssl:before {
    content: "\f23e";
}

.fa-external-link-alt:before {
    content: "\f35d";
}

.fa-external-link-square-alt:before {
    content: "\f360";
}

.fa-eye:before {
    content: "\f06e";
}

.fa-eye-dropper:before {
    content: "\f1fb";
}

.fa-eye-slash:before {
    content: "\f070";
}

.fa-facebook:before {
    content: "\f09a";
}

.fa-facebook-f:before {
    content: "\f39e";
}

.fa-facebook-messenger:before {
    content: "\f39f";
}

.fa-facebook-square:before {
    content: "\f082";
}

.fa-fan:before {
    content: "\f863";
}

.fa-fantasy-flight-games:before {
    content: "\f6dc";
}

.fa-fast-backward:before {
    content: "\f049";
}

.fa-fast-forward:before {
    content: "\f050";
}

.fa-faucet:before {
    content: "\e005";
}

.fa-fax:before {
    content: "\f1ac";
}

.fa-feather:before {
    content: "\f52d";
}

.fa-feather-alt:before {
    content: "\f56b";
}

.fa-fedex:before {
    content: "\f797";
}

.fa-fedora:before {
    content: "\f798";
}

.fa-female:before {
    content: "\f182";
}

.fa-fighter-jet:before {
    content: "\f0fb";
}

.fa-figma:before {
    content: "\f799";
}

.fa-file:before {
    content: "\f15b";
}

.fa-file-alt:before {
    content: "\f15c";
}

.fa-file-archive:before {
    content: "\f1c6";
}

.fa-file-audio:before {
    content: "\f1c7";
}

.fa-file-code:before {
    content: "\f1c9";
}

.fa-file-contract:before {
    content: "\f56c";
}

.fa-file-csv:before {
    content: "\f6dd";
}

.fa-file-download:before {
    content: "\f56d";
}

.fa-file-excel:before {
    content: "\f1c3";
}

.fa-file-export:before {
    content: "\f56e";
}

.fa-file-image:before {
    content: "\f1c5";
}

.fa-file-import:before {
    content: "\f56f";
}

.fa-file-invoice:before {
    content: "\f570";
}

.fa-file-invoice-dollar:before {
    content: "\f571";
}

.fa-file-medical:before {
    content: "\f477";
}

.fa-file-medical-alt:before {
    content: "\f478";
}

.fa-file-pdf:before {
    content: "\f1c1";
}

.fa-file-powerpoint:before {
    content: "\f1c4";
}

.fa-file-prescription:before {
    content: "\f572";
}

.fa-file-signature:before {
    content: "\f573";
}

.fa-file-upload:before {
    content: "\f574";
}

.fa-file-video:before {
    content: "\f1c8";
}

.fa-file-word:before {
    content: "\f1c2";
}

.fa-fill:before {
    content: "\f575";
}

.fa-fill-drip:before {
    content: "\f576";
}

.fa-film:before {
    content: "\f008";
}

.fa-filter:before {
    content: "\f0b0";
}

.fa-fingerprint:before {
    content: "\f577";
}

.fa-fire:before {
    content: "\f06d";
}

.fa-fire-alt:before {
    content: "\f7e4";
}

.fa-fire-extinguisher:before {
    content: "\f134";
}

.fa-firefox:before {
    content: "\f269";
}

.fa-firefox-browser:before {
    content: "\e007";
}

.fa-first-aid:before {
    content: "\f479";
}

.fa-first-order:before {
    content: "\f2b0";
}

.fa-first-order-alt:before {
    content: "\f50a";
}

.fa-firstdraft:before {
    content: "\f3a1";
}

.fa-fish:before {
    content: "\f578";
}

.fa-fist-raised:before {
    content: "\f6de";
}

.fa-flag:before {
    content: "\f024";
}

.fa-flag-checkered:before {
    content: "\f11e";
}

.fa-flag-usa:before {
    content: "\f74d";
}

.fa-flask:before {
    content: "\f0c3";
}

.fa-flickr:before {
    content: "\f16e";
}

.fa-flipboard:before {
    content: "\f44d";
}

.fa-flushed:before {
    content: "\f579";
}

.fa-fly:before {
    content: "\f417";
}

.fa-folder:before {
    content: "\f07b";
}

.fa-folder-minus:before {
    content: "\f65d";
}

.fa-folder-open:before {
    content: "\f07c";
}

.fa-folder-plus:before {
    content: "\f65e";
}

.fa-font:before {
    content: "\f031";
}

.fa-font-awesome:before {
    content: "\f2b4";
}

.fa-font-awesome-alt:before {
    content: "\f35c";
}

.fa-font-awesome-flag:before {
    content: "\f425";
}

.fa-font-awesome-logo-full:before {
    content: "\f4e6";
}

.fa-fonticons:before {
    content: "\f280";
}

.fa-fonticons-fi:before {
    content: "\f3a2";
}

.fa-football-ball:before {
    content: "\f44e";
}

.fa-fort-awesome:before {
    content: "\f286";
}

.fa-fort-awesome-alt:before {
    content: "\f3a3";
}

.fa-forumbee:before {
    content: "\f211";
}

.fa-forward:before {
    content: "\f04e";
}

.fa-foursquare:before {
    content: "\f180";
}

.fa-free-code-camp:before {
    content: "\f2c5";
}

.fa-freebsd:before {
    content: "\f3a4";
}

.fa-frog:before {
    content: "\f52e";
}

.fa-frown:before {
    content: "\f119";
}

.fa-frown-open:before {
    content: "\f57a";
}

.fa-fulcrum:before {
    content: "\f50b";
}

.fa-funnel-dollar:before {
    content: "\f662";
}

.fa-futbol:before {
    content: "\f1e3";
}

.fa-galactic-republic:before {
    content: "\f50c";
}

.fa-galactic-senate:before {
    content: "\f50d";
}

.fa-gamepad:before {
    content: "\f11b";
}

.fa-gas-pump:before {
    content: "\f52f";
}

.fa-gavel:before {
    content: "\f0e3";
}

.fa-gem:before {
    content: "\f3a5";
}

.fa-genderless:before {
    content: "\f22d";
}

.fa-get-pocket:before {
    content: "\f265";
}

.fa-gg:before {
    content: "\f260";
}

.fa-gg-circle:before {
    content: "\f261";
}

.fa-ghost:before {
    content: "\f6e2";
}

.fa-gift:before {
    content: "\f06b";
}

.fa-gifts:before {
    content: "\f79c";
}

.fa-git:before {
    content: "\f1d3";
}

.fa-git-alt:before {
    content: "\f841";
}

.fa-git-square:before {
    content: "\f1d2";
}

.fa-github:before {
    content: "\f09b";
}

.fa-github-alt:before {
    content: "\f113";
}

.fa-github-square:before {
    content: "\f092";
}

.fa-gitkraken:before {
    content: "\f3a6";
}

.fa-gitlab:before {
    content: "\f296";
}

.fa-gitter:before {
    content: "\f426";
}

.fa-glass-cheers:before {
    content: "\f79f";
}

.fa-glass-martini:before {
    content: "\f000";
}

.fa-glass-martini-alt:before {
    content: "\f57b";
}

.fa-glass-whiskey:before {
    content: "\f7a0";
}

.fa-glasses:before {
    content: "\f530";
}

.fa-glide:before {
    content: "\f2a5";
}

.fa-glide-g:before {
    content: "\f2a6";
}

.fa-globe:before {
    content: "\f0ac";
}

.fa-globe-africa:before {
    content: "\f57c";
}

.fa-globe-americas:before {
    content: "\f57d";
}

.fa-globe-asia:before {
    content: "\f57e";
}

.fa-globe-europe:before {
    content: "\f7a2";
}

.fa-gofore:before {
    content: "\f3a7";
}

.fa-golf-ball:before {
    content: "\f450";
}

.fa-goodreads:before {
    content: "\f3a8";
}

.fa-goodreads-g:before {
    content: "\f3a9";
}

.fa-google:before {
    content: "\f1a0";
}

.fa-google-drive:before {
    content: "\f3aa";
}

.fa-google-pay:before {
    content: "\e079";
}

.fa-google-play:before {
    content: "\f3ab";
}

.fa-google-plus:before {
    content: "\f2b3";
}

.fa-google-plus-g:before {
    content: "\f0d5";
}

.fa-google-plus-square:before {
    content: "\f0d4";
}

.fa-google-wallet:before {
    content: "\f1ee";
}

.fa-gopuram:before {
    content: "\f664";
}

.fa-graduation-cap:before {
    content: "\f19d";
}

.fa-gratipay:before {
    content: "\f184";
}

.fa-grav:before {
    content: "\f2d6";
}

.fa-greater-than:before {
    content: "\f531";
}

.fa-greater-than-equal:before {
    content: "\f532";
}

.fa-grimace:before {
    content: "\f57f";
}

.fa-grin:before {
    content: "\f580";
}

.fa-grin-alt:before {
    content: "\f581";
}

.fa-grin-beam:before {
    content: "\f582";
}

.fa-grin-beam-sweat:before {
    content: "\f583";
}

.fa-grin-hearts:before {
    content: "\f584";
}

.fa-grin-squint:before {
    content: "\f585";
}

.fa-grin-squint-tears:before {
    content: "\f586";
}

.fa-grin-stars:before {
    content: "\f587";
}

.fa-grin-tears:before {
    content: "\f588";
}

.fa-grin-tongue:before {
    content: "\f589";
}

.fa-grin-tongue-squint:before {
    content: "\f58a";
}

.fa-grin-tongue-wink:before {
    content: "\f58b";
}

.fa-grin-wink:before {
    content: "\f58c";
}

.fa-grip-horizontal:before {
    content: "\f58d";
}

.fa-grip-lines:before {
    content: "\f7a4";
}

.fa-grip-lines-vertical:before {
    content: "\f7a5";
}

.fa-grip-vertical:before {
    content: "\f58e";
}

.fa-gripfire:before {
    content: "\f3ac";
}

.fa-grunt:before {
    content: "\f3ad";
}

.fa-guilded:before {
    content: "\e07e";
}

.fa-guitar:before {
    content: "\f7a6";
}

.fa-gulp:before {
    content: "\f3ae";
}

.fa-h-square:before {
    content: "\f0fd";
}

.fa-hacker-news:before {
    content: "\f1d4";
}

.fa-hacker-news-square:before {
    content: "\f3af";
}

.fa-hackerrank:before {
    content: "\f5f7";
}

.fa-hamburger:before {
    content: "\f805";
}

.fa-hammer:before {
    content: "\f6e3";
}

.fa-hamsa:before {
    content: "\f665";
}

.fa-hand-holding:before {
    content: "\f4bd";
}

.fa-hand-holding-heart:before {
    content: "\f4be";
}

.fa-hand-holding-medical:before {
    content: "\e05c";
}

.fa-hand-holding-usd:before {
    content: "\f4c0";
}

.fa-hand-holding-water:before {
    content: "\f4c1";
}

.fa-hand-lizard:before {
    content: "\f258";
}

.fa-hand-middle-finger:before {
    content: "\f806";
}

.fa-hand-paper:before {
    content: "\f256";
}

.fa-hand-peace:before {
    content: "\f25b";
}

.fa-hand-point-down:before {
    content: "\f0a7";
}

.fa-hand-point-left:before {
    content: "\f0a5";
}

.fa-hand-point-right:before {
    content: "\f0a4";
}

.fa-hand-point-up:before {
    content: "\f0a6";
}

.fa-hand-pointer:before {
    content: "\f25a";
}

.fa-hand-rock:before {
    content: "\f255";
}

.fa-hand-scissors:before {
    content: "\f257";
}

.fa-hand-sparkles:before {
    content: "\e05d";
}

.fa-hand-spock:before {
    content: "\f259";
}

.fa-hands:before {
    content: "\f4c2";
}

.fa-hands-helping:before {
    content: "\f4c4";
}

.fa-hands-wash:before {
    content: "\e05e";
}

.fa-handshake:before {
    content: "\f2b5";
}

.fa-handshake-alt-slash:before {
    content: "\e05f";
}

.fa-handshake-slash:before {
    content: "\e060";
}

.fa-hanukiah:before {
    content: "\f6e6";
}

.fa-hard-hat:before {
    content: "\f807";
}

.fa-hashtag:before {
    content: "\f292";
}

.fa-hat-cowboy:before {
    content: "\f8c0";
}

.fa-hat-cowboy-side:before {
    content: "\f8c1";
}

.fa-hat-wizard:before {
    content: "\f6e8";
}

.fa-hdd:before {
    content: "\f0a0";
}

.fa-head-side-cough:before {
    content: "\e061";
}

.fa-head-side-cough-slash:before {
    content: "\e062";
}

.fa-head-side-mask:before {
    content: "\e063";
}

.fa-head-side-virus:before {
    content: "\e064";
}

.fa-heading:before {
    content: "\f1dc";
}

.fa-headphones:before {
    content: "\f025";
}

.fa-headphones-alt:before {
    content: "\f58f";
}

.fa-headset:before {
    content: "\f590";
}

.fa-heart:before {
    content: "\f004";
}

.fa-heart-broken:before {
    content: "\f7a9";
}

.fa-heartbeat:before {
    content: "\f21e";
}

.fa-helicopter:before {
    content: "\f533";
}

.fa-highlighter:before {
    content: "\f591";
}

.fa-hiking:before {
    content: "\f6ec";
}

.fa-hippo:before {
    content: "\f6ed";
}

.fa-hips:before {
    content: "\f452";
}

.fa-hire-a-helper:before {
    content: "\f3b0";
}

.fa-history:before {
    content: "\f1da";
}

.fa-hive:before {
    content: "\e07f";
}

.fa-hockey-puck:before {
    content: "\f453";
}

.fa-holly-berry:before {
    content: "\f7aa";
}

.fa-home:before {
    content: "\f015";
}

.fa-hooli:before {
    content: "\f427";
}

.fa-hornbill:before {
    content: "\f592";
}

.fa-horse:before {
    content: "\f6f0";
}

.fa-horse-head:before {
    content: "\f7ab";
}

.fa-hospital:before {
    content: "\f0f8";
}

.fa-hospital-alt:before {
    content: "\f47d";
}

.fa-hospital-symbol:before {
    content: "\f47e";
}

.fa-hospital-user:before {
    content: "\f80d";
}

.fa-hot-tub:before {
    content: "\f593";
}

.fa-hotdog:before {
    content: "\f80f";
}

.fa-hotel:before {
    content: "\f594";
}

.fa-hotjar:before {
    content: "\f3b1";
}

.fa-hourglass:before {
    content: "\f254";
}

.fa-hourglass-end:before {
    content: "\f253";
}

.fa-hourglass-half:before {
    content: "\f252";
}

.fa-hourglass-start:before {
    content: "\f251";
}

.fa-house-damage:before {
    content: "\f6f1";
}

.fa-house-user:before {
    content: "\e065";
}

.fa-houzz:before {
    content: "\f27c";
}

.fa-hryvnia:before {
    content: "\f6f2";
}

.fa-html5:before {
    content: "\f13b";
}

.fa-hubspot:before {
    content: "\f3b2";
}

.fa-i-cursor:before {
    content: "\f246";
}

.fa-ice-cream:before {
    content: "\f810";
}

.fa-icicles:before {
    content: "\f7ad";
}

.fa-icons:before {
    content: "\f86d";
}

.fa-id-badge:before {
    content: "\f2c1";
}

.fa-id-card:before {
    content: "\f2c2";
}

.fa-id-card-alt:before {
    content: "\f47f";
}

.fa-ideal:before {
    content: "\e013";
}

.fa-igloo:before {
    content: "\f7ae";
}

.fa-image:before {
    content: "\f03e";
}

.fa-images:before {
    content: "\f302";
}

.fa-imdb:before {
    content: "\f2d8";
}

.fa-inbox:before {
    content: "\f01c";
}

.fa-indent:before {
    content: "\f03c";
}

.fa-industry:before {
    content: "\f275";
}

.fa-infinity:before {
    content: "\f534";
}

.fa-info:before {
    content: "\f129";
}

.fa-info-circle:before {
    content: "\f05a";
}

.fa-innosoft:before {
    content: "\e080";
}

.fa-instagram:before {
    content: "\f16d";
}

.fa-instagram-square:before {
    content: "\e055";
}

.fa-instalod:before {
    content: "\e081";
}

.fa-intercom:before {
    content: "\f7af";
}

.fa-internet-explorer:before {
    content: "\f26b";
}

.fa-invision:before {
    content: "\f7b0";
}

.fa-ioxhost:before {
    content: "\f208";
}

.fa-italic:before {
    content: "\f033";
}

.fa-itch-io:before {
    content: "\f83a";
}

.fa-itunes:before {
    content: "\f3b4";
}

.fa-itunes-note:before {
    content: "\f3b5";
}

.fa-java:before {
    content: "\f4e4";
}

.fa-jedi:before {
    content: "\f669";
}

.fa-jedi-order:before {
    content: "\f50e";
}

.fa-jenkins:before {
    content: "\f3b6";
}

.fa-jira:before {
    content: "\f7b1";
}

.fa-joget:before {
    content: "\f3b7";
}

.fa-joint:before {
    content: "\f595";
}

.fa-joomla:before {
    content: "\f1aa";
}

.fa-journal-whills:before {
    content: "\f66a";
}

.fa-js:before {
    content: "\f3b8";
}

.fa-js-square:before {
    content: "\f3b9";
}

.fa-jsfiddle:before {
    content: "\f1cc";
}

.fa-kaaba:before {
    content: "\f66b";
}

.fa-kaggle:before {
    content: "\f5fa";
}

.fa-key:before {
    content: "\f084";
}

.fa-keybase:before {
    content: "\f4f5";
}

.fa-keyboard:before {
    content: "\f11c";
}

.fa-keycdn:before {
    content: "\f3ba";
}

.fa-khanda:before {
    content: "\f66d";
}

.fa-kickstarter:before {
    content: "\f3bb";
}

.fa-kickstarter-k:before {
    content: "\f3bc";
}

.fa-kiss:before {
    content: "\f596";
}

.fa-kiss-beam:before {
    content: "\f597";
}

.fa-kiss-wink-heart:before {
    content: "\f598";
}

.fa-kiwi-bird:before {
    content: "\f535";
}

.fa-korvue:before {
    content: "\f42f";
}

.fa-landmark:before {
    content: "\f66f";
}

.fa-language:before {
    content: "\f1ab";
}

.fa-laptop:before {
    content: "\f109";
}

.fa-laptop-code:before {
    content: "\f5fc";
}

.fa-laptop-house:before {
    content: "\e066";
}

.fa-laptop-medical:before {
    content: "\f812";
}

.fa-laravel:before {
    content: "\f3bd";
}

.fa-lastfm:before {
    content: "\f202";
}

.fa-lastfm-square:before {
    content: "\f203";
}

.fa-laugh:before {
    content: "\f599";
}

.fa-laugh-beam:before {
    content: "\f59a";
}

.fa-laugh-squint:before {
    content: "\f59b";
}

.fa-laugh-wink:before {
    content: "\f59c";
}

.fa-layer-group:before {
    content: "\f5fd";
}

.fa-leaf:before {
    content: "\f06c";
}

.fa-leanpub:before {
    content: "\f212";
}

.fa-lemon:before {
    content: "\f094";
}

.fa-less:before {
    content: "\f41d";
}

.fa-less-than:before {
    content: "\f536";
}

.fa-less-than-equal:before {
    content: "\f537";
}

.fa-level-down-alt:before {
    content: "\f3be";
}

.fa-level-up-alt:before {
    content: "\f3bf";
}

.fa-life-ring:before {
    content: "\f1cd";
}

.fa-lightbulb:before {
    content: "\f0eb";
}

.fa-line:before {
    content: "\f3c0";
}

.fa-link:before {
    content: "\f0c1";
}

.fa-linkedin:before {
    content: "\f08c";
}

.fa-linkedin-in:before {
    content: "\f0e1";
}

.fa-linode:before {
    content: "\f2b8";
}

.fa-linux:before {
    content: "\f17c";
}

.fa-lira-sign:before {
    content: "\f195";
}

.fa-list:before {
    content: "\f03a";
}

.fa-list-alt:before {
    content: "\f022";
}

.fa-list-ol:before {
    content: "\f0cb";
}

.fa-list-ul:before {
    content: "\f0ca";
}

.fa-location-arrow:before {
    content: "\f124";
}

.fa-lock:before {
    content: "\f023";
}

.fa-lock-open:before {
    content: "\f3c1";
}

.fa-long-arrow-alt-down:before {
    content: "\f309";
}

.fa-long-arrow-alt-left:before {
    content: "\f30a";
}

.fa-long-arrow-alt-right:before {
    content: "\f30b";
}

.fa-long-arrow-alt-up:before {
    content: "\f30c";
}

.fa-low-vision:before {
    content: "\f2a8";
}

.fa-luggage-cart:before {
    content: "\f59d";
}

.fa-lungs:before {
    content: "\f604";
}

.fa-lungs-virus:before {
    content: "\e067";
}

.fa-lyft:before {
    content: "\f3c3";
}

.fa-magento:before {
    content: "\f3c4";
}

.fa-magic:before {
    content: "\f0d0";
}

.fa-magnet:before {
    content: "\f076";
}

.fa-mail-bulk:before {
    content: "\f674";
}

.fa-mailchimp:before {
    content: "\f59e";
}

.fa-male:before {
    content: "\f183";
}

.fa-mandalorian:before {
    content: "\f50f";
}

.fa-map:before {
    content: "\f279";
}

.fa-map-marked:before {
    content: "\f59f";
}

.fa-map-marked-alt:before {
    content: "\f5a0";
}

.fa-map-marker:before {
    content: "\f041";
}

.fa-map-marker-alt:before {
    content: "\f3c5";
}

.fa-map-pin:before {
    content: "\f276";
}

.fa-map-signs:before {
    content: "\f277";
}

.fa-markdown:before {
    content: "\f60f";
}

.fa-marker:before {
    content: "\f5a1";
}

.fa-mars:before {
    content: "\f222";
}

.fa-mars-double:before {
    content: "\f227";
}

.fa-mars-stroke:before {
    content: "\f229";
}

.fa-mars-stroke-h:before {
    content: "\f22b";
}

.fa-mars-stroke-v:before {
    content: "\f22a";
}

.fa-mask:before {
    content: "\f6fa";
}

.fa-mastodon:before {
    content: "\f4f6";
}

.fa-maxcdn:before {
    content: "\f136";
}

.fa-mdb:before {
    content: "\f8ca";
}

.fa-medal:before {
    content: "\f5a2";
}

.fa-medapps:before {
    content: "\f3c6";
}

.fa-medium:before {
    content: "\f23a";
}

.fa-medium-m:before {
    content: "\f3c7";
}

.fa-medkit:before {
    content: "\f0fa";
}

.fa-medrt:before {
    content: "\f3c8";
}

.fa-meetup:before {
    content: "\f2e0";
}

.fa-megaport:before {
    content: "\f5a3";
}

.fa-meh:before {
    content: "\f11a";
}

.fa-meh-blank:before {
    content: "\f5a4";
}

.fa-meh-rolling-eyes:before {
    content: "\f5a5";
}

.fa-memory:before {
    content: "\f538";
}

.fa-mendeley:before {
    content: "\f7b3";
}

.fa-menorah:before {
    content: "\f676";
}

.fa-mercury:before {
    content: "\f223";
}

.fa-meteor:before {
    content: "\f753";
}

.fa-microblog:before {
    content: "\e01a";
}

.fa-microchip:before {
    content: "\f2db";
}

.fa-microphone:before {
    content: "\f130";
}

.fa-microphone-alt:before {
    content: "\f3c9";
}

.fa-microphone-alt-slash:before {
    content: "\f539";
}

.fa-microphone-slash:before {
    content: "\f131";
}

.fa-microscope:before {
    content: "\f610";
}

.fa-microsoft:before {
    content: "\f3ca";
}

.fa-minus:before {
    content: "\f068";
}

.fa-minus-circle:before {
    content: "\f056";
}

.fa-minus-square:before {
    content: "\f146";
}

.fa-mitten:before {
    content: "\f7b5";
}

.fa-mix:before {
    content: "\f3cb";
}

.fa-mixcloud:before {
    content: "\f289";
}

.fa-mixer:before {
    content: "\e056";
}

.fa-mizuni:before {
    content: "\f3cc";
}

.fa-mobile:before {
    content: "\f10b";
}

.fa-mobile-alt:before {
    content: "\f3cd";
}

.fa-modx:before {
    content: "\f285";
}

.fa-monero:before {
    content: "\f3d0";
}

.fa-money-bill:before {
    content: "\f0d6";
}

.fa-money-bill-alt:before {
    content: "\f3d1";
}

.fa-money-bill-wave:before {
    content: "\f53a";
}

.fa-money-bill-wave-alt:before {
    content: "\f53b";
}

.fa-money-check:before {
    content: "\f53c";
}

.fa-money-check-alt:before {
    content: "\f53d";
}

.fa-monument:before {
    content: "\f5a6";
}

.fa-moon:before {
    content: "\f186";
}

.fa-mortar-pestle:before {
    content: "\f5a7";
}

.fa-mosque:before {
    content: "\f678";
}

.fa-motorcycle:before {
    content: "\f21c";
}

.fa-mountain:before {
    content: "\f6fc";
}

.fa-mouse:before {
    content: "\f8cc";
}

.fa-mouse-pointer:before {
    content: "\f245";
}

.fa-mug-hot:before {
    content: "\f7b6";
}

.fa-music:before {
    content: "\f001";
}

.fa-napster:before {
    content: "\f3d2";
}

.fa-neos:before {
    content: "\f612";
}

.fa-network-wired:before {
    content: "\f6ff";
}

.fa-neuter:before {
    content: "\f22c";
}

.fa-newspaper:before {
    content: "\f1ea";
}

.fa-nimblr:before {
    content: "\f5a8";
}

.fa-node:before {
    content: "\f419";
}

.fa-node-js:before {
    content: "\f3d3";
}

.fa-not-equal:before {
    content: "\f53e";
}

.fa-notes-medical:before {
    content: "\f481";
}

.fa-npm:before {
    content: "\f3d4";
}

.fa-ns8:before {
    content: "\f3d5";
}

.fa-nutritionix:before {
    content: "\f3d6";
}

.fa-object-group:before {
    content: "\f247";
}

.fa-object-ungroup:before {
    content: "\f248";
}

.fa-octopus-deploy:before {
    content: "\e082";
}

.fa-odnoklassniki:before {
    content: "\f263";
}

.fa-odnoklassniki-square:before {
    content: "\f264";
}

.fa-oil-can:before {
    content: "\f613";
}

.fa-old-republic:before {
    content: "\f510";
}

.fa-om:before {
    content: "\f679";
}

.fa-opencart:before {
    content: "\f23d";
}

.fa-openid:before {
    content: "\f19b";
}

.fa-opera:before {
    content: "\f26a";
}

.fa-optin-monster:before {
    content: "\f23c";
}

.fa-orcid:before {
    content: "\f8d2";
}

.fa-osi:before {
    content: "\f41a";
}

.fa-otter:before {
    content: "\f700";
}

.fa-outdent:before {
    content: "\f03b";
}

.fa-page4:before {
    content: "\f3d7";
}

.fa-pagelines:before {
    content: "\f18c";
}

.fa-pager:before {
    content: "\f815";
}

.fa-paint-brush:before {
    content: "\f1fc";
}

.fa-paint-roller:before {
    content: "\f5aa";
}

.fa-palette:before {
    content: "\f53f";
}

.fa-palfed:before {
    content: "\f3d8";
}

.fa-pallet:before {
    content: "\f482";
}

.fa-paper-plane:before {
    content: "\f1d8";
}

.fa-paperclip:before {
    content: "\f0c6";
}

.fa-parachute-box:before {
    content: "\f4cd";
}

.fa-paragraph:before {
    content: "\f1dd";
}

.fa-parking:before {
    content: "\f540";
}

.fa-passport:before {
    content: "\f5ab";
}

.fa-pastafarianism:before {
    content: "\f67b";
}

.fa-paste:before {
    content: "\f0ea";
}

.fa-patreon:before {
    content: "\f3d9";
}

.fa-pause:before {
    content: "\f04c";
}

.fa-pause-circle:before {
    content: "\f28b";
}

.fa-paw:before {
    content: "\f1b0";
}

.fa-paypal:before {
    content: "\f1ed";
}

.fa-peace:before {
    content: "\f67c";
}

.fa-pen:before {
    content: "\f304";
}

.fa-pen-alt:before {
    content: "\f305";
}

.fa-pen-fancy:before {
    content: "\f5ac";
}

.fa-pen-nib:before {
    content: "\f5ad";
}

.fa-pen-square:before {
    content: "\f14b";
}

.fa-pencil-alt:before {
    content: "\f303";
}

.fa-pencil-ruler:before {
    content: "\f5ae";
}

.fa-penny-arcade:before {
    content: "\f704";
}

.fa-people-arrows:before {
    content: "\e068";
}

.fa-people-carry:before {
    content: "\f4ce";
}

.fa-pepper-hot:before {
    content: "\f816";
}

.fa-perbyte:before {
    content: "\e083";
}

.fa-percent:before {
    content: "\f295";
}

.fa-percentage:before {
    content: "\f541";
}

.fa-periscope:before {
    content: "\f3da";
}

.fa-person-booth:before {
    content: "\f756";
}

.fa-phabricator:before {
    content: "\f3db";
}

.fa-phoenix-framework:before {
    content: "\f3dc";
}

.fa-phoenix-squadron:before {
    content: "\f511";
}

.fa-phone:before {
    content: "\f095";
}

.fa-phone-alt:before {
    content: "\f879";
}

.fa-phone-slash:before {
    content: "\f3dd";
}

.fa-phone-square:before {
    content: "\f098";
}

.fa-phone-square-alt:before {
    content: "\f87b";
}

.fa-phone-volume:before {
    content: "\f2a0";
}

.fa-photo-video:before {
    content: "\f87c";
}

.fa-php:before {
    content: "\f457";
}

.fa-pied-piper:before {
    content: "\f2ae";
}

.fa-pied-piper-alt:before {
    content: "\f1a8";
}

.fa-pied-piper-hat:before {
    content: "\f4e5";
}

.fa-pied-piper-pp:before {
    content: "\f1a7";
}

.fa-pied-piper-square:before {
    content: "\e01e";
}

.fa-piggy-bank:before {
    content: "\f4d3";
}

.fa-pills:before {
    content: "\f484";
}

.fa-pinterest:before {
    content: "\f0d2";
}

.fa-pinterest-p:before {
    content: "\f231";
}

.fa-pinterest-square:before {
    content: "\f0d3";
}

.fa-pizza-slice:before {
    content: "\f818";
}

.fa-place-of-worship:before {
    content: "\f67f";
}

.fa-plane:before {
    content: "\f072";
}

.fa-plane-arrival:before {
    content: "\f5af";
}

.fa-plane-departure:before {
    content: "\f5b0";
}

.fa-plane-slash:before {
    content: "\e069";
}

.fa-play:before {
    content: "\f04b";
}

.fa-play-circle:before {
    content: "\f144";
}

.fa-playstation:before {
    content: "\f3df";
}

.fa-plug:before {
    content: "\f1e6";
}

.fa-plus:before {
    content: "\f067";
}

.fa-plus-circle:before {
    content: "\f055";
}

.fa-plus-square:before {
    content: "\f0fe";
}

.fa-podcast:before {
    content: "\f2ce";
}

.fa-poll:before {
    content: "\f681";
}

.fa-poll-h:before {
    content: "\f682";
}

.fa-poo:before {
    content: "\f2fe";
}

.fa-poo-storm:before {
    content: "\f75a";
}

.fa-poop:before {
    content: "\f619";
}

.fa-portrait:before {
    content: "\f3e0";
}

.fa-pound-sign:before {
    content: "\f154";
}

.fa-power-off:before {
    content: "\f011";
}

.fa-pray:before {
    content: "\f683";
}

.fa-praying-hands:before {
    content: "\f684";
}

.fa-prescription:before {
    content: "\f5b1";
}

.fa-prescription-bottle:before {
    content: "\f485";
}

.fa-prescription-bottle-alt:before {
    content: "\f486";
}

.fa-print:before {
    content: "\f02f";
}

.fa-procedures:before {
    content: "\f487";
}

.fa-product-hunt:before {
    content: "\f288";
}

.fa-project-diagram:before {
    content: "\f542";
}

.fa-pump-medical:before {
    content: "\e06a";
}

.fa-pump-soap:before {
    content: "\e06b";
}

.fa-pushed:before {
    content: "\f3e1";
}

.fa-puzzle-piece:before {
    content: "\f12e";
}

.fa-python:before {
    content: "\f3e2";
}

.fa-qq:before {
    content: "\f1d6";
}

.fa-qrcode:before {
    content: "\f029";
}

.fa-question:before {
    content: "\f128";
}

.fa-question-circle:before {
    content: "\f059";
}

.fa-quidditch:before {
    content: "\f458";
}

.fa-quinscape:before {
    content: "\f459";
}

.fa-quora:before {
    content: "\f2c4";
}

.fa-quote-left:before {
    content: "\f10d";
}

.fa-quote-right:before {
    content: "\f10e";
}

.fa-quran:before {
    content: "\f687";
}

.fa-r-project:before {
    content: "\f4f7";
}

.fa-radiation:before {
    content: "\f7b9";
}

.fa-radiation-alt:before {
    content: "\f7ba";
}

.fa-rainbow:before {
    content: "\f75b";
}

.fa-random:before {
    content: "\f074";
}

.fa-raspberry-pi:before {
    content: "\f7bb";
}

.fa-ravelry:before {
    content: "\f2d9";
}

.fa-react:before {
    content: "\f41b";
}

.fa-reacteurope:before {
    content: "\f75d";
}

.fa-readme:before {
    content: "\f4d5";
}

.fa-rebel:before {
    content: "\f1d0";
}

.fa-receipt:before {
    content: "\f543";
}

.fa-record-vinyl:before {
    content: "\f8d9";
}

.fa-recycle:before {
    content: "\f1b8";
}

.fa-red-river:before {
    content: "\f3e3";
}

.fa-reddit:before {
    content: "\f1a1";
}

.fa-reddit-alien:before {
    content: "\f281";
}

.fa-reddit-square:before {
    content: "\f1a2";
}

.fa-redhat:before {
    content: "\f7bc";
}

.fa-redo:before {
    content: "\f01e";
}

.fa-redo-alt:before {
    content: "\f2f9";
}

.fa-registered:before {
    content: "\f25d";
}

.fa-remove-format:before {
    content: "\f87d";
}

.fa-renren:before {
    content: "\f18b";
}

.fa-reply:before {
    content: "\f3e5";
}

.fa-reply-all:before {
    content: "\f122";
}

.fa-replyd:before {
    content: "\f3e6";
}

.fa-republican:before {
    content: "\f75e";
}

.fa-researchgate:before {
    content: "\f4f8";
}

.fa-resolving:before {
    content: "\f3e7";
}

.fa-restroom:before {
    content: "\f7bd";
}

.fa-retweet:before {
    content: "\f079";
}

.fa-rev:before {
    content: "\f5b2";
}

.fa-ribbon:before {
    content: "\f4d6";
}

.fa-ring:before {
    content: "\f70b";
}

.fa-road:before {
    content: "\f018";
}

.fa-robot:before {
    content: "\f544";
}

.fa-rocket:before {
    content: "\f135";
}

.fa-rocketchat:before {
    content: "\f3e8";
}

.fa-rockrms:before {
    content: "\f3e9";
}

.fa-route:before {
    content: "\f4d7";
}

.fa-rss:before {
    content: "\f09e";
}

.fa-rss-square:before {
    content: "\f143";
}

.fa-ruble-sign:before {
    content: "\f158";
}

.fa-ruler:before {
    content: "\f545";
}

.fa-ruler-combined:before {
    content: "\f546";
}

.fa-ruler-horizontal:before {
    content: "\f547";
}

.fa-ruler-vertical:before {
    content: "\f548";
}

.fa-running:before {
    content: "\f70c";
}

.fa-rupee-sign:before {
    content: "\f156";
}

.fa-rust:before {
    content: "\e07a";
}

.fa-sad-cry:before {
    content: "\f5b3";
}

.fa-sad-tear:before {
    content: "\f5b4";
}

.fa-safari:before {
    content: "\f267";
}

.fa-salesforce:before {
    content: "\f83b";
}

.fa-sass:before {
    content: "\f41e";
}

.fa-satellite:before {
    content: "\f7bf";
}

.fa-satellite-dish:before {
    content: "\f7c0";
}

.fa-save:before {
    content: "\f0c7";
}

.fa-schlix:before {
    content: "\f3ea";
}

.fa-school:before {
    content: "\f549";
}

.fa-screwdriver:before {
    content: "\f54a";
}

.fa-scribd:before {
    content: "\f28a";
}

.fa-scroll:before {
    content: "\f70e";
}

.fa-sd-card:before {
    content: "\f7c2";
}

.fa-search:before {
    content: "\f002";
}

.fa-search-dollar:before {
    content: "\f688";
}

.fa-search-location:before {
    content: "\f689";
}

.fa-search-minus:before {
    content: "\f010";
}

.fa-search-plus:before {
    content: "\f00e";
}

.fa-searchengin:before {
    content: "\f3eb";
}

.fa-seedling:before {
    content: "\f4d8";
}

.fa-sellcast:before {
    content: "\f2da";
}

.fa-sellsy:before {
    content: "\f213";
}

.fa-server:before {
    content: "\f233";
}

.fa-servicestack:before {
    content: "\f3ec";
}

.fa-shapes:before {
    content: "\f61f";
}

.fa-share:before {
    content: "\f064";
}

.fa-share-alt:before {
    content: "\f1e0";
}

.fa-share-alt-square:before {
    content: "\f1e1";
}

.fa-share-square:before {
    content: "\f14d";
}

.fa-shekel-sign:before {
    content: "\f20b";
}

.fa-shield-alt:before {
    content: "\f3ed";
}

.fa-shield-virus:before {
    content: "\e06c";
}

.fa-ship:before {
    content: "\f21a";
}

.fa-shipping-fast:before {
    content: "\f48b";
}

.fa-shirtsinbulk:before {
    content: "\f214";
}

.fa-shoe-prints:before {
    content: "\f54b";
}

.fa-shopify:before {
    content: "\e057";
}

.fa-shopping-bag:before {
    content: "\f290";
}

.fa-shopping-basket:before {
    content: "\f291";
}

.fa-shopping-cart:before {
    content: "\f07a";
}

.fa-shopware:before {
    content: "\f5b5";
}

.fa-shower:before {
    content: "\f2cc";
}

.fa-shuttle-van:before {
    content: "\f5b6";
}

.fa-sign:before {
    content: "\f4d9";
}

.fa-sign-in-alt:before {
    content: "\f2f6";
}

.fa-sign-language:before {
    content: "\f2a7";
}

.fa-sign-out-alt:before {
    content: "\f2f5";
}

.fa-signal:before {
    content: "\f012";
}

.fa-signature:before {
    content: "\f5b7";
}

.fa-sim-card:before {
    content: "\f7c4";
}

.fa-simplybuilt:before {
    content: "\f215";
}

.fa-sink:before {
    content: "\e06d";
}

.fa-sistrix:before {
    content: "\f3ee";
}

.fa-sitemap:before {
    content: "\f0e8";
}

.fa-sith:before {
    content: "\f512";
}

.fa-skating:before {
    content: "\f7c5";
}

.fa-sketch:before {
    content: "\f7c6";
}

.fa-skiing:before {
    content: "\f7c9";
}

.fa-skiing-nordic:before {
    content: "\f7ca";
}

.fa-skull:before {
    content: "\f54c";
}

.fa-skull-crossbones:before {
    content: "\f714";
}

.fa-skyatlas:before {
    content: "\f216";
}

.fa-skype:before {
    content: "\f17e";
}

.fa-slack:before {
    content: "\f198";
}

.fa-slack-hash:before {
    content: "\f3ef";
}

.fa-slash:before {
    content: "\f715";
}

.fa-sleigh:before {
    content: "\f7cc";
}

.fa-sliders-h:before {
    content: "\f1de";
}

.fa-slideshare:before {
    content: "\f1e7";
}

.fa-smile:before {
    content: "\f118";
}

.fa-smile-beam:before {
    content: "\f5b8";
}

.fa-smile-wink:before {
    content: "\f4da";
}

.fa-smog:before {
    content: "\f75f";
}

.fa-smoking:before {
    content: "\f48d";
}

.fa-smoking-ban:before {
    content: "\f54d";
}

.fa-sms:before {
    content: "\f7cd";
}

.fa-snapchat:before {
    content: "\f2ab";
}

.fa-snapchat-ghost:before {
    content: "\f2ac";
}

.fa-snapchat-square:before {
    content: "\f2ad";
}

.fa-snowboarding:before {
    content: "\f7ce";
}

.fa-snowflake:before {
    content: "\f2dc";
}

.fa-snowman:before {
    content: "\f7d0";
}

.fa-snowplow:before {
    content: "\f7d2";
}

.fa-soap:before {
    content: "\e06e";
}

.fa-socks:before {
    content: "\f696";
}

.fa-solar-panel:before {
    content: "\f5ba";
}

.fa-sort:before {
    content: "\f0dc";
}

.fa-sort-alpha-down:before {
    content: "\f15d";
}

.fa-sort-alpha-down-alt:before {
    content: "\f881";
}

.fa-sort-alpha-up:before {
    content: "\f15e";
}

.fa-sort-alpha-up-alt:before {
    content: "\f882";
}

.fa-sort-amount-down:before {
    content: "\f160";
}

.fa-sort-amount-down-alt:before {
    content: "\f884";
}

.fa-sort-amount-up:before {
    content: "\f161";
}

.fa-sort-amount-up-alt:before {
    content: "\f885";
}

.fa-sort-down:before {
    content: "\f0dd";
}

.fa-sort-numeric-down:before {
    content: "\f162";
}

.fa-sort-numeric-down-alt:before {
    content: "\f886";
}

.fa-sort-numeric-up:before {
    content: "\f163";
}

.fa-sort-numeric-up-alt:before {
    content: "\f887";
}

.fa-sort-up:before {
    content: "\f0de";
}

.fa-soundcloud:before {
    content: "\f1be";
}

.fa-sourcetree:before {
    content: "\f7d3";
}

.fa-spa:before {
    content: "\f5bb";
}

.fa-space-shuttle:before {
    content: "\f197";
}

.fa-speakap:before {
    content: "\f3f3";
}

.fa-speaker-deck:before {
    content: "\f83c";
}

.fa-spell-check:before {
    content: "\f891";
}

.fa-spider:before {
    content: "\f717";
}

.fa-spinner:before {
    content: "\f110";
}

.fa-splotch:before {
    content: "\f5bc";
}

.fa-spotify:before {
    content: "\f1bc";
}

.fa-spray-can:before {
    content: "\f5bd";
}

.fa-square:before {
    content: "\f0c8";
}

.fa-square-full:before {
    content: "\f45c";
}

.fa-square-root-alt:before {
    content: "\f698";
}

.fa-squarespace:before {
    content: "\f5be";
}

.fa-stack-exchange:before {
    content: "\f18d";
}

.fa-stack-overflow:before {
    content: "\f16c";
}

.fa-stackpath:before {
    content: "\f842";
}

.fa-stamp:before {
    content: "\f5bf";
}

.fa-star:before {
    content: "\f005";
}

.fa-star-and-crescent:before {
    content: "\f699";
}

.fa-star-half:before {
    content: "\f089";
}

.fa-star-half-alt:before {
    content: "\f5c0";
}

.fa-star-of-david:before {
    content: "\f69a";
}

.fa-star-of-life:before {
    content: "\f621";
}

.fa-staylinked:before {
    content: "\f3f5";
}

.fa-steam:before {
    content: "\f1b6";
}

.fa-steam-square:before {
    content: "\f1b7";
}

.fa-steam-symbol:before {
    content: "\f3f6";
}

.fa-step-backward:before {
    content: "\f048";
}

.fa-step-forward:before {
    content: "\f051";
}

.fa-stethoscope:before {
    content: "\f0f1";
}

.fa-sticker-mule:before {
    content: "\f3f7";
}

.fa-sticky-note:before {
    content: "\f249";
}

.fa-stop:before {
    content: "\f04d";
}

.fa-stop-circle:before {
    content: "\f28d";
}

.fa-stopwatch:before {
    content: "\f2f2";
}

.fa-stopwatch-20:before {
    content: "\e06f";
}

.fa-store:before {
    content: "\f54e";
}

.fa-store-alt:before {
    content: "\f54f";
}

.fa-store-alt-slash:before {
    content: "\e070";
}

.fa-store-slash:before {
    content: "\e071";
}

.fa-strava:before {
    content: "\f428";
}

.fa-stream:before {
    content: "\f550";
}

.fa-street-view:before {
    content: "\f21d";
}

.fa-strikethrough:before {
    content: "\f0cc";
}

.fa-stripe:before {
    content: "\f429";
}

.fa-stripe-s:before {
    content: "\f42a";
}

.fa-stroopwafel:before {
    content: "\f551";
}

.fa-studiovinari:before {
    content: "\f3f8";
}

.fa-stumbleupon:before {
    content: "\f1a4";
}

.fa-stumbleupon-circle:before {
    content: "\f1a3";
}

.fa-subscript:before {
    content: "\f12c";
}

.fa-subway:before {
    content: "\f239";
}

.fa-suitcase:before {
    content: "\f0f2";
}

.fa-suitcase-rolling:before {
    content: "\f5c1";
}

.fa-sun:before {
    content: "\f185";
}

.fa-superpowers:before {
    content: "\f2dd";
}

.fa-superscript:before {
    content: "\f12b";
}

.fa-supple:before {
    content: "\f3f9";
}

.fa-surprise:before {
    content: "\f5c2";
}

.fa-suse:before {
    content: "\f7d6";
}

.fa-swatchbook:before {
    content: "\f5c3";
}

.fa-swift:before {
    content: "\f8e1";
}

.fa-swimmer:before {
    content: "\f5c4";
}

.fa-swimming-pool:before {
    content: "\f5c5";
}

.fa-symfony:before {
    content: "\f83d";
}

.fa-synagogue:before {
    content: "\f69b";
}

.fa-sync:before {
    content: "\f021";
}

.fa-sync-alt:before {
    content: "\f2f1";
}

.fa-syringe:before {
    content: "\f48e";
}

.fa-table:before {
    content: "\f0ce";
}

.fa-table-tennis:before {
    content: "\f45d";
}

.fa-tablet:before {
    content: "\f10a";
}

.fa-tablet-alt:before {
    content: "\f3fa";
}

.fa-tablets:before {
    content: "\f490";
}

.fa-tachometer-alt:before {
    content: "\f3fd";
}

.fa-tag:before {
    content: "\f02b";
}

.fa-tags:before {
    content: "\f02c";
}

.fa-tape:before {
    content: "\f4db";
}

.fa-tasks:before {
    content: "\f0ae";
}

.fa-taxi:before {
    content: "\f1ba";
}

.fa-teamspeak:before {
    content: "\f4f9";
}

.fa-teeth:before {
    content: "\f62e";
}

.fa-teeth-open:before {
    content: "\f62f";
}

.fa-telegram:before {
    content: "\f2c6";
}

.fa-telegram-plane:before {
    content: "\f3fe";
}

.fa-temperature-high:before {
    content: "\f769";
}

.fa-temperature-low:before {
    content: "\f76b";
}

.fa-tencent-weibo:before {
    content: "\f1d5";
}

.fa-tenge:before {
    content: "\f7d7";
}

.fa-terminal:before {
    content: "\f120";
}

.fa-text-height:before {
    content: "\f034";
}

.fa-text-width:before {
    content: "\f035";
}

.fa-th:before {
    content: "\f00a";
}

.fa-th-large:before {
    content: "\f009";
}

.fa-th-list:before {
    content: "\f00b";
}

.fa-the-red-yeti:before {
    content: "\f69d";
}

.fa-theater-masks:before {
    content: "\f630";
}

.fa-themeco:before {
    content: "\f5c6";
}

.fa-themeisle:before {
    content: "\f2b2";
}

.fa-thermometer:before {
    content: "\f491";
}

.fa-thermometer-empty:before {
    content: "\f2cb";
}

.fa-thermometer-full:before {
    content: "\f2c7";
}

.fa-thermometer-half:before {
    content: "\f2c9";
}

.fa-thermometer-quarter:before {
    content: "\f2ca";
}

.fa-thermometer-three-quarters:before {
    content: "\f2c8";
}

.fa-think-peaks:before {
    content: "\f731";
}

.fa-thumbs-down:before {
    content: "\f165";
}

.fa-thumbs-up:before {
    content: "\f164";
}

.fa-thumbtack:before {
    content: "\f08d";
}

.fa-ticket-alt:before {
    content: "\f3ff";
}

.fa-tiktok:before {
    content: "\e07b";
}

.fa-times:before {
    content: "\f00d";
}

.fa-times-circle:before {
    content: "\f057";
}

.fa-tint:before {
    content: "\f043";
}

.fa-tint-slash:before {
    content: "\f5c7";
}

.fa-tired:before {
    content: "\f5c8";
}

.fa-toggle-off:before {
    content: "\f204";
}

.fa-toggle-on:before {
    content: "\f205";
}

.fa-toilet:before {
    content: "\f7d8";
}

.fa-toilet-paper:before {
    content: "\f71e";
}

.fa-toilet-paper-slash:before {
    content: "\e072";
}

.fa-toolbox:before {
    content: "\f552";
}

.fa-tools:before {
    content: "\f7d9";
}

.fa-tooth:before {
    content: "\f5c9";
}

.fa-torah:before {
    content: "\f6a0";
}

.fa-torii-gate:before {
    content: "\f6a1";
}

.fa-tractor:before {
    content: "\f722";
}

.fa-trade-federation:before {
    content: "\f513";
}

.fa-trademark:before {
    content: "\f25c";
}

.fa-traffic-light:before {
    content: "\f637";
}

.fa-trailer:before {
    content: "\e041";
}

.fa-train:before {
    content: "\f238";
}

.fa-tram:before {
    content: "\f7da";
}

.fa-transgender:before {
    content: "\f224";
}

.fa-transgender-alt:before {
    content: "\f225";
}

.fa-trash:before {
    content: "\f1f8";
}

.fa-trash-alt:before {
    content: "\f2ed";
}

.fa-trash-restore:before {
    content: "\f829";
}

.fa-trash-restore-alt:before {
    content: "\f82a";
}

.fa-tree:before {
    content: "\f1bb";
}

.fa-trello:before {
    content: "\f181";
}

.fa-tripadvisor:before {
    content: "\f262";
}

.fa-trophy:before {
    content: "\f091";
}

.fa-truck:before {
    content: "\f0d1";
}

.fa-truck-loading:before {
    content: "\f4de";
}

.fa-truck-monster:before {
    content: "\f63b";
}

.fa-truck-moving:before {
    content: "\f4df";
}

.fa-truck-pickup:before {
    content: "\f63c";
}

.fa-tshirt:before {
    content: "\f553";
}

.fa-tty:before {
    content: "\f1e4";
}

.fa-tumblr:before {
    content: "\f173";
}

.fa-tumblr-square:before {
    content: "\f174";
}

.fa-tv:before {
    content: "\f26c";
}

.fa-twitch:before {
    content: "\f1e8";
}

.fa-twitter:before {
    content: "\f099";
}

.fa-twitter-square:before {
    content: "\f081";
}

.fa-typo3:before {
    content: "\f42b";
}

.fa-uber:before {
    content: "\f402";
}

.fa-ubuntu:before {
    content: "\f7df";
}

.fa-uikit:before {
    content: "\f403";
}

.fa-umbraco:before {
    content: "\f8e8";
}

.fa-umbrella:before {
    content: "\f0e9";
}

.fa-umbrella-beach:before {
    content: "\f5ca";
}

.fa-uncharted:before {
    content: "\e084";
}

.fa-underline:before {
    content: "\f0cd";
}

.fa-undo:before {
    content: "\f0e2";
}

.fa-undo-alt:before {
    content: "\f2ea";
}

.fa-uniregistry:before {
    content: "\f404";
}

.fa-unity:before {
    content: "\e049";
}

.fa-universal-access:before {
    content: "\f29a";
}

.fa-university:before {
    content: "\f19c";
}

.fa-unlink:before {
    content: "\f127";
}

.fa-unlock:before {
    content: "\f09c";
}

.fa-unlock-alt:before {
    content: "\f13e";
}

.fa-unsplash:before {
    content: "\e07c";
}

.fa-untappd:before {
    content: "\f405";
}

.fa-upload:before {
    content: "\f093";
}

.fa-ups:before {
    content: "\f7e0";
}

.fa-usb:before {
    content: "\f287";
}

.fa-user:before {
    content: "\f007";
}

.fa-user-alt:before {
    content: "\f406";
}

.fa-user-alt-slash:before {
    content: "\f4fa";
}

.fa-user-astronaut:before {
    content: "\f4fb";
}

.fa-user-check:before {
    content: "\f4fc";
}

.fa-user-circle:before {
    content: "\f2bd";
}

.fa-user-clock:before {
    content: "\f4fd";
}

.fa-user-cog:before {
    content: "\f4fe";
}

.fa-user-edit:before {
    content: "\f4ff";
}

.fa-user-friends:before {
    content: "\f500";
}

.fa-user-graduate:before {
    content: "\f501";
}

.fa-user-injured:before {
    content: "\f728";
}

.fa-user-lock:before {
    content: "\f502";
}

.fa-user-md:before {
    content: "\f0f0";
}

.fa-user-minus:before {
    content: "\f503";
}

.fa-user-ninja:before {
    content: "\f504";
}

.fa-user-nurse:before {
    content: "\f82f";
}

.fa-user-plus:before {
    content: "\f234";
}

.fa-user-secret:before {
    content: "\f21b";
}

.fa-user-shield:before {
    content: "\f505";
}

.fa-user-slash:before {
    content: "\f506";
}

.fa-user-tag:before {
    content: "\f507";
}

.fa-user-tie:before {
    content: "\f508";
}

.fa-user-times:before {
    content: "\f235";
}

.fa-users:before {
    content: "\f0c0";
}

.fa-users-cog:before {
    content: "\f509";
}

.fa-users-slash:before {
    content: "\e073";
}

.fa-usps:before {
    content: "\f7e1";
}

.fa-ussunnah:before {
    content: "\f407";
}

.fa-utensil-spoon:before {
    content: "\f2e5";
}

.fa-utensils:before {
    content: "\f2e7";
}

.fa-vaadin:before {
    content: "\f408";
}

.fa-vector-square:before {
    content: "\f5cb";
}

.fa-venus:before {
    content: "\f221";
}

.fa-venus-double:before {
    content: "\f226";
}

.fa-venus-mars:before {
    content: "\f228";
}

.fa-vest:before {
    content: "\e085";
}

.fa-vest-patches:before {
    content: "\e086";
}

.fa-viacoin:before {
    content: "\f237";
}

.fa-viadeo:before {
    content: "\f2a9";
}

.fa-viadeo-square:before {
    content: "\f2aa";
}

.fa-vial:before {
    content: "\f492";
}

.fa-vials:before {
    content: "\f493";
}

.fa-viber:before {
    content: "\f409";
}

.fa-video:before {
    content: "\f03d";
}

.fa-video-slash:before {
    content: "\f4e2";
}

.fa-vihara:before {
    content: "\f6a7";
}

.fa-vimeo:before {
    content: "\f40a";
}

.fa-vimeo-square:before {
    content: "\f194";
}

.fa-vimeo-v:before {
    content: "\f27d";
}

.fa-vine:before {
    content: "\f1ca";
}

.fa-virus:before {
    content: "\e074";
}

.fa-virus-slash:before {
    content: "\e075";
}

.fa-viruses:before {
    content: "\e076";
}

.fa-vk:before {
    content: "\f189";
}

.fa-vnv:before {
    content: "\f40b";
}

.fa-voicemail:before {
    content: "\f897";
}

.fa-volleyball-ball:before {
    content: "\f45f";
}

.fa-volume-down:before {
    content: "\f027";
}

.fa-volume-mute:before {
    content: "\f6a9";
}

.fa-volume-off:before {
    content: "\f026";
}

.fa-volume-up:before {
    content: "\f028";
}

.fa-vote-yea:before {
    content: "\f772";
}

.fa-vr-cardboard:before {
    content: "\f729";
}

.fa-vuejs:before {
    content: "\f41f";
}

.fa-walking:before {
    content: "\f554";
}

.fa-wallet:before {
    content: "\f555";
}

.fa-warehouse:before {
    content: "\f494";
}

.fa-watchman-monitoring:before {
    content: "\e087";
}

.fa-water:before {
    content: "\f773";
}

.fa-wave-square:before {
    content: "\f83e";
}

.fa-waze:before {
    content: "\f83f";
}

.fa-weebly:before {
    content: "\f5cc";
}

.fa-weibo:before {
    content: "\f18a";
}

.fa-weight:before {
    content: "\f496";
}

.fa-weight-hanging:before {
    content: "\f5cd";
}

.fa-weixin:before {
    content: "\f1d7";
}

.fa-whatsapp:before {
    content: "\f232";
}

.fa-whatsapp-square:before {
    content: "\f40c";
}

.fa-wheelchair:before {
    content: "\f193";
}

.fa-whmcs:before {
    content: "\f40d";
}

.fa-wifi:before {
    content: "\f1eb";
}

.fa-wikipedia-w:before {
    content: "\f266";
}

.fa-wind:before {
    content: "\f72e";
}

.fa-window-close:before {
    content: "\f410";
}

.fa-window-maximize:before {
    content: "\f2d0";
}

.fa-window-minimize:before {
    content: "\f2d1";
}

.fa-window-restore:before {
    content: "\f2d2";
}

.fa-windows:before {
    content: "\f17a";
}

.fa-wine-bottle:before {
    content: "\f72f";
}

.fa-wine-glass:before {
    content: "\f4e3";
}

.fa-wine-glass-alt:before {
    content: "\f5ce";
}

.fa-wix:before {
    content: "\f5cf";
}

.fa-wizards-of-the-coast:before {
    content: "\f730";
}

.fa-wodu:before {
    content: "\e088";
}

.fa-wolf-pack-battalion:before {
    content: "\f514";
}

.fa-won-sign:before {
    content: "\f159";
}

.fa-wordpress:before {
    content: "\f19a";
}

.fa-wordpress-simple:before {
    content: "\f411";
}

.fa-wpbeginner:before {
    content: "\f297";
}

.fa-wpexplorer:before {
    content: "\f2de";
}

.fa-wpforms:before {
    content: "\f298";
}

.fa-wpressr:before {
    content: "\f3e4";
}

.fa-wrench:before {
    content: "\f0ad";
}

.fa-x-ray:before {
    content: "\f497";
}

.fa-xbox:before {
    content: "\f412";
}

.fa-xing:before {
    content: "\f168";
}

.fa-xing-square:before {
    content: "\f169";
}

.fa-y-combinator:before {
    content: "\f23b";
}

.fa-yahoo:before {
    content: "\f19e";
}

.fa-yammer:before {
    content: "\f840";
}

.fa-yandex:before {
    content: "\f413";
}

.fa-yandex-international:before {
    content: "\f414";
}

.fa-yarn:before {
    content: "\f7e3";
}

.fa-yelp:before {
    content: "\f1e9";
}

.fa-yen-sign:before {
    content: "\f157";
}

.fa-yin-yang:before {
    content: "\f6ad";
}

.fa-yoast:before {
    content: "\f2b1";
}

.fa-youtube:before {
    content: "\f167";
}

.fa-youtube-square:before {
    content: "\f431";
}

.fa-zhihu:before {
    content: "\f63f";
}

.sr-only {
    border: 0;
    clip: rect(0, 0, 0, 0);
    height: 1px;
    margin: -1px;
    overflow: hidden;
    padding: 0;
    position: absolute;
    width: 1px;
}

.sr-only-focusable:active,
.sr-only-focusable:focus {
    clip: auto;
    height: auto;
    margin: 0;
    overflow: visible;
    position: static;
    width: auto;
}"###,
    });
    vec_file.push(crate::FileItem{
            file_name :"web_server_folder/pwa_short_name/css/fa-solid-900.woff2",
            file_content : r###"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"###,
});
    vec_file.push(crate::FileItem {
        file_name: "web_server_folder/pwa_short_name/css/normalize.css",
        file_content: r###"/*! normalize.css v8.0.1 | MIT License | github.com/necolas/normalize.css */


/* Document
   ========================================================================== */


/**
 * 1. Correct the line height in all browsers.
 * 2. Prevent adjustments of font size after orientation changes in iOS.
 */

html {
    line-height: 1.15;
    /* 1 */
    -webkit-text-size-adjust: 100%;
    /* 2 */
}


/* Sections
   ========================================================================== */


/**
 * Remove the margin in all browsers.
 */

body {
    margin: 0;
}


/**
 * Render the `main` element consistently in IE.
 */

main {
    display: block;
}


/**
 * Correct the font size and margin on `h1` elements within `section` and
 * `article` contexts in Chrome, Firefox, and Safari.
 */

h1 {
    font-size: 2em;
    margin: 0.67em 0;
}


/* Grouping content
   ========================================================================== */


/**
 * 1. Add the correct box sizing in Firefox.
 * 2. Show the overflow in Edge and IE.
 */

hr {
    box-sizing: content-box;
    /* 1 */
    height: 0;
    /* 1 */
    overflow: visible;
    /* 2 */
}


/**
 * 1. Correct the inheritance and scaling of font size in all browsers.
 * 2. Correct the odd `em` font sizing in all browsers.
 */

pre {
    font-family: monospace, monospace;
    /* 1 */
    font-size: 1em;
    /* 2 */
}


/* Text-level semantics
   ========================================================================== */


/**
 * Remove the gray background on active links in IE 10.
 */

a {
    background-color: transparent;
}


/**
 * 1. Remove the bottom border in Chrome 57-
 * 2. Add the correct text decoration in Chrome, Edge, IE, Opera, and Safari.
 */

abbr[title] {
    border-bottom: none;
    /* 1 */
    text-decoration: underline;
    /* 2 */
    text-decoration: underline dotted;
    /* 2 */
}


/**
 * Add the correct font weight in Chrome, Edge, and Safari.
 */

b,
strong {
    font-weight: bolder;
}


/**
 * 1. Correct the inheritance and scaling of font size in all browsers.
 * 2. Correct the odd `em` font sizing in all browsers.
 */

code,
kbd,
samp {
    font-family: monospace, monospace;
    /* 1 */
    font-size: 1em;
    /* 2 */
}


/**
 * Add the correct font size in all browsers.
 */

small {
    font-size: 80%;
}


/**
 * Prevent `sub` and `sup` elements from affecting the line height in
 * all browsers.
 */

sub,
sup {
    font-size: 75%;
    line-height: 0;
    position: relative;
    vertical-align: baseline;
}

sub {
    bottom: -0.25em;
}

sup {
    top: -0.5em;
}


/* Embedded content
   ========================================================================== */


/**
 * Remove the border on images inside links in IE 10.
 */

img {
    border-style: none;
}


/* Forms
   ========================================================================== */


/**
 * 1. Change the font styles in all browsers.
 * 2. Remove the margin in Firefox and Safari.
 */

button,
input,
optgroup,
select,
textarea {
    font-family: inherit;
    /* 1 */
    font-size: 100%;
    /* 1 */
    line-height: 1.15;
    /* 1 */
    margin: 0;
    /* 2 */
}


/**
 * Show the overflow in IE.
 * 1. Show the overflow in Edge.
 */

button,
input {
    /* 1 */
    overflow: visible;
}


/**
 * Remove the inheritance of text transform in Edge, Firefox, and IE.
 * 1. Remove the inheritance of text transform in Firefox.
 */

button,
select {
    /* 1 */
    text-transform: none;
}


/**
 * Correct the inability to style clickable types in iOS and Safari.
 */

button,
[type="button"],
[type="reset"],
[type="submit"] {
    -webkit-appearance: button;
}


/**
 * Remove the inner border and padding in Firefox.
 */

button::-moz-focus-inner,
[type="button"]::-moz-focus-inner,
[type="reset"]::-moz-focus-inner,
[type="submit"]::-moz-focus-inner {
    border-style: none;
    padding: 0;
}


/**
 * Restore the focus styles unset by the previous rule.
 */

button:-moz-focusring,
[type="button"]:-moz-focusring,
[type="reset"]:-moz-focusring,
[type="submit"]:-moz-focusring {
    outline: 1px dotted ButtonText;
}


/**
 * Correct the padding in Firefox.
 */

fieldset {
    padding: 0.35em 0.75em 0.625em;
}


/**
 * 1. Correct the text wrapping in Edge and IE.
 * 2. Correct the color inheritance from `fieldset` elements in IE.
 * 3. Remove the padding so developers are not caught out when they zero out
 *    `fieldset` elements in all browsers.
 */

legend {
    box-sizing: border-box;
    /* 1 */
    color: inherit;
    /* 2 */
    display: table;
    /* 1 */
    max-width: 100%;
    /* 1 */
    padding: 0;
    /* 3 */
    white-space: normal;
    /* 1 */
}


/**
 * Add the correct vertical alignment in Chrome, Firefox, and Opera.
 */

progress {
    vertical-align: baseline;
}


/**
 * Remove the default vertical scrollbar in IE 10+.
 */

textarea {
    overflow: auto;
}


/**
 * 1. Add the correct box sizing in IE 10.
 * 2. Remove the padding in IE 10.
 */

[type="checkbox"],
[type="radio"] {
    box-sizing: border-box;
    /* 1 */
    padding: 0;
    /* 2 */
}


/**
 * Correct the cursor style of increment and decrement buttons in Chrome.
 */

[type="number"]::-webkit-inner-spin-button,
[type="number"]::-webkit-outer-spin-button {
    height: auto;
}


/**
 * 1. Correct the odd appearance in Chrome and Safari.
 * 2. Correct the outline style in Safari.
 */

[type="search"] {
    -webkit-appearance: textfield;
    /* 1 */
    outline-offset: -2px;
    /* 2 */
}


/**
 * Remove the inner padding in Chrome and Safari on macOS.
 */

[type="search"]::-webkit-search-decoration {
    -webkit-appearance: none;
}


/**
 * 1. Correct the inability to style clickable types in iOS and Safari.
 * 2. Change font properties to `inherit` in Safari.
 */

::-webkit-file-upload-button {
    -webkit-appearance: button;
    /* 1 */
    font: inherit;
    /* 2 */
}


/* Interactive
   ========================================================================== */


/*
 * Add the correct display in Edge, IE 10+, and Firefox.
 */

details {
    display: block;
}


/*
 * Add the correct display in all browsers.
 */

summary {
    display: list-item;
}


/* Misc
   ========================================================================== */


/**
 * Add the correct display in IE 10+.
 */

template {
    display: none;
}


/**
 * Add the correct display in IE 10.
 */

[hidden] {
    display: none;
}"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "web_server_folder/pwa_short_name/start_service_worker.js",
        file_content: r###"
if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('service_worker.js').then(function (registration) {
        console.log('Registration succeeded.');
    }).catch(function (error) {
        console.log('Registration failed with ' + error);
    });
};
//Listen for claiming of our ServiceWorker
navigator.serviceWorker.addEventListener('controllerchange', function () {
    console.log('Service worker status changed: ', this.controller.state);
    // Listen for changes in the state of our ServiceWorker
    navigator.serviceWorker.controller.addEventListener('statechange', function () {
        // If the ServiceWorker becomes "activated", let the user know they can go offline!
        if (this.state === 'activated') {
            console.log('ServiceWorker activated. Can go offline.');
            window.location.reload();
        }
    });
});
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: ".vscode/settings.json",
        file_content: r###"{
    "workbench.colorCustomizations": {
        "titleBar.activeForeground": "#fff",
        "titleBar.inactiveForeground": "#ffffffcc",
        "titleBar.activeBackground": "#477587",
        "titleBar.inactiveBackground": "#3F758DCC"
      },
    "spellright.language": [
        "en"
    ],
    "spellright.documentTypes": [
        "markdown",
        "latex",
        "plaintext"
    ],
    "rust-analyzer.showUnlinkedFileNotification": false,
    "cSpell.words": [
        "Alla",
        "apos",
        "bestia",
        "bestiadev",
        "bindgen",
        "cdylib",
        "CRDE",
        "endregion",
        "Nazdravlje",
        "onchange",
        "onclick",
        "onhashchange",
        "plantuml",
        "Prost",
        "rustlang",
        "rustprojects",
        "substack",
        "thiserror",
        "webassembly",
        "zdravje"
    ]
}"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "Cargo.toml",
        file_content: r###"[package]
name = "rust_project_name"
version = "0.0.1"
authors = ["project_author"]
homepage = "project_homepage"
edition = "2021"
description = "pwa_description"
repository = "project_repository"
readme = "README.md"
license = "MIT"
# Keyword must be only one word: lowercase letters, hyphens(-) or numbers, less then 35 characters.
keywords = ["maintained", "work-in-progress", "rustlang", "wasm"]
categories = ["wasm"]
publish = false

[lib]
# cdylib is for the wasm module library
crate-type = ["cdylib"]

[dependencies]
# the macro unwrap is great for WASM, because it shows the correct file and line number of the error
unwrap = "1.2.1"
wasm-bindgen = { version = "0.2.86", features = ["serde-serialize"] }
console_error_panic_hook = "0.1.7"
js-sys = "0.3.63"
thiserror="1.0.40"
anyhow="1.0.71"
log = "0.4.17"
wasm-logger = "0.2.0"
wasm-rs-dbg = {version="0.1.2", default-features = false, features = ["console-log"]}
html-escape = "0.2.13"

[dependencies.web-sys]
version = "0.3.63"
features = [
  "AbortController",
  "console",
  "Document",
  "Element",
  "ErrorEvent",
  "HtmlElement",
  "HtmlInputElement",
  "Location",
  "Window",
]

# [dev-dependencies]
# wasm-bindgen-test = "0.3.36"

[profile.release]
panic = "abort"
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/main_mod/lib_mod.rs",
        file_content: r###"// src/lib_mod.rs

//! This module is like a lib.rs module for a binary CLI executable.
//! The `lib_mod.rs` must not contains any input/output interface stuff.
//! So the program logic can be separate from the interface.  

// The `main_mod.rs` contains all input/output interface stuff.
// This `lib_mod.rs` can then be used as dependency crate for other projects.

// The `lib_mod.rs` does not have any real code. All the code is in modules in separate files.
// The `lib_mod.rs` has just the list of modules, it publishes module's functions or class for the caller
// and it has some global stuff like the Error enum.

// The `main_mod.rs` uses the `anyhow` error library.
// The `lib_mod.rs` uses the `thiserror` library.

mod hello_mod;
pub mod web_sys_mod;

// re-exports
pub use hello_mod::format_hello_phrase;
pub use hello_mod::format_upper_hello_phrase;
pub use web_sys_mod as wsm;

/// all possible library errors for `thiserror`
#[derive(thiserror::Error, Debug)]
pub enum LibraryError {
    #[error("Name `{0}` is already uppercase.")]
    Uppercase(String),
    #[error("Unknown error.")]
    Unknown,
}

// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/main_mod/lib_mod/web_sys_mod.rs",
        file_content: r###"// src/web_sys_mod.rs

//! Helper functions for web_sys, window, document, dom, console, html elements,...  
//! Trying to isolate/hide all javascript code and conversion in this module.  

// region: use
// the macro unwrap! shows the TRUE location where the error has ocurred.
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use wasm_bindgen::JsValue;
// use wasm_bindgen_futures::JsFuture;
// use web_sys::console;
// use web_sys::{Request, RequestInit, Response};
// endregion: use

mod html_source_code_mod;
//re-export
pub use html_source_code_mod::HtmlSourceCode;

/// return the global window object  
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// get element by id  
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(window().document());
    unwrap!(document.get_element_by_id(element_id))
}

/// get html element by id  
pub fn get_html_element_by_id(element_id: &str) -> web_sys::HtmlElement {
    let element = get_element_by_id(element_id);
    let html_element: web_sys::HtmlElement = unwrap!(element.dyn_into::<web_sys::HtmlElement>());
    //return
    html_element
}

/// get input element value string by id  
pub fn get_input_element_value_string_by_id(element_id: &str) -> String {
    // debug_write("before get_element_by_id");
    let input_element = get_element_by_id(element_id);
    // debug_write("before dyn_into");
    let input_html_element = unwrap!(input_element.dyn_into::<web_sys::HtmlInputElement>());
    // debug_write("before value()");
    input_html_element.value()
}

/// add event listener for button  
pub fn add_listener_to_button(element_id: &str, fn_on_click_button: &'static (dyn Fn() + 'static)) {
    let handler_1 = Box::new(move || {
        fn_on_click_button();
    }) as Box<dyn FnMut()>;
    let closure = Closure::wrap(handler_1);

    let html_element = get_html_element_by_id(element_id);
    html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

/// add event listener for onhashchange  
pub fn add_listener_for_onhashchange(fn_on_hash_change: &'static (dyn Fn() + 'static)) {
    let handler_1 = Box::new(move || {
        fn_on_hash_change();
    }) as Box<dyn FnMut()>;
    let closure = Closure::wrap(handler_1);

    window().set_onhashchange(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

/// set inner text  
pub fn set_html_element_inner_text(element_id: &str, inner_text: &str) {
    let html_element = get_html_element_by_id(element_id);
    html_element.set_inner_text(inner_text);
}

/// open URL in same tab (PWA don't have tabs, only one windows)  
pub fn open_url(url: &str) {
    dbg!(url);
    window().location().assign(url).unwrap();
    // Strange behavior: if url has hash, then it does not load ?!?
    match window().location().hash() {
        Ok(hash) => {
            dbg!(&hash);
            window().location().set_hash(&hash).unwrap();
        }
        Err(_err) => {}
    }
}

/// Wasm must read time from javascript.  
pub fn now_time_as_string() -> String {
    let now = js_sys::Date::new_0();
    let now_time = format!(
        "{:02}:{:02}:{:02}",
        now.get_hours(),
        now.get_minutes(),
        now.get_seconds()
    );
    now_time
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/main_mod/lib_mod/web_sys_mod/html_source_code_mod.rs",
        file_content: r###"//! html_source_code_mod.rs

/// HtmlSourceCode - type to manipulate HTML source code safer than with string functions only  
/// WARNING for HTML INJECTION!   
/// HTML is the standard markup language for Web pages. HTML source code is just a text.  
/// It is easy to read, write, understand and parse.  
/// The syntax of HTML source code is similar to XML structured with elements, tags, nodes, texts, comments and attributes.  
/// The browser then transforms this HTML source code into the DOM tree and then renders that.  
/// It is very tempting to modify this source code in our application with string manipulation and then pass it to the browser.  
/// The html source code (it is just a string) that is provided by the programmer is always ok, he wants it to work properly.  
/// The BIG problem arises when we need to inject some user provided data into the HTML source code.  
/// The HTML syntax mixes instructions and data together and this creates a BIG problem.  
/// Never put user provided strings in a html source code directly, because it can contain an HTML injection attack.  
/// We need to encode all user data before putting it into the HTML source code.  
/// There are 2 types of encodings: one for attributes values and another for text nodes.  
/// We will create a new type that makes it safer and easier for the programmer to replace data in the HTML source code.  
///

pub struct HtmlSourceCode {
    html: String,
}

impl HtmlSourceCode {
    /// The programmer provides a &'static str to initiate HtmlSourceCode.  
    /// The html source code coming from the programmer is always ok, he wants it to work properly.  
    /// The data that will be replaced, have a recognizable, unique and delimited value.  
    pub fn new(html_code: &'static str) -> Self {
        HtmlSourceCode {
            html: html_code.to_string(),
        }
    }

    /// get the well formed html  
    /// We trust the programmer to carefully work with HtmlSourceCode to be always well formed and without HTML injection.  
    pub fn get_html(&self) -> String {
        self.html.clone()
    }

    /// This must be pure text, no html element are allowed for bold or italic...  
    /// We trust the programmer that it will replace only the anticipated placeholders.  
    pub fn replace_text_node(&mut self, placeholder: &'static str, text: &str) {
        self.html = self.html.replace(placeholder, &html_escape::encode_text(text));
    }

    /// The attribute value must be double_quoted.  
    /// We trust the programmer that it will replace only the anticipated placeholders.  
    pub fn replace_attribute_value(&mut self, placeholder: &'static str, value: &str) {
        self.html = self
            .html
            .replace(placeholder, &html_escape::encode_double_quoted_attribute(value));
    }

    /// We expect the HtmlSourceCode to be well formed. For that we trust the programmer.  
    /// We trust the programmer that it will replace only the anticipated placeholders.  
    pub fn replace_html_source_code(&mut self, placeholder: &'static str, html_source_code: &HtmlSourceCode) {
        self.html = self.html.replace(placeholder, &html_source_code.get_html());
    }

    /// Injects the HTMLSourceCode into a DOM element.  
    /// We trust the programmer to carefully work with HtmlSourceCode to be always well formed and without HTML injection.  
    pub fn inject_into_dom_element(self, element_id: &str) {
        let html_element = super::get_element_by_id(element_id);
        html_element.set_inner_html(&self.html);
    }
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/main_mod/lib_mod/hello_mod.rs",
        file_content: r###"// src/hello_mod.rs

//! All the real code (program logic) is inside modules in separate files.
//! This module are UI agnostic and must not have anything to do with UI.
//! So the same library could be used for CLI and for WASM, that have vastly different UI.

/// format the hello phrase
pub fn format_hello_phrase(greet_name: &str) -> String {
    log::info!("start format_hello_phrase()");
    // return
    format!("Hello {}!", greet_name)
}

/// format the hello phrase with uppercase name
/// if it is already uppercase, return error with thiserror
pub fn format_upper_hello_phrase(greet_name: &str) -> Result<String, crate::LibraryError> {
    log::info!("start format_upper_hello_phrase()");
    // shadowing the same variable name:
    let upper_greet_name = make_uppercase(greet_name);
    if upper_greet_name == greet_name {
        return Err(crate::LibraryError::Uppercase(greet_name.to_string()));
    }

    // return
    Ok(format!("Hello {}!", &upper_greet_name))
}

/// return uppercase
pub fn make_uppercase(greet_name: &str) -> String {
    // return
    greet_name.to_uppercase()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_format_upper_hello_phrase() {
        assert_eq!(format_upper_hello_phrase("abcd").expect("error"), "Hello ABCD!");
        assert!(format_upper_hello_phrase("ABCD").is_err());
    }

    #[test]
    pub fn test_make_uppercase() {
        assert_eq!(make_uppercase("abcd"), "ABCD");
        assert_eq!(make_uppercase("1234abcd"), "1234ABCD");
        assert_eq!(make_uppercase("čšž"), "ČŠŽ");
    }
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/main_mod.rs",
        file_content: r###"// src/main_mod.rs

//! This module is like a main.rs module for a binary CLI executable.  
//! The `main_mod.rs` contains all input/output interface stuff.  
//! So the program logic can be separate from the interface.  

// The `lib_mod.rs` must not contains any input/output interface stuff.
// This `lib_mod.rs` can then be used as dependency crate for other projects.

// The `main_mod.rs` uses the `anyhow` error library.
// The `lib_mod.rs` uses the `thiserror` library.

use unwrap::unwrap;
use wasm_rs_dbg::dbg;

mod lib_mod;
use lib_mod::wsm;
pub use lib_mod::LibraryError;

/// entry point just like for cli-bin-executable
pub fn main() {
    // logging is essential for every project
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("main() started");

    let args = get_args_from_hash_fragment();
    routing_by_arguments(args);
}

/// get args from hash fragment
fn get_args_from_hash_fragment() -> Vec<String> {
    // region: In browser we can use 'local routing' on url path with # fragment
    // but sometimes it does not reload the page, because the browser thinks # is an anchor on the same page
    // So we need to add a listener also to this other event.
    // http://localhost:4000/pwa_short_name/#arg_1/arg_2
    let location = wsm::window().location();
    let mut location_hash_fragment = unwrap!(location.hash());
    // the hash is not decoded automatically !
    // dbg! is now writing to the console, crate wasm-rs-dbg
    dbg!(&location_hash_fragment);
    dbg!(&wsm::now_time_as_string());

    // in std::env::args() the nth(0) is the exe name. Let's make it similar.
    if !location_hash_fragment.is_empty() {
        // replace # with delimiter /
        location_hash_fragment.replace_range(..1, "/");
    }
    let location_hash_fragment = format!("pwa_short_name{}", location_hash_fragment);
    dbg!(&location_hash_fragment);
    let args = location_hash_fragment.split("/");
    let args: Vec<String> = args.map(|x| x.to_string()).collect();
    dbg!(&args);
    args
}

/// routing by arguments  
/// routing can come from:  
/// 1. on page load and then read the window().location()  
/// 2. or from event change_hash  
/// 3. or can be called from a wasm function directly  
fn routing_by_arguments(args: Vec<String>) {
    // every page must have the header and onhashchange
    wsm::add_listener_for_onhashchange(&on_hash_change);
    header();
    // endregion

    // transforming Vec<String> to Vec<&str>, because we need that in the match expression
    let args: Vec<&str> = args.iter().map(|s| s as &str).collect();

    // super simple argument parsing.
    match args.get(1).copied() {
        None => page_with_inputs(),
        Some("page_with_inputs") => page_with_inputs(),
        Some("help") => print_help(),
        Some("print") => {
            match args.get(2).copied() {
                // second argument
                Some(greet_name) => print_greet_name(greet_name),
                None => wsm::set_html_element_inner_text("div_for_errors", "Error: Missing second argument for print."),
            }
        }
        Some("upper") => {
            match args.get(2).copied() {
                // second argument
                Some(greet_name) => {
                    // this can return an error. Here is the last place I can deal with the error.
                    match upper_greet_name(greet_name) {
                        // do nothing
                        Ok(()) => (),
                        // log error from anyhow
                        Err(err) => wsm::set_html_element_inner_text("div_for_errors", &format!("Error: {err}")),
                    }
                }
                None => wsm::set_html_element_inner_text("div_for_errors", "Error: Missing second argument for upper."),
            }
        }
        _ => wsm::set_html_element_inner_text(
            "div_for_errors",
            "Error: Unrecognized arguments. Try \n http://localhost:4000/pwa_short_name/#help",
        ),
    }
}

/// the listener calls this function  
fn on_hash_change() {
    dbg!("on_hash_change");
    let args = get_args_from_hash_fragment();
    routing_by_arguments(args);
}

/// render header with Home and Help  
fn header() {
    let html_source_code = wsm::HtmlSourceCode::new(
        r#"
<div class="div_header">
    <a href="/pwa_short_name/#page_with_inputs"><span class="fa-solid fa-home"></span>Home</a>
    &nbsp;
    <a href="/pwa_short_name/#help"><span class="fa-solid fa-question-circle"></span>Help</a>
    &nbsp;
</div>
<div>&nbsp;</div>
<div id="div_body"></div>
"#,
    );
    html_source_code.inject_into_dom_element("div_for_wasm_html_injecting");
}

/// print help  
fn print_help() {
    wsm::set_html_element_inner_text(
        "div_body",
        r#"Welcome to pwa_short_name !

This is a simple yet complete template for a PWA WASM program written in Rust.
The file structure is on purpose similar to a Rust CLI project and accepts similar arguments.

http://localhost:4000/pwa_short_name/
http://localhost:4000/pwa_short_name/#help
http://localhost:4000/pwa_short_name/#print/world
http://localhost:4000/pwa_short_name/#upper/world

This command should return an error:
http://localhost:4000/pwa_short_name/#upper/WORLD

© 2024 bestia.dev  MIT License github.com/bestia-dev/cargo-auto
"#,
    );
}

/// render first page  
fn page_with_inputs() {
    // rust has `Raw string literals` that are great!
    // just add r# before the starting double quotes and # after the ending double quotes.
    let mut html_source_code = wsm::HtmlSourceCode::new(
        r#"<h1>pwa_short_name</h1>
<p>Write a command in the Argument 1: print or upper</p>
<div class="input-wrap">
    <label for="arg_1">Argument 1:</label>  
    <input style="width:20%;" type="text" id="arg_1" value="{ph_arg_1}"/>
</div>
<p>Write a name in the Argument 2: world or WORLD</p>
<div class="input-wrap">
    <label for="arg_2">Argument 2:</label>  
    <input style="width:20%;" type="text" id="arg_2" value="{ph_arg_2}"/>
</div>
<p>Click on Run</p>
<div class="input-wrap">
    <input type="button" class="button" id="btn_run" value="Run"/>
</div>
{ph_elem_p_1}
        "#,
    );

    // {ph_...} is the prefix for placeholder to make the string unique and distinctive
    html_source_code.replace_attribute_value("{ph_arg_1}", "upper");
    html_source_code.replace_attribute_value("{ph_arg_2}", "world");

    let mut fragment = wsm::HtmlSourceCode::new(r#"<p class="{ph_attr_class_1}">{ph_text_node_1}</p>"#);
    fragment.replace_attribute_value("{ph_attr_class_1}", "small");
    fragment.replace_text_node("{ph_text_node_1}", "bestia.dev");
    html_source_code.replace_html_source_code("{ph_elem_p_1}", &fragment);

    dbg!(html_source_code.get_html());
    html_source_code.inject_into_dom_element("div_body");
    wsm::add_listener_to_button("btn_run", &on_click_btn_run);
}

/// the listener calls this function  
fn on_click_btn_run() {
    let arg_1 = wsm::get_input_element_value_string_by_id("arg_1");
    let arg_2 = wsm::get_input_element_value_string_by_id("arg_2");
    if !arg_1.is_empty() && !arg_2.is_empty() {
        // pass arguments as URL in a new tab
        let url = format!("/pwa_short_name/#{arg_1}/{arg_2}");
        wsm::open_url(&url);
    } else {
        // write on the same web page
        wsm::set_html_element_inner_text("div_for_errors", &format!("Error: Both arguments are mandatory."));
    }
}

/// print my name  
fn print_greet_name(greet_name: &str) {
    wsm::set_html_element_inner_text(
        "div_body",
        &format!(
            r#"The result is
{}
"#,
            lib_mod::format_hello_phrase(greet_name)
        ),
    );
}

/// print my name upper, can return error  
fn upper_greet_name(greet_name: &str) -> anyhow::Result<()> {
    // the function from `lib.rs`, can return error
    // use the ? syntax to bubble the error up one level or continue (early return)
    let upper = lib_mod::format_upper_hello_phrase(greet_name)?;
    wsm::set_html_element_inner_text(
        "div_body",
        &format!(
            r#"The result is
{upper}
"#
        ),
    );
    // return
    Ok(())
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "src/lib.rs",
        file_content: r###"// src/lib.rs
// This file has just the wasm_bindgen_start() function
// and calls into main_mod.rs.
// So the structure of the project modules can be similar to a binary CLI executable.

#![doc=include_str!("../README.md")]

use wasm_bindgen::prelude::*;

mod main_mod;
/// LibraryError must be accessible in every module.
pub use main_mod::LibraryError;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    dbg!("pwa_short_name v{}", env!("CARGO_PKG_VERSION"));

    main_mod::main();
    // return
    Ok(())
}
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: ".github/workflows/rust_fmt_auto_build_test.yml",
        file_content: r###"name: rust_fmt_auto_build_test

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  rust_fmt_auto_build_test:

    runs-on: ubuntu-latest

    steps:
    - name: checkout
      uses: actions/checkout@v4

    - name: cargo fmt -- --check
      run: cargo fmt -- --check

    - name: Run cache for rust dependencies
      uses: Swatinem/rust-cache@v2.7.3

    - name: Configure sccache
      run: echo "RUSTC_WRAPPER=sccache" >> $GITHUB_ENV; echo "SCCACHE_GHA_ENABLED=true" >> $GITHUB_ENV

    - name: Run sccache-cache for artifacts
      uses: mozilla-actions/sccache-action@v0.0.4

    - name: install and cache cargo-auto
      uses: baptiste0928/cargo-install@v3.0.0
      with:
        crate: cargo-auto

    - name: Cache for automation tasks
      uses: actions/cache@v4.0.0
      with:
        path: |
          /home/runner/work/cargo-auto/cargo-auto/automation_tasks_rs/.file_hashes.json 
          /home/runner/work/cargo-auto/cargo-auto/automation_tasks_rs/target 
          /home/runner/work/cargo-auto/cargo-auto/automation_tasks_rs/Cargo.toml
        key: automation_tasks_rs

    - name: cargo auto build
      run: cargo auto build

    - name: cargo auto test
      run: cargo auto test
      
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: ".github/workflows/docs_pages.yml",
        file_content: r###"# Simple workflow for deploying static content to GitHub Pages
name: docs_pages

on:
  # Runs on pushes targeting the default branch
  push:
    branches: ["main"]

  # Allows you to run this workflow manually from the Actions tab
  workflow_dispatch:

# Sets permissions of the GITHUB_TOKEN to allow deployment to GitHub Pages
permissions:
  contents: read
  pages: write
  id-token: write

# Allow only one concurrent deployment, skipping runs queued between the run in-progress and latest queued.
# However, do NOT cancel in-progress runs as we want to allow these production deployments to complete.
concurrency:
  group: "pages"
  cancel-in-progress: false

jobs:
  # Single deploy job since we're just deploying
  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4
      - name: Setup Pages
        uses: actions/configure-pages@v4
      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          # Upload entire repository
          path: 'docs'
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: ".gitattributes",
        file_content: r###"# Specific git config for the project

# Declare files that will always have LF line endings on checkout.
*.rs text eol=lf
*.toml text eol=lf
*.md text eol=lf
*.json text eol=lf
*.json5 text eol=lf
*.lock text eol=lf
*.yml text eol=lf
*.html text eol=lf
*.js text eol=lf
*.css text eol=lf
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: ".gitignore",
        file_content: r###"# Generated by Cargo
# will have compiled files and executables
/target/

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
# Cargo.lock

# These are backup files generated by rustfmt
**/*.rs.bk

# result of compilation does not need to go to repository
/pkg/

# not needed in commits, but also not a problem if they are committed
/.automation_tasks_rs_file_hashes.json
/.auto_version_from_date.json
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: ".automation_tasks_rs_file_hashes.json",
        file_content: r###"{
  "vec_file_metadata": [
    {
      "filename": "Cargo.toml",
      "filehash": "f0b1dbe0670ef3bc20d8975a4f592fa1388bb70b8c7695494d6dcb8890571927"
    },
    {
      "filename": "src/main_mod/lib_mod.rs",
      "filehash": "5301f1e8f18591606e82534667a4c7e1bfd5328f354dd966d328226578cac61e"
    },
    {
      "filename": "src/main_mod/lib_mod/web_sys_mod.rs",
      "filehash": "a3d14936b65035e81c52fa255bfc7b3d83e9c5de8218544753d6202526fb3120"
    },
    {
      "filename": "src/main_mod/lib_mod/web_sys_mod/html_source_code_mod.rs",
      "filehash": "f737932003b48996650ae2c7caf54162c302c1613b5ee69adef4e04a207a6d8e"
    },
    {
      "filename": "src/main_mod/lib_mod/hello_mod.rs",
      "filehash": "cefd0a1665590fcda307dd12ab212598f7c6cab7fc5e33baac37128ab6d013bd"
    },
    {
      "filename": "src/main_mod.rs",
      "filehash": "6805b2f551bf74a36f436864706ca661c5557356abe10803a2411e6ab8b2950f"
    },
    {
      "filename": "src/lib.rs",
      "filehash": "42949bd6eeee8e5143e43aa91e47116f1e2814d2f0da499c5b45c16485272a5f"
    }
  ]
}"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/.vscode/settings.json",
        file_content: r###"{
    "workbench.colorCustomizations": {
        "titleBar.activeForeground": "#fff",
        "titleBar.inactiveForeground": "#ffffffcc",
        "titleBar.activeBackground": "#404040",
        "titleBar.inactiveBackground": "#2d2d2dcc"
    },
    "spellright.language": [
        "en"
    ],
    "spellright.documentTypes": [
        "markdown",
        "latex",
        "plaintext"
    ],
    "rust-analyzer.showUnlinkedFileNotification": false,
    "cSpell.words": [
        "bestia",
        "endregion",
        "plantuml",
        "rustdevuser",
        "rustprojects",
        "zcvf"
    ]
}"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/Cargo.toml",
        file_content: r###"[package]
name = "automation_tasks_rs"
version = "1.0.0"
authors = ["bestia.dev"]
homepage = "https://bestia.dev"
edition = "2021"
description = "cargo auto - automation tasks written in Rust language"
publish = false

[dependencies]
cargo_auto_lib = "1.3.33""###,
    });
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/src/main.rs",
        file_content: r###"// automation_tasks_rs for rust_project_name

// region: library with basic automation tasks
use cargo_auto_lib as cl;
// traits must be in scope (Rust strangeness)
use cl::CargoTomlPublicApiMethods;

use cargo_auto_lib::GREEN;
use cargo_auto_lib::RED;
use cargo_auto_lib::RESET;
use cargo_auto_lib::YELLOW;

// region: library with basic automation tasks

fn main() {
    cl::exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("{YELLOW}Running automation task: {task}{RESET}");
                if &task == "build" {
                    task_build();
                } else if &task == "release" {
                    task_release();
                } else if &task == "doc" {
                    task_doc();
                } else if &task == "test" {
                    task_test();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else if &task == "publish_to_web" {
                    task_publish_to_web();
                } else if &task == "github_new_release" {
                    task_github_new_release();
                } else {
                    eprintln!("{RED}Error: Task {task} is unknown.{RESET}");
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
    {YELLOW}Welcome to cargo-auto !{RESET}
    {YELLOW}This program automates your custom tasks when developing a Rust project.{RESET}

    {YELLOW}User defined tasks in automation_tasks_rs:{RESET}
{GREEN}cargo auto build{RESET} - {YELLOW}builds the crate in debug mode, fmt, increment version{RESET}
{GREEN}cargo auto release{RESET} - {YELLOW}builds the crate in release mode, fmt, increment version{RESET}
{GREEN}cargo auto doc{RESET} - {YELLOW}builds the docs, copy to docs directory{RESET}
{GREEN}cargo auto test{RESET} - {YELLOW}runs all the tests{RESET}
{GREEN}cargo auto commit_and_push "message"{RESET} - {YELLOW}commits with message and push with mandatory message{RESET}
    {YELLOW}It is preferred to use SSH for git push to GitHub.{RESET}
    {YELLOW}<https://github.com/bestia-dev/docker_rust_development/blob/main/ssh_easy.md>{YELLOW}
    {YELLOW}On the very first commit, this task will initialize a new local git repository and create a remote GitHub repo.{RESET}
    {YELLOW}In that case the task needs the Personal Access Token Classic from <https://github.com/settings/tokens>{RESET}
{GREEN}cargo auto publish_to_web - publish to web, git tag{RESET}
    {YELLOW}It is preferred to use SSH to publish to web and remotely manage the web server.{RESET}
    {YELLOW}<https://github.com/bestia-dev/docker_rust_development/blob/main/ssh_easy.md>{YELLOW}
{GREEN}cargo auto github_new_release{RESET} - {YELLOW}creates new release on github{RESET}
    {YELLOW}This task needs the Personal Access Token Classic from <https://github.com/settings/tokens>{RESET}

    {YELLOW}© 2024 bestia.dev  MIT License github.com/bestia-dev/cargo-auto{RESET}
"#
    );
    print_examples_cmd();
}

/// all example commands in one place
fn print_examples_cmd() {
    /*
        println!(r#"{YELLOW}run examples:{RESET}
    {GREEN}cargo run --example example1{RESET}
    "#);
    */
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "release", "doc", "test", "commit_and_push", "publish_to_web", "github_new_release",];
        cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
       cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// wasm-pack build
fn task_build() {
    // let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::run_shell_command("cargo fmt");
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");
    cl::run_shell_command("wasm-pack build --target web");
    cl::run_shell_command("\\rsync -a --delete-after pkg/ web_server_folder/pwa_short_name/pkg/");
    println!(
        r#"
    {YELLOW}After `cargo auto build`, open port 4000 in VSCode and run the basic web server{RESET}
    {YELLOW}in a separate VSCode bash terminal, so it can serve constantly in the background.{RESET}
{GREEN}basic-http-server -a 0.0.0.0:4000 ./web_server_folder{RESET}
    {YELLOW}and open the browser on{RESET}
{GREEN}http://localhost:4000/pwa_short_name/{RESET}
{GREEN}http://localhost:4000/pwa_short_name/#print/world{RESET}
{GREEN}http://localhost:4000/pwa_short_name/#upper/world{RESET}
    {YELLOW}This will return an error:{RESET}
{GREEN}http://localhost:4000/pwa_short_name/#upper/WORLD{RESET}
    {YELLOW}If all is fine, run{RESET}
{GREEN}cargo auto release{RESET}
"#
    );
}

/// wasm-pack build --release
fn task_release() {
    // let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");

    cl::run_shell_command("cargo fmt");
    cl::run_shell_command("wasm-pack build --target web");
    cl::run_shell_command("\\rsync -a --delete-after pkg/ web_server_folder/pwa_short_name/pkg/");
    println!(
        r#"
    {YELLOW}After `cargo auto build`, open port 4000 in VSCode and run the basic web server{RESET}
    {YELLOW}in a separate VSCode bash terminal, so it can serve constantly in the background.{RESET}
{GREEN}basic-http-server -a 0.0.0.0:4000 ./web_server_folder{RESET}
    {YELLOW}and open the browser on{RESET}
{GREEN}http://localhost:4000/pwa_short_name/{RESET}    
{GREEN}http://localhost:4000/pwa_short_name/#print/world{RESET}
{GREEN}http://localhost:4000/pwa_short_name/#upper/world{RESET}
    {YELLOW}This will return an error:{RESET}
{GREEN}http://localhost:4000/pwa_short_name/#upper/WORLD{RESET}
    {YELLOW}If all is fine, run{RESET}
{GREEN}cargo auto doc{RESET}
"#
    );
    print_examples_cmd();
}

/// cargo doc, then copies to /docs/ folder, because this is a github standard folder
fn task_doc() {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");
    cl::auto_plantuml(&cargo_toml.package_repository().unwrap());
    cl::auto_playground_run_code();
    cl::auto_md_to_doc_comments();

    cl::run_shell_command("cargo doc --no-deps --document-private-items");
    // copy target/doc into docs/ because it is github standard
    cl::run_shell_command("rsync -a --info=progress2 --delete-after target/doc/ docs/");
    // Create simple index.html file in docs directory
    cl::run_shell_command(&format!(
        r#"echo "<meta http-equiv=\"refresh\" content=\"0; url={}/index.html\" />" > docs/index.html"#,
        cargo_toml.package_name().replace("-", "_")
    ));
    // pretty html
    cl::auto_doc_tidy_html().unwrap();
    cl::run_shell_command("cargo fmt");
    // message to help user with next move
    println!(
        r#"
    {YELLOW}After `cargo auto doc`, check `docs/index.html`. If ok then test the documentation code examples{RESET}
{GREEN}cargo auto test{RESET}
"#
    );
}

/// cargo test
fn task_test() {
    println!(r#"    {YELLOW}Wasm is a cdylib and therefore doc-tests are not run !{RESET}"#);
    cl::run_shell_command("cargo test");
    println!(
        r#"
    {YELLOW}After `cargo auto test`. If ok then {RESET}
{GREEN}cargo auto commit_and_push "message"{RESET}
    {YELLOW}with mandatory commit message{RESET}
"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    let Some(message) = arg_2 else {
        eprintln!("{RED}Error: Message for commit is mandatory. Exiting.{RESET}");
        // early exit
        return;
    };

    // if description or topics/keywords/tags have changed
    cl::description_and_topics_to_github();

    // init repository if needed. If it is not init then normal commit and push.
    if !cl::init_repository_if_needed(&message) {
        // separate commit for docs if they changed, to not make a lot of noise in the real commit
        if std::path::Path::new("docs").exists() {
            cl::run_shell_command(r#"git add docs && git diff --staged --quiet || git commit -m "update docs" "#);
        }
        cl::add_message_to_unreleased(&message);
        // the real commit of code
        cl::run_shell_command(&format!( r#"git add -A && git diff --staged --quiet || git commit -m "{message}" "#));
        cl::run_shell_command("git push");
        println!(
r#"
    {YELLOW}After `cargo auto commit_and_push "message"`{RESET}
{GREEN}cargo auto publish_to_crates_io{RESET}
"#
        );
    }
}

/// publish to web
fn task_publish_to_web() {
    let cargo_toml = cl::CargoToml::read();
    // take care of tags
    let tag_name_version = cl::git_tag_sync_check_create_push(&version);

    // Find the filename of the identity_file for ssh connection to host_name, to find out if need ssh-add or not.
    // parse the ~/.ssh/config. 99% probably there should be a record for host_name and there is the identity_file.
    // else ask user for filename, then run ssh-add
    cl::ssh_add_resolve("project_homepage","bestia_dev_ssh_1");

    // rsync to copy to server over ssh
    let shell_command = format!(
        r#"rsync -e ssh -a --info=progress2 --delete-after ~/rustprojects/{package_name}/web_server_folder/ project_author@project_homepage:/var/www/project_homepage/pwa_short_name/"#,
        package_name = cargo_toml.package_name()
    );
    cl::run_shell_command(&shell_command);

    println!(
        r#"{YELLOW}
    After `cargo auto publish_to_web`, 
    check 
https://bestia.dev/{package_name}
{RESET}"#,
        package_name = cargo_toml.package_name()
    );
}

/// create a new release on github
fn task_github_new_release() {
    let cargo_toml = cl::CargoToml::read();
    let version = cargo_toml.package_version();
    // take care of tags
    let tag_name_version = cl::git_tag_sync_check_create_push(&version);

    let owner = cargo_toml.github_owner().unwrap();
    let repo_name = cargo_toml.package_name();
    let now_date = cl::now_utc_date_iso();
    let release_name = format!("Version {} ({})", &version, now_date);
    let branch = "main";

    // First, the user must write the content into file RELEASES.md in the section ## Unreleased.
    // Then the automation task will copy the content to GitHub release
    // and create a new Version title in RELEASES.md.
    let body_md_text = cl::body_text_from_releases_md(&release_name).unwrap();

    let _release_id = cl::github_api_create_new_release(
        &owner,
        &repo_name,
        &tag_name_version,
        &release_name,
        branch,
        &body_md_text,
    );

    println!(
        "
    {YELLOW}New GitHub release created: {release_name}.{RESET}
"
    );

    /*
        // region: upload asset only for executables, not for libraries
        println!("
        {YELLOW}Now uploading release asset. This can take some time if the files are big. Wait...{RESET}
    ");
        // compress files tar.gz
        let tar_name = format!("{repo_name}-{tag_name_version}-x86_64-unknown-linux-gnu.tar.gz");
        cl::run_shell_command(&format!("tar -zcvf {tar_name} target/release/{repo_name}"));

        // upload asset
        cl::github_api_upload_asset_to_release(&owner, &repo_name, &release_id, &tar_name).await;
        cl::run_shell_command(&format!("rm {tar_name}"));

        println!("
        {YELLOW}Asset uploaded. Open and edit the description on GitHub Releases in the browser.{RESET}
    ");
        // endregion: upload asset only for executables, not for libraries

        */
    println!(
        "
{GREEN}https://github.com/{owner}/{repo_name}/releases{RESET}
    "
    );
}
// endregion: tasks"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/.gitignore",
        file_content: r###"/target

# not needed in commits, but also not a problem if they are committed
/.file_hashes.json
"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/.file_hashes.json",
        file_content: r###"{
  "vec_file_metadata": [
    {
      "filename": "automation_tasks_rs/Cargo.toml",
      "filehash": "55eef2dc82f193e7fcdf566b0c5214166c70e682cb0614b10a5e8fdd719d172b"
    },
    {
      "filename": "automation_tasks_rs/target/debug/automation_tasks_rs",
      "filehash": "1dc66793c865a408dd30f99611f4496f25d8a08c0643904a17c38d6a1ffdab11"
    },
    {
      "filename": "automation_tasks_rs/src/main.rs",
      "filehash": "2bca6dd799543322f8ba0a8a61feb92213a411a567ff8469ab20afe7ebfba35f"
    }
  ]
}"###,
    });
    vec_file.push(crate::FileItem {
        file_name: "automation_tasks_rs/Cargo.lock",
        file_content: r###"# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 3

[[package]]
name = "addr2line"
version = "0.21.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8a30b2e23b9e17a9f90641c7ab1549cd9b44f296d3ccbf309d2863cfe398a0cb"
dependencies = [
 "gimli",
]

[[package]]
name = "adler"
version = "1.0.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f26201604c87b1e01bd3d98f8d5d9a8fcbb815e8cedb41ffccbeb4bf593a35fe"

[[package]]
name = "adler32"
version = "1.2.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "aae1277d39aeec15cb388266ecc24b11c80469deae6067e17a1a7aa9e5c1f234"

[[package]]
name = "aho-corasick"
version = "1.1.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b2969dcb958b36655471fc61f7e416fa76033bdd4bfed0678d8fee1e2d07a1f0"
dependencies = [
 "memchr",
]

[[package]]
name = "android-tzdata"
version = "0.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e999941b234f3131b00bc13c22d06e8c5ff726d1b6318ac7eb276997bbb4fef0"

[[package]]
name = "android_system_properties"
version = "0.1.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "819e7219dbd41043ac279b19830f2efc897156490d7fd6ea916720117ee66311"
dependencies = [
 "libc",
]

[[package]]
name = "anyhow"
version = "1.0.79"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "080e9890a082662b09c1ad45f567faeeb47f22b5fb23895fbe1e651e718e25ca"

[[package]]
name = "arrayref"
version = "0.3.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6b4930d2cb77ce62f89ee5d5289b4ac049559b1c45539271f5ed4fdc7db34545"

[[package]]
name = "autocfg"
version = "1.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d468802bab17cbc0cc575e9b053f41e72aa36bfa6b7f55e3529ffa43161b97fa"

[[package]]
name = "automation_tasks_rs"
version = "1.0.1"
dependencies = [
 "cargo_auto_lib",
 "pretty_dbg",
]

[[package]]
name = "backtrace"
version = "0.3.69"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2089b7e3f35b9dd2d0ed921ead4f6d318c27680d4a5bd167b3ee120edb105837"
dependencies = [
 "addr2line",
 "cc",
 "cfg-if 1.0.0",
 "libc",
 "miniz_oxide",
 "object",
 "rustc-demangle",
]

[[package]]
name = "base64"
version = "0.21.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9d297deb1925b89f2ccc13d7635fa0714f12c87adce1c75356b39ca9b7178567"

[[package]]
name = "base64ct"
version = "1.6.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8c3c1a368f70d6cf7302d78f8f7093da241fb8e8807c05cc9e51a125895a6d5b"

[[package]]
name = "bitflags"
version = "1.3.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bef38d45163c2f1dde094a7dfd33ccf595c92905c8f8f4fdc18d06fb1037718a"

[[package]]
name = "bitflags"
version = "2.4.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ed570934406eb16438a4e976b1b4500774099c13b8cb96eec99f620f05090ddf"

[[package]]
name = "block-buffer"
version = "0.10.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3078c7629b62d3f0439517fa394996acacc5cbc91c5a20d8c658e77abd503a71"
dependencies = [
 "generic-array",
]

[[package]]
name = "bumpalo"
version = "3.14.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7f30e7476521f6f8af1a1c4c0b8cc94f0bee37d91763d0ca2665f299b6cd8aec"

[[package]]
name = "bytes"
version = "1.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a2bd12c1caf447e69cd4528f47f94d203fd2582878ecb9e9465484c4148a8223"

[[package]]
name = "cargo_auto_lib"
version = "1.2.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "94d92d8df6e9898e1032585e15804484d2eaeddd777679b285551e24bb16b1be"
dependencies = [
 "anyhow",
 "base64ct",
 "cargo_toml",
 "chrono",
 "data-encoding",
 "deflate",
 "filetime",
 "glob",
 "home",
 "inquire",
 "lazy_static",
 "pretty_dbg",
 "radix64",
 "reader_for_microxml",
 "regex",
 "reqwest",
 "ring",
 "semver",
 "serde",
 "serde_derive",
 "serde_json",
 "sha2",
 "ssh2-config",
 "termion",
 "thiserror",
 "tokio",
 "tokio-util",
 "toml",
 "url",
]

[[package]]
name = "cargo_toml"
version = "0.19.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3dc9f7a067415ab5058020f04c60ec7b557084dbec0e021217bbabc7a8d38d14"
dependencies = [
 "serde",
 "toml",
]

[[package]]
name = "cc"
version = "1.0.83"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f1174fb0b6ec23863f8b971027804a42614e347eafb0a95bf0b12cdae21fc4d0"
dependencies = [
 "libc",
]

[[package]]
name = "cfg-if"
version = "0.1.10"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4785bdd1c96b2a846b2bd7cc02e86b6b3dbf14e7e53446c4f54c92a361040822"

[[package]]
name = "cfg-if"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "baf1de4339761588bc0619e3cbc0120ee582ebb74b53b4efbf79117bd2da40fd"

[[package]]
name = "chrono"
version = "0.4.33"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9f13690e35a5e4ace198e7beea2895d29f3a9cc55015fcebe6336bd2010af9eb"
dependencies = [
 "android-tzdata",
 "iana-time-zone",
 "js-sys",
 "num-traits",
 "wasm-bindgen",
 "windows-targets 0.52.0",
]

[[package]]
name = "core-foundation"
version = "0.9.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "91e195e091a93c46f7102ec7818a2aa394e1e1771c3ab4825963fa03e45afb8f"
dependencies = [
 "core-foundation-sys",
 "libc",
]

[[package]]
name = "core-foundation-sys"
version = "0.8.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "06ea2b9bc92be3c2baa9334a323ebca2d6f074ff852cd1d7b11064035cd3868f"

[[package]]
name = "cpufeatures"
version = "0.2.12"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "53fe5e26ff1b7aef8bca9c6080520cfb8d9333c7568e1829cef191a9723e5504"
dependencies = [
 "libc",
]

[[package]]
name = "crossterm"
version = "0.25.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e64e6c0fbe2c17357405f7c758c1ef960fce08bdfb2c03d88d2a18d7e09c4b67"
dependencies = [
 "bitflags 1.3.2",
 "crossterm_winapi",
 "libc",
 "mio",
 "parking_lot",
 "signal-hook",
 "signal-hook-mio",
 "winapi",
]

[[package]]
name = "crossterm_winapi"
version = "0.9.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "acdd7c62a3665c7f6830a51635d9ac9b23ed385797f70a83bb8bafe9c572ab2b"
dependencies = [
 "winapi",
]

[[package]]
name = "crypto-common"
version = "0.1.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1bfb12502f3fc46cca1bb51ac28df9d618d813cdc3d2f25b9fe775a34af26bb3"
dependencies = [
 "generic-array",
 "typenum",
]

[[package]]
name = "data-encoding"
version = "2.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7e962a19be5cfc3f3bf6dd8f61eb50107f356ad6270fbb3ed41476571db78be5"

[[package]]
name = "deflate"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c86f7e25f518f4b81808a2cf1c50996a61f5c2eb394b2393bd87f2a4780a432f"
dependencies = [
 "adler32",
]

[[package]]
name = "digest"
version = "0.10.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9ed9a281f7bc9b7576e61468ba615a66a5c8cfdff42420a70aa82701a3b1e292"
dependencies = [
 "block-buffer",
 "crypto-common",
]

[[package]]
name = "dirs"
version = "5.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "44c45a9d03d6676652bcb5e724c7e988de1acad23a711b5217ab9cbecbec2225"
dependencies = [
 "dirs-sys",
]

[[package]]
name = "dirs-sys"
version = "0.4.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "520f05a5cbd335fae5a99ff7a6ab8627577660ee5cfd6a94a6a929b52ff0321c"
dependencies = [
 "libc",
 "option-ext",
 "redox_users",
 "windows-sys 0.48.0",
]

[[package]]
name = "dyn-clone"
version = "1.0.16"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "545b22097d44f8a9581187cdf93de7a71e4722bf51200cfaba810865b49a495d"

[[package]]
name = "encoding_rs"
version = "0.8.33"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7268b386296a025e474d5140678f75d6de9493ae55a5d709eeb9dd08149945e1"
dependencies = [
 "cfg-if 1.0.0",
]

[[package]]
name = "equivalent"
version = "1.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5443807d6dff69373d433ab9ef5378ad8df50ca6298caf15de6e52e24aaf54d5"

[[package]]
name = "errno"
version = "0.3.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a258e46cdc063eb8519c00b9fc845fc47bcfca4130e2f08e88665ceda8474245"
dependencies = [
 "libc",
 "windows-sys 0.52.0",
]

[[package]]
name = "fastrand"
version = "2.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "25cbce373ec4653f1a01a31e8a5e5ec0c622dc27ff9c4e6606eefef5cbbed4a5"

[[package]]
name = "filetime"
version = "0.2.23"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1ee447700ac8aa0b2f2bd7bc4462ad686ba06baa6727ac149a2d6277f0d240fd"
dependencies = [
 "cfg-if 1.0.0",
 "libc",
 "redox_syscall",
 "windows-sys 0.52.0",
]

[[package]]
name = "fnv"
version = "1.0.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3f9eec918d3f24069decb9af1554cad7c880e2da24a9afd88aca000531ab82c1"

[[package]]
name = "foreign-types"
version = "0.3.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f6f339eb8adc052cd2ca78910fda869aefa38d22d5cb648e6485e4d3fc06f3b1"
dependencies = [
 "foreign-types-shared",
]

[[package]]
name = "foreign-types-shared"
version = "0.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "00b0228411908ca8685dba7fc2cdd70ec9990a6e753e89b6ac91a84c40fbaf4b"

[[package]]
name = "form_urlencoded"
version = "1.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e13624c2627564efccf4934284bdd98cbaa14e79b0b5a141218e507b3a823456"
dependencies = [
 "percent-encoding",
]

[[package]]
name = "futures-channel"
version = "0.3.30"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "eac8f7d7865dcb88bd4373ab671c8cf4508703796caa2b1985a9ca867b3fcb78"
dependencies = [
 "futures-core",
]

[[package]]
name = "futures-core"
version = "0.3.30"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "dfc6580bb841c5a68e9ef15c77ccc837b40a7504914d52e47b8b0e9bbda25a1d"

[[package]]
name = "futures-io"
version = "0.3.30"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a44623e20b9681a318efdd71c299b6b222ed6f231972bfe2f224ebad6311f0c1"

[[package]]
name = "futures-macro"
version = "0.3.30"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "87750cf4b7a4c0625b1529e4c543c2182106e4dedc60a2a6455e00d212c489ac"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "futures-sink"
version = "0.3.30"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9fb8e00e87438d937621c1c6269e53f536c14d3fbd6a042bb24879e57d474fb5"

[[package]]
name = "futures-task"
version = "0.3.30"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "38d84fa142264698cdce1a9f9172cf383a0c82de1bddcf3092901442c4097004"

[[package]]
name = "futures-util"
version = "0.3.30"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3d6401deb83407ab3da39eba7e33987a73c3df0c82b4bb5813ee871c19c41d48"
dependencies = [
 "futures-core",
 "futures-io",
 "futures-macro",
 "futures-sink",
 "futures-task",
 "memchr",
 "pin-project-lite",
 "pin-utils",
 "slab",
]

[[package]]
name = "generic-array"
version = "0.14.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "85649ca51fd72272d7821adaf274ad91c288277713d9c18820d8499a7ff69e9a"
dependencies = [
 "typenum",
 "version_check",
]

[[package]]
name = "getrandom"
version = "0.2.12"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "190092ea657667030ac6a35e305e62fc4dd69fd98ac98631e5d3a2b1575a12b5"
dependencies = [
 "cfg-if 1.0.0",
 "libc",
 "wasi",
]

[[package]]
name = "gimli"
version = "0.28.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4271d37baee1b8c7e4b708028c57d816cf9d2434acb33a549475f78c181f6253"

[[package]]
name = "glob"
version = "0.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d2fabcfbdc87f4758337ca535fb41a6d701b65693ce38287d856d1674551ec9b"

[[package]]
name = "h2"
version = "0.3.24"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bb2c4422095b67ee78da96fbb51a4cc413b3b25883c7717ff7ca1ab31022c9c9"
dependencies = [
 "bytes",
 "fnv",
 "futures-core",
 "futures-sink",
 "futures-util",
 "http",
 "indexmap",
 "slab",
 "tokio",
 "tokio-util",
 "tracing",
]

[[package]]
name = "hashbrown"
version = "0.14.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "290f1a1d9242c78d09ce40a5e87e7554ee637af1351968159f4952f028f75604"

[[package]]
name = "hermit-abi"
version = "0.3.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d0c62115964e08cb8039170eb33c1d0e2388a256930279edca206fff675f82c3"

[[package]]
name = "home"
version = "0.5.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e3d1354bf6b7235cb4a0576c2619fd4ed18183f689b12b006a0ee7329eeff9a5"
dependencies = [
 "windows-sys 0.52.0",
]

[[package]]
name = "http"
version = "0.2.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8947b1a6fad4393052c7ba1f4cd97bed3e953a95c79c92ad9b051a04611d9fbb"
dependencies = [
 "bytes",
 "fnv",
 "itoa",
]

[[package]]
name = "http-body"
version = "0.4.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7ceab25649e9960c0311ea418d17bee82c0dcec1bd053b5f9a66e265a693bed2"
dependencies = [
 "bytes",
 "http",
 "pin-project-lite",
]

[[package]]
name = "httparse"
version = "1.8.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d897f394bad6a705d5f4104762e116a75639e470d80901eed05a860a95cb1904"

[[package]]
name = "httpdate"
version = "1.0.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "df3b46402a9d5adb4c86a0cf463f42e19994e3ee891101b1841f30a545cb49a9"

[[package]]
name = "hyper"
version = "0.14.28"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bf96e135eb83a2a8ddf766e426a841d8ddd7449d5f00d34ea02b41d2f19eef80"
dependencies = [
 "bytes",
 "futures-channel",
 "futures-core",
 "futures-util",
 "h2",
 "http",
 "http-body",
 "httparse",
 "httpdate",
 "itoa",
 "pin-project-lite",
 "socket2",
 "tokio",
 "tower-service",
 "tracing",
 "want",
]

[[package]]
name = "hyper-tls"
version = "0.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d6183ddfa99b85da61a140bea0efc93fdf56ceaa041b37d553518030827f9905"
dependencies = [
 "bytes",
 "hyper",
 "native-tls",
 "tokio",
 "tokio-native-tls",
]

[[package]]
name = "iana-time-zone"
version = "0.1.60"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e7ffbb5a1b541ea2561f8c41c087286cc091e21e556a4f09a8f6cbf17b69b141"
dependencies = [
 "android_system_properties",
 "core-foundation-sys",
 "iana-time-zone-haiku",
 "js-sys",
 "wasm-bindgen",
 "windows-core",
]

[[package]]
name = "iana-time-zone-haiku"
version = "0.1.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f31827a206f56af32e590ba56d5d2d085f558508192593743f16b2306495269f"
dependencies = [
 "cc",
]

[[package]]
name = "idna"
version = "0.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "634d9b1461af396cad843f47fdba5597a4f9e6ddd4bfb6ff5d85028c25cb12f6"
dependencies = [
 "unicode-bidi",
 "unicode-normalization",
]

[[package]]
name = "indexmap"
version = "2.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "824b2ae422412366ba479e8111fd301f7b5faece8149317bb81925979a53f520"
dependencies = [
 "equivalent",
 "hashbrown",
]

[[package]]
name = "inquire"
version = "0.6.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c33e7c1ddeb15c9abcbfef6029d8e29f69b52b6d6c891031b88ed91b5065803b"
dependencies = [
 "bitflags 1.3.2",
 "crossterm",
 "dyn-clone",
 "lazy_static",
 "newline-converter",
 "thiserror",
 "unicode-segmentation",
 "unicode-width",
]

[[package]]
name = "ipnet"
version = "2.9.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8f518f335dce6725a761382244631d86cf0ccb2863413590b31338feb467f9c3"

[[package]]
name = "itoa"
version = "1.0.10"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b1a46d1a171d865aa5f83f92695765caa047a9b4cbae2cbf37dbd613a793fd4c"

[[package]]
name = "js-sys"
version = "0.3.68"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "406cda4b368d531c842222cf9d2600a9a4acce8d29423695379c6868a143a9ee"
dependencies = [
 "wasm-bindgen",
]

[[package]]
name = "lazy_static"
version = "1.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e2abad23fbc42b3700f2f279844dc832adb2b2eb069b2df918f455c4e18cc646"

[[package]]
name = "libc"
version = "0.2.153"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9c198f91728a82281a64e1f4f9eeb25d82cb32a5de251c6bd1b5154d63a8e7bd"

[[package]]
name = "libredox"
version = "0.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "85c833ca1e66078851dba29046874e38f08b2c883700aa29a03ddd3b23814ee8"
dependencies = [
 "bitflags 2.4.2",
 "libc",
 "redox_syscall",
]

[[package]]
name = "libredox"
version = "0.0.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3af92c55d7d839293953fcd0fda5ecfe93297cfde6ffbdec13b41d99c0ba6607"
dependencies = [
 "bitflags 2.4.2",
 "libc",
 "redox_syscall",
]

[[package]]
name = "linux-raw-sys"
version = "0.4.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "01cda141df6706de531b6c46c3a33ecca755538219bd484262fa09410c13539c"

[[package]]
name = "lock_api"
version = "0.4.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3c168f8615b12bc01f9c17e2eb0cc07dcae1940121185446edc3744920e8ef45"
dependencies = [
 "autocfg",
 "scopeguard",
]

[[package]]
name = "log"
version = "0.4.20"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b5e6163cb8c49088c2c36f57875e58ccd8c87c7427f7fbd50ea6710b2f3f2e8f"

[[package]]
name = "memchr"
version = "2.7.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "523dc4f511e55ab87b694dc30d0f820d60906ef06413f93d4d7a1385599cc149"

[[package]]
name = "mime"
version = "0.3.17"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6877bb514081ee2a7ff5ef9de3281f14a4dd4bceac4c09388074a6b5df8a139a"

[[package]]
name = "miniz_oxide"
version = "0.7.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9d811f3e15f28568be3407c8e7fdb6514c1cda3cb30683f15b6a1a1dc4ea14a7"
dependencies = [
 "adler",
]

[[package]]
name = "mio"
version = "0.8.10"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8f3d0b296e374a4e6f3c7b0a1f5a51d748a0d34c85e7dc48fc3fa9a87657fe09"
dependencies = [
 "libc",
 "log",
 "wasi",
 "windows-sys 0.48.0",
]

[[package]]
name = "native-tls"
version = "0.2.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "07226173c32f2926027b63cce4bcd8076c3552846cbe7925f3aaffeac0a3b92e"
dependencies = [
 "lazy_static",
 "libc",
 "log",
 "openssl",
 "openssl-probe",
 "openssl-sys",
 "schannel",
 "security-framework",
 "security-framework-sys",
 "tempfile",
]

[[package]]
name = "newline-converter"
version = "0.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1f71d09d5c87634207f894c6b31b6a2b2c64ea3bdcf71bd5599fdbbe1600c00f"
dependencies = [
 "unicode-segmentation",
]

[[package]]
name = "num-traits"
version = "0.2.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "da0df0e5185db44f69b44f26786fe401b6c293d1907744beaa7fa62b2e5a517a"
dependencies = [
 "autocfg",
]

[[package]]
name = "num_cpus"
version = "1.16.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4161fcb6d602d4d2081af7c3a45852d875a03dd337a6bfdd6e06407b61342a43"
dependencies = [
 "hermit-abi",
 "libc",
]

[[package]]
name = "numtoa"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b8f8bdf33df195859076e54ab11ee78a1b208382d3a26ec40d142ffc1ecc49ef"

[[package]]
name = "object"
version = "0.32.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a6a622008b6e321afc04970976f62ee297fdbaa6f95318ca343e3eebb9648441"
dependencies = [
 "memchr",
]

[[package]]
name = "once_cell"
version = "1.19.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3fdb12b2476b595f9358c5161aa467c2438859caa136dec86c26fdd2efe17b92"

[[package]]
name = "openssl"
version = "0.10.63"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "15c9d69dd87a29568d4d017cfe8ec518706046a05184e5aea92d0af890b803c8"
dependencies = [
 "bitflags 2.4.2",
 "cfg-if 1.0.0",
 "foreign-types",
 "libc",
 "once_cell",
 "openssl-macros",
 "openssl-sys",
]

[[package]]
name = "openssl-macros"
version = "0.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a948666b637a0f465e8564c73e89d4dde00d72d4d473cc972f390fc3dcee7d9c"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "openssl-probe"
version = "0.1.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ff011a302c396a5197692431fc1948019154afc178baf7d8e37367442a4601cf"

[[package]]
name = "openssl-sys"
version = "0.9.99"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "22e1bf214306098e4832460f797824c05d25aacdf896f64a985fb0fd992454ae"
dependencies = [
 "cc",
 "libc",
 "pkg-config",
 "vcpkg",
]

[[package]]
name = "option-ext"
version = "0.2.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "04744f49eae99ab78e0d5c0b603ab218f515ea8cfe5a456d7629ad883a3b6e7d"

[[package]]
name = "parking_lot"
version = "0.12.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3742b2c103b9f06bc9fff0a37ff4912935851bee6d36f3c02bcc755bcfec228f"
dependencies = [
 "lock_api",
 "parking_lot_core",
]

[[package]]
name = "parking_lot_core"
version = "0.9.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4c42a9226546d68acdd9c0a280d17ce19bfe27a46bf68784e4066115788d008e"
dependencies = [
 "cfg-if 1.0.0",
 "libc",
 "redox_syscall",
 "smallvec",
 "windows-targets 0.48.5",
]

[[package]]
name = "percent-encoding"
version = "2.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e3148f5046208a5d56bcfc03053e3ca6334e51da8dfb19b6cdc8b306fae3283e"

[[package]]
name = "pin-project-lite"
version = "0.2.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8afb450f006bf6385ca15ef45d71d2288452bc3683ce2e2cacc0d18e4be60b58"

[[package]]
name = "pin-utils"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8b870d8c151b6f2fb93e84a13146138f05d02ed11c7e7c54f8826aaaf7c9f184"

[[package]]
name = "pkg-config"
version = "0.3.29"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2900ede94e305130c13ddd391e0ab7cbaeb783945ae07a279c268cb05109c6cb"

[[package]]
name = "pretty_dbg"
version = "1.0.49"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "85c9cc6dcf2ee8ab93287669c0beaca467eb8dfcfef3ba71b6beb72bd81b11e1"

[[package]]
name = "proc-macro2"
version = "1.0.78"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e2422ad645d89c99f8f3e6b88a9fdeca7fabeac836b1002371c4367c8f984aae"
dependencies = [
 "unicode-ident",
]

[[package]]
name = "quote"
version = "1.0.35"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "291ec9ab5efd934aaf503a6466c5d5251535d108ee747472c3977cc5acc868ef"
dependencies = [
 "proc-macro2",
]

[[package]]
name = "radix64"
version = "0.6.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "999718fa65c3be3a74f3f6dae5a98526ff436ea58a82a574f0de89eecd342bee"
dependencies = [
 "arrayref",
 "cfg-if 0.1.10",
]

[[package]]
name = "reader_for_microxml"
version = "2.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0d726a3f4c11def37106edaf44caf861a9012ebcd4eb6f748cc4fd93c2a15de1"

[[package]]
name = "redox_syscall"
version = "0.4.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4722d768eff46b75989dd134e5c353f0d6296e5aaa3132e776cbdb56be7731aa"
dependencies = [
 "bitflags 1.3.2",
]

[[package]]
name = "redox_termios"
version = "0.1.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "20145670ba436b55d91fc92d25e71160fbfbdd57831631c8d7d36377a476f1cb"

[[package]]
name = "redox_users"
version = "0.4.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a18479200779601e498ada4e8c1e1f50e3ee19deb0259c25825a98b5603b2cb4"
dependencies = [
 "getrandom",
 "libredox 0.0.1",
 "thiserror",
]

[[package]]
name = "regex"
version = "1.10.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b62dbe01f0b06f9d8dc7d49e05a0785f153b00b2c227856282f671e0318c9b15"
dependencies = [
 "aho-corasick",
 "memchr",
 "regex-automata",
 "regex-syntax",
]

[[package]]
name = "regex-automata"
version = "0.4.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5bb987efffd3c6d0d8f5f89510bb458559eab11e4f869acb20bf845e016259cd"
dependencies = [
 "aho-corasick",
 "memchr",
 "regex-syntax",
]

[[package]]
name = "regex-syntax"
version = "0.8.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c08c74e62047bb2de4ff487b251e4a92e24f48745648451635cec7d591162d9f"

[[package]]
name = "reqwest"
version = "0.11.24"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c6920094eb85afde5e4a138be3f2de8bbdf28000f0029e72c45025a56b042251"
dependencies = [
 "base64",
 "bytes",
 "encoding_rs",
 "futures-core",
 "futures-util",
 "h2",
 "http",
 "http-body",
 "hyper",
 "hyper-tls",
 "ipnet",
 "js-sys",
 "log",
 "mime",
 "native-tls",
 "once_cell",
 "percent-encoding",
 "pin-project-lite",
 "rustls-pemfile",
 "serde",
 "serde_json",
 "serde_urlencoded",
 "sync_wrapper",
 "system-configuration",
 "tokio",
 "tokio-native-tls",
 "tokio-util",
 "tower-service",
 "url",
 "wasm-bindgen",
 "wasm-bindgen-futures",
 "wasm-streams",
 "web-sys",
 "winreg",
]

[[package]]
name = "ring"
version = "0.17.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "688c63d65483050968b2a8937f7995f443e27041a0f7700aa59b0822aedebb74"
dependencies = [
 "cc",
 "getrandom",
 "libc",
 "spin",
 "untrusted",
 "windows-sys 0.48.0",
]

[[package]]
name = "rustc-demangle"
version = "0.1.23"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d626bb9dae77e28219937af045c257c28bfd3f69333c512553507f5f9798cb76"

[[package]]
name = "rustix"
version = "0.38.31"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6ea3e1a662af26cd7a3ba09c0297a31af215563ecf42817c98df621387f4e949"
dependencies = [
 "bitflags 2.4.2",
 "errno",
 "libc",
 "linux-raw-sys",
 "windows-sys 0.52.0",
]

[[package]]
name = "rustls-pemfile"
version = "1.0.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1c74cae0a4cf6ccbbf5f359f08efdf8ee7e1dc532573bf0db71968cb56b1448c"
dependencies = [
 "base64",
]

[[package]]
name = "ryu"
version = "1.0.16"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f98d2aa92eebf49b69786be48e4477826b256916e84a57ff2a4f21923b48eb4c"

[[package]]
name = "schannel"
version = "0.1.23"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fbc91545643bcf3a0bbb6569265615222618bdf33ce4ffbbd13c4bbd4c093534"
dependencies = [
 "windows-sys 0.52.0",
]

[[package]]
name = "scopeguard"
version = "1.2.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "94143f37725109f92c262ed2cf5e59bce7498c01bcc1502d7b9afe439a4e9f49"

[[package]]
name = "security-framework"
version = "2.9.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "05b64fb303737d99b81884b2c63433e9ae28abebe5eb5045dcdd175dc2ecf4de"
dependencies = [
 "bitflags 1.3.2",
 "core-foundation",
 "core-foundation-sys",
 "libc",
 "security-framework-sys",
]

[[package]]
name = "security-framework-sys"
version = "2.9.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e932934257d3b408ed8f30db49d85ea163bfe74961f017f405b025af298f0c7a"
dependencies = [
 "core-foundation-sys",
 "libc",
]

[[package]]
name = "semver"
version = "1.0.21"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b97ed7a9823b74f99c7742f5336af7be5ecd3eeafcb1507d1fa93347b1d589b0"

[[package]]
name = "serde"
version = "1.0.196"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "870026e60fa08c69f064aa766c10f10b1d62db9ccd4d0abb206472bee0ce3b32"
dependencies = [
 "serde_derive",
]

[[package]]
name = "serde_derive"
version = "1.0.196"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "33c85360c95e7d137454dc81d9a4ed2b8efd8fbe19cee57357b32b9771fccb67"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "serde_json"
version = "1.0.113"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "69801b70b1c3dac963ecb03a364ba0ceda9cf60c71cfe475e99864759c8b8a79"
dependencies = [
 "itoa",
 "ryu",
 "serde",
]

[[package]]
name = "serde_spanned"
version = "0.6.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "eb3622f419d1296904700073ea6cc23ad690adbd66f13ea683df73298736f0c1"
dependencies = [
 "serde",
]

[[package]]
name = "serde_urlencoded"
version = "0.7.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d3491c14715ca2294c4d6a88f15e84739788c1d030eed8c110436aafdaa2f3fd"
dependencies = [
 "form_urlencoded",
 "itoa",
 "ryu",
 "serde",
]

[[package]]
name = "sha2"
version = "0.10.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "793db75ad2bcafc3ffa7c68b215fee268f537982cd901d132f89c6343f3a3dc8"
dependencies = [
 "cfg-if 1.0.0",
 "cpufeatures",
 "digest",
]

[[package]]
name = "signal-hook"
version = "0.3.17"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8621587d4798caf8eb44879d42e56b9a93ea5dcd315a6487c357130095b62801"
dependencies = [
 "libc",
 "signal-hook-registry",
]

[[package]]
name = "signal-hook-mio"
version = "0.2.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "29ad2e15f37ec9a6cc544097b78a1ec90001e9f71b81338ca39f430adaca99af"
dependencies = [
 "libc",
 "mio",
 "signal-hook",
]

[[package]]
name = "signal-hook-registry"
version = "1.4.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d8229b473baa5980ac72ef434c4415e70c4b5e71b423043adb4ba059f89c99a1"
dependencies = [
 "libc",
]

[[package]]
name = "slab"
version = "0.4.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8f92a496fb766b417c996b9c5e57daf2f7ad3b0bebe1ccfca4856390e3d3bb67"
dependencies = [
 "autocfg",
]

[[package]]
name = "smallvec"
version = "1.13.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e6ecd384b10a64542d77071bd64bd7b231f4ed5940fba55e98c3de13824cf3d7"

[[package]]
name = "socket2"
version = "0.5.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7b5fac59a5cb5dd637972e5fca70daf0523c9067fcdc4842f053dae04a18f8e9"
dependencies = [
 "libc",
 "windows-sys 0.48.0",
]

[[package]]
name = "spin"
version = "0.9.8"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6980e8d7511241f8acf4aebddbb1ff938df5eebe98691418c4468d0b72a96a67"

[[package]]
name = "ssh2-config"
version = "0.2.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "98150bad1e8fe53df07f38b53364f4d34e84a6cc2ee9f933e43629571060af65"
dependencies = [
 "bitflags 2.4.2",
 "dirs",
 "thiserror",
 "wildmatch",
]

[[package]]
name = "syn"
version = "2.0.48"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0f3531638e407dfc0814761abb7c00a5b54992b849452a0646b7f65c9f770f3f"
dependencies = [
 "proc-macro2",
 "quote",
 "unicode-ident",
]

[[package]]
name = "sync_wrapper"
version = "0.1.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2047c6ded9c721764247e62cd3b03c09ffc529b2ba5b10ec482ae507a4a70160"

[[package]]
name = "system-configuration"
version = "0.5.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ba3a3adc5c275d719af8cb4272ea1c4a6d668a777f37e115f6d11ddbc1c8e0e7"
dependencies = [
 "bitflags 1.3.2",
 "core-foundation",
 "system-configuration-sys",
]

[[package]]
name = "system-configuration-sys"
version = "0.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a75fb188eb626b924683e3b95e3a48e63551fcfb51949de2f06a9d91dbee93c9"
dependencies = [
 "core-foundation-sys",
 "libc",
]

[[package]]
name = "tempfile"
version = "3.10.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a365e8cd18e44762ef95d87f284f4b5cd04107fec2ff3052bd6a3e6069669e67"
dependencies = [
 "cfg-if 1.0.0",
 "fastrand",
 "rustix",
 "windows-sys 0.52.0",
]

[[package]]
name = "termion"
version = "3.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "417813675a504dfbbf21bfde32c03e5bf9f2413999962b479023c02848c1c7a5"
dependencies = [
 "libc",
 "libredox 0.0.2",
 "numtoa",
 "redox_termios",
]

[[package]]
name = "thiserror"
version = "1.0.56"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d54378c645627613241d077a3a79db965db602882668f9136ac42af9ecb730ad"
dependencies = [
 "thiserror-impl",
]

[[package]]
name = "thiserror-impl"
version = "1.0.56"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fa0faa943b50f3db30a20aa7e265dbc66076993efed8463e8de414e5d06d3471"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "tinyvec"
version = "1.6.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "87cc5ceb3875bb20c2890005a4e226a4651264a5c75edb2421b52861a0a0cb50"
dependencies = [
 "tinyvec_macros",
]

[[package]]
name = "tinyvec_macros"
version = "0.1.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1f3ccbac311fea05f86f61904b462b55fb3df8837a366dfc601a0161d0532f20"

[[package]]
name = "tokio"
version = "1.36.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "61285f6515fa018fb2d1e46eb21223fff441ee8db5d0f1435e8ab4f5cdb80931"
dependencies = [
 "backtrace",
 "bytes",
 "libc",
 "mio",
 "num_cpus",
 "pin-project-lite",
 "socket2",
 "windows-sys 0.48.0",
]

[[package]]
name = "tokio-native-tls"
version = "0.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bbae76ab933c85776efabc971569dd6119c580d8f5d448769dec1764bf796ef2"
dependencies = [
 "native-tls",
 "tokio",
]

[[package]]
name = "tokio-util"
version = "0.7.10"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5419f34732d9eb6ee4c3578b7989078579b7f039cbbb9ca2c4da015749371e15"
dependencies = [
 "bytes",
 "futures-core",
 "futures-sink",
 "pin-project-lite",
 "tokio",
 "tracing",
]

[[package]]
name = "toml"
version = "0.8.10"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9a9aad4a3066010876e8dcf5a8a06e70a558751117a145c6ce2b82c2e2054290"
dependencies = [
 "serde",
 "serde_spanned",
 "toml_datetime",
 "toml_edit",
]

[[package]]
name = "toml_datetime"
version = "0.6.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3550f4e9685620ac18a50ed434eb3aec30db8ba93b0287467bca5826ea25baf1"
dependencies = [
 "serde",
]

[[package]]
name = "toml_edit"
version = "0.22.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0c9ffdf896f8daaabf9b66ba8e77ea1ed5ed0f72821b398aba62352e95062951"
dependencies = [
 "indexmap",
 "serde",
 "serde_spanned",
 "toml_datetime",
 "winnow",
]

[[package]]
name = "tower-service"
version = "0.3.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b6bc1c9ce2b5135ac7f93c72918fc37feb872bdc6a5533a8b85eb4b86bfdae52"

[[package]]
name = "tracing"
version = "0.1.40"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c3523ab5a71916ccf420eebdf5521fcef02141234bbc0b8a49f2fdc4544364ef"
dependencies = [
 "pin-project-lite",
 "tracing-core",
]

[[package]]
name = "tracing-core"
version = "0.1.32"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c06d3da6113f116aaee68e4d601191614c9053067f9ab7f6edbcb161237daa54"
dependencies = [
 "once_cell",
]

[[package]]
name = "try-lock"
version = "0.2.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e421abadd41a4225275504ea4d6566923418b7f05506fbc9c0fe86ba7396114b"

[[package]]
name = "typenum"
version = "1.17.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "42ff0bf0c66b8238c6f3b578df37d0b7848e55df8577b3f74f92a69acceeb825"

[[package]]
name = "unicode-bidi"
version = "0.3.15"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "08f95100a766bf4f8f28f90d77e0a5461bbdb219042e7679bebe79004fed8d75"

[[package]]
name = "unicode-ident"
version = "1.0.12"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3354b9ac3fae1ff6755cb6db53683adb661634f67557942dea4facebec0fee4b"

[[package]]
name = "unicode-normalization"
version = "0.1.22"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5c5713f0fc4b5db668a2ac63cdb7bb4469d8c9fed047b1d0292cc7b0ce2ba921"
dependencies = [
 "tinyvec",
]

[[package]]
name = "unicode-segmentation"
version = "1.11.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d4c87d22b6e3f4a18d4d40ef354e97c90fcb14dd91d7dc0aa9d8a1172ebf7202"

[[package]]
name = "unicode-width"
version = "0.1.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e51733f11c9c4f72aa0c160008246859e340b00807569a0da0e7a1079b27ba85"

[[package]]
name = "untrusted"
version = "0.9.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8ecb6da28b8a351d773b68d5825ac39017e680750f980f3a1a85cd8dd28a47c1"

[[package]]
name = "url"
version = "2.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "31e6302e3bb753d46e83516cae55ae196fc0c309407cf11ab35cc51a4c2a4633"
dependencies = [
 "form_urlencoded",
 "idna",
 "percent-encoding",
]

[[package]]
name = "vcpkg"
version = "0.2.15"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "accd4ea62f7bb7a82fe23066fb0957d48ef677f6eeb8215f372f52e48bb32426"

[[package]]
name = "version_check"
version = "0.9.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "49874b5167b65d7193b8aba1567f5c7d93d001cafc34600cee003eda787e483f"

[[package]]
name = "want"
version = "0.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bfa7760aed19e106de2c7c0b581b509f2f25d3dacaf737cb82ac61bc6d760b0e"
dependencies = [
 "try-lock",
]

[[package]]
name = "wasi"
version = "0.11.0+wasi-snapshot-preview1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9c8d87e72b64a3b4db28d11ce29237c246188f4f51057d65a7eab63b7987e423"

[[package]]
name = "wasm-bindgen"
version = "0.2.91"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c1e124130aee3fb58c5bdd6b639a0509486b0338acaaae0c84a5124b0f588b7f"
dependencies = [
 "cfg-if 1.0.0",
 "wasm-bindgen-macro",
]

[[package]]
name = "wasm-bindgen-backend"
version = "0.2.91"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c9e7e1900c352b609c8488ad12639a311045f40a35491fb69ba8c12f758af70b"
dependencies = [
 "bumpalo",
 "log",
 "once_cell",
 "proc-macro2",
 "quote",
 "syn",
 "wasm-bindgen-shared",
]

[[package]]
name = "wasm-bindgen-futures"
version = "0.4.41"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "877b9c3f61ceea0e56331985743b13f3d25c406a7098d45180fb5f09bc19ed97"
dependencies = [
 "cfg-if 1.0.0",
 "js-sys",
 "wasm-bindgen",
 "web-sys",
]

[[package]]
name = "wasm-bindgen-macro"
version = "0.2.91"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b30af9e2d358182b5c7449424f017eba305ed32a7010509ede96cdc4696c46ed"
dependencies = [
 "quote",
 "wasm-bindgen-macro-support",
]

[[package]]
name = "wasm-bindgen-macro-support"
version = "0.2.91"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "642f325be6301eb8107a83d12a8ac6c1e1c54345a7ef1a9261962dfefda09e66"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
 "wasm-bindgen-backend",
 "wasm-bindgen-shared",
]

[[package]]
name = "wasm-bindgen-shared"
version = "0.2.91"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4f186bd2dcf04330886ce82d6f33dd75a7bfcf69ecf5763b89fcde53b6ac9838"

[[package]]
name = "wasm-streams"
version = "0.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b65dc4c90b63b118468cf747d8bf3566c1913ef60be765b5730ead9e0a3ba129"
dependencies = [
 "futures-util",
 "js-sys",
 "wasm-bindgen",
 "wasm-bindgen-futures",
 "web-sys",
]

[[package]]
name = "web-sys"
version = "0.3.68"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "96565907687f7aceb35bc5fc03770a8a0471d82e479f25832f54a0e3f4b28446"
dependencies = [
 "js-sys",
 "wasm-bindgen",
]

[[package]]
name = "wildmatch"
version = "2.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "495ec47bf3c1345005f40724f0269362c8556cbc43aed0526ed44cae1d35fceb"

[[package]]
name = "winapi"
version = "0.3.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419"
dependencies = [
 "winapi-i686-pc-windows-gnu",
 "winapi-x86_64-pc-windows-gnu",
]

[[package]]
name = "winapi-i686-pc-windows-gnu"
version = "0.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6"

[[package]]
name = "winapi-x86_64-pc-windows-gnu"
version = "0.4.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f"

[[package]]
name = "windows-core"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "33ab640c8d7e35bf8ba19b884ba838ceb4fba93a4e8c65a9059d08afcfc683d9"
dependencies = [
 "windows-targets 0.52.0",
]

[[package]]
name = "windows-sys"
version = "0.48.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "677d2418bec65e3338edb076e806bc1ec15693c5d0104683f2efe857f61056a9"
dependencies = [
 "windows-targets 0.48.5",
]

[[package]]
name = "windows-sys"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "282be5f36a8ce781fad8c8ae18fa3f9beff57ec1b52cb3de0789201425d9a33d"
dependencies = [
 "windows-targets 0.52.0",
]

[[package]]
name = "windows-targets"
version = "0.48.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9a2fa6e2155d7247be68c096456083145c183cbbbc2764150dda45a87197940c"
dependencies = [
 "windows_aarch64_gnullvm 0.48.5",
 "windows_aarch64_msvc 0.48.5",
 "windows_i686_gnu 0.48.5",
 "windows_i686_msvc 0.48.5",
 "windows_x86_64_gnu 0.48.5",
 "windows_x86_64_gnullvm 0.48.5",
 "windows_x86_64_msvc 0.48.5",
]

[[package]]
name = "windows-targets"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8a18201040b24831fbb9e4eb208f8892e1f50a37feb53cc7ff887feb8f50e7cd"
dependencies = [
 "windows_aarch64_gnullvm 0.52.0",
 "windows_aarch64_msvc 0.52.0",
 "windows_i686_gnu 0.52.0",
 "windows_i686_msvc 0.52.0",
 "windows_x86_64_gnu 0.52.0",
 "windows_x86_64_gnullvm 0.52.0",
 "windows_x86_64_msvc 0.52.0",
]

[[package]]
name = "windows_aarch64_gnullvm"
version = "0.48.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2b38e32f0abccf9987a4e3079dfb67dcd799fb61361e53e2882c3cbaf0d905d8"

[[package]]
name = "windows_aarch64_gnullvm"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "cb7764e35d4db8a7921e09562a0304bf2f93e0a51bfccee0bd0bb0b666b015ea"

[[package]]
name = "windows_aarch64_msvc"
version = "0.48.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "dc35310971f3b2dbbf3f0690a219f40e2d9afcf64f9ab7cc1be722937c26b4bc"

[[package]]
name = "windows_aarch64_msvc"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bbaa0368d4f1d2aaefc55b6fcfee13f41544ddf36801e793edbbfd7d7df075ef"

[[package]]
name = "windows_i686_gnu"
version = "0.48.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a75915e7def60c94dcef72200b9a8e58e5091744960da64ec734a6c6e9b3743e"

[[package]]
name = "windows_i686_gnu"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a28637cb1fa3560a16915793afb20081aba2c92ee8af57b4d5f28e4b3e7df313"

[[package]]
name = "windows_i686_msvc"
version = "0.48.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8f55c233f70c4b27f66c523580f78f1004e8b5a8b659e05a4eb49d4166cca406"

[[package]]
name = "windows_i686_msvc"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ffe5e8e31046ce6230cc7215707b816e339ff4d4d67c65dffa206fd0f7aa7b9a"

[[package]]
name = "windows_x86_64_gnu"
version = "0.48.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "53d40abd2583d23e4718fddf1ebec84dbff8381c07cae67ff7768bbf19c6718e"

[[package]]
name = "windows_x86_64_gnu"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3d6fa32db2bc4a2f5abeacf2b69f7992cd09dca97498da74a151a3132c26befd"

[[package]]
name = "windows_x86_64_gnullvm"
version = "0.48.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0b7b52767868a23d5bab768e390dc5f5c55825b6d30b86c844ff2dc7414044cc"

[[package]]
name = "windows_x86_64_gnullvm"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1a657e1e9d3f514745a572a6846d3c7aa7dbe1658c056ed9c3344c4109a6949e"

[[package]]
name = "windows_x86_64_msvc"
version = "0.48.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ed94fce61571a4006852b7389a063ab983c02eb1bb37b47f8272ce92d06d9538"

[[package]]
name = "windows_x86_64_msvc"
version = "0.52.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "dff9641d1cd4be8d1a070daf9e3773c5f67e78b4d9d42263020c057706765c04"

[[package]]
name = "winnow"
version = "0.5.39"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5389a154b01683d28c77f8f68f49dea75f0a4da32557a58f68ee51ebba472d29"
dependencies = [
 "memchr",
]

[[package]]
name = "winreg"
version = "0.50.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "524e57b2c537c0f9b1e69f1965311ec12182b4122e45035b1508cd24d2adadb1"
dependencies = [
 "cfg-if 1.0.0",
 "windows-sys 0.48.0",
]
"###,
    });
    vec_file.push(crate::FileItem{
            file_name :"DEVELOPMENT.md",
            file_content : r###"# Development details

## CRDE - Containerized Rust Development Environment

I recommend using the CRDE - Containerized Rust Development Environment to write Rust projects. Follow the instructions here <https://github.com/bestia-dev/docker_rust_development>.  

It is an isolated development environment that will not mess with you system.
It will work on Linux (tested on Debian) and inside WSL (Windows Subsystem for Linux).

You just need to install the newer alternative to Docker: [podman](https://podman.io/). Then you download the prepared container image from DockerHub (3GB). And then a little juggling with ssh keys. All this is simplified by running a few bash scripts. Just follow the easy instructions.  

The container image contains cargo, rustc, wasm-pack, basic-http-server, cargo-auto and other utils that a Rust project needs.  

## Workflow with automation_tasks_rs and cargo-auto

For easy workflow, use the automation tasks that are already coded in the sub-project `automation_tasks_rs`. This is a basic workflow:

```bash
cargo auto build
cargo auto release
cargo auto doc
cargo auto test
cargo auto commit_and push
cargo auto publish_to_crates_io
cargo auto github_new_release
```

Every task finishes with instructions how to proceed.  
The [cargo-auto](https://github.com/bestia-dev/cargo-auto) and [dev_bestia_cargo_completion](https://github.com/bestia-dev/dev_bestia_cargo_completion) are already installed inside the CRDE container.

You can open the automation sub-project in VSCode and then code your own tasks in Rust.

```bash
code automation_tasks_rs
```

## HTML, CSS

The simple static HTML and CSS files are in `web_server_folder/cargo_auto_template_new_pwa_wasm`.  
Then the Rust code injects html elements into the DOM.  

## Web server and wasm

The browser security does not allow the loading of WASM modules from local files. It needs to be loaded from a web server. The CRDE container has the [basic-http-server](https://github.com/brson/basic-http-server) already installed.  

Run the server in a second VSCode terminal, so it can keep running all the time.  

```bash
basic-http-server -a 0.0.0.0:4000 ./web_server_folder
```

In the first VSCode terminal, we can build the project.  
Then in the browser, we can refresh the page <http://localhost:4000/cargo_auto_template_new_pwa_wasm> with F5 to see the changes.  

## Rust and wasm

In the `Cargo.toml` it is important to define the output as wasm library and the required dependencies to web-sys, js-sys, and wasm-bindgen.  
Wasm starts from the `src/lib.rs`. On purpose, I added the `main_mod.rs` and `lib_mod.rs` to make the project structure similar to a Rust CLI project.  
The User Interface UI is completely different in-browser than in a CLI, but we can reuse the libraries if they are UI agnostic.  
It is smart to split a project so that the logic does not contain anything about the UI.

## GitHub

This template contains GitHub actions to build the project on commit and publish the documentation on GutHub pages.  
"###,
});
    vec_file.push(crate::FileItem {
        file_name: "LICENSE",
        file_content: r###"MIT License

Copyright (c) 2024 bestia.dev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"###,
    });
    // endregion: files copied into strings by automation tasks

    // return
    vec_file
}
