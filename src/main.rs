use std::{
    env::{self, consts::OS},
    fs, io, os,
    path::Path,
};

fn main() {
    let current_dir = env::current_dir().unwrap();
    let username = env::var("username").unwrap();
    // let config_dir = generate_config_dir("config.json");
    // let config_exists = fs::read(config_dir);

    let dir = Path::new("/home/capts/desktop/test");
    fs::create_dir(dir);
}

// fn generate_config_dir(filename: &str) -> &Path {
//     let os = env::consts::OS;
//     let username = env::var("username").unwrap().to_owned();

//     if (os == "windows") {
//         return Path::new(format!("C:\\{username}\\.npm-purger\\{filename}"));
//     } else if (os == "macos" || os == "linux") {
//         return Path::new(&format!("/home/{username}/.npm-purger/{filename}"));
//     } else {
//         return Path::new(&format!("/home/{username}/.npm-purger/{filename}"));
//     }
// }
