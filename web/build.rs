use std::{fs, path::Path};

fn main() {
    if Path::new("../frontend").exists() {
        _ = fs::create_dir_all("../frontend/app/");
        fs::copy(
            "../browser-ext/script.js",
            "../frontend/app/script.js",
        )
        .expect("copy script file from `browser-ext` folder to frontend folder");
    }
}
