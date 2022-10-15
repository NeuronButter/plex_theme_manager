use crate::lib::config::Config;
use crate::lib::helpers;

use std::fs;
use std::path::Path;

pub fn deleter(contents: Config) {
    println!("Deleting all theme.mp3 copies within media folders.");

    // Check if folders are writable.
    let root_folders = contents.folders;
    helpers::check_folders_permissions(&root_folders);
    helpers::check_folders_permissions(&vec![contents.themes.clone()]);

    // Get vector of all media folders
    let media_folders = helpers::determine_media_folders(root_folders);

    // If theme.mp3 exists, delete it.
    for media_folder in &media_folders {
        let theme = Path::new(media_folder).join("theme.mp3");
        if theme.exists() && theme.is_file() {
            fs::remove_file(theme).expect("Cannot delete theme.mp3");
        }
    }
}
