use std::{fs, path::Path};

fn main() {
    fs::copy("../browser-ext/script.js", "../static/script.js")
        .expect("copy script file from `browser-ext` folder to static folder");
    if Path::new("../frontend").exists() {
        fs::copy(
            "../browser-ext/script.js",
            "../frontend/public/app/script.js",
        )
        .expect("copy script file from `browser-ext` folder to frontend folder");
    }
}
