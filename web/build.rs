use std::{fs, path::Path};

fn main() {
    if Path::new("../frontend").exists() {
        fs::copy("../browser-ext/script.js", "../frontend/script.js")
            .expect("copy script file from `browser-ext` folder to frontend folder");
    }
}
