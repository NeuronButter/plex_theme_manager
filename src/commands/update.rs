use crate::lib::config::Config;
use crate::lib::helpers;
use crate::commands::derive::derive;

use std::os::unix::fs::symlink;
use std::fs;

pub fn updater(contents: Config) {
    println!("Deriving theme.mp3 files, and linking them into your theme folder.");

    // Check if folders are writable.
    let root_folders = contents.folders;
    helpers::check_folders_permissions(&root_folders);
    helpers::check_folders_permissions(&vec![contents.themes.clone()]);

    // Get vector of all media folders
    let media_folders = helpers::determine_media_folders(root_folders);
    let theme_songs = helpers::determine_theme_songs(contents.themes.clone());

    // Derive first
    derive(&media_folders, contents.themes.clone());

    // Then update
    for media_folder in &media_folders {
        // If media_folder exists in theme_songs
        let media_folder = media_folder;
        for theme_song in &theme_songs {
            if theme_song.file_stem().unwrap() == media_folder.file_name().unwrap() {
                match fs::remove_file(media_folder.join("theme.mp3")) {
                    Ok (_) => {},
                    _ => {}
                }
                symlink(theme_song, media_folder.join("theme.mp3")).expect("Cannot symlink.");
                println!("Symlinked {}", media_folder.file_name().unwrap().to_str().unwrap());
            }
        }
    }
}