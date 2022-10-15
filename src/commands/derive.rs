use crate::lib::config::Config;
use crate::lib::helpers;

use std::fs;
use std::path::{Path, PathBuf};

pub fn deriver(contents: Config) {
    println!("Deriving theme.mp3 files, and copying them into your theme folder.");

    // Check if folders are writable.
    let root_folders = contents.folders;
    helpers::check_folders_permissions(&root_folders);
    helpers::check_folders_permissions(&vec![contents.themes.clone()]);

    // Get vector of all media folders
    let media_folders = helpers::determine_media_folders(root_folders);
    derive(&media_folders, contents.themes.clone());
}

pub fn derive(media_folders: &Vec<PathBuf>, themes_folder: String){
    // For each media folder, if a theme.mp3 exists, copy it into the themes folder.
    for media_folder in media_folders {
        let themes_folder = Path::new(&themes_folder);

        let media_folder = Path::new(media_folder);
        let theme = media_folder.join("theme.mp3");

        if theme.exists() && theme.is_file() && !theme.is_symlink(){
            let mut new_path = themes_folder.join(media_folder.file_name().unwrap());
            new_path.set_extension("mp3");
            fs::copy(theme, new_path).expect("Cannot copy.");

            // dear heavens i hate rust rn
            println!("Copied {}", media_folder.file_name().unwrap().to_str().unwrap());
        }
    }
}