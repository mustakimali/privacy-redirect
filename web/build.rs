use std::{fs, path::Path};

fn main() {
    if !Path::new("../static").exists() {
        panic!("Welcome: Please run `./build.sh` to build your static assets before the first run.")
    }
    fs::copy("../browser-ext/script.js", "../static/script.js")
        .expect("copy script file from `browser-ext` folder to static folder");

    fs::copy(
        "../browser-ext/script.js",
        "../frontend/public/app/script.js",
    )
    .expect("copy script file from `browser-ext` folder to frontend folder");
}
