use std::fs;
use std::path::PathBuf;
use std::process::exit;

pub fn check_folders_permissions(root_folders: &Vec<String>) {
    for root_folder in root_folders {
        if fs::metadata(root_folder)
            .expect("Error in inspecting folder")
            .permissions()
            .readonly()
        {
            println!("Folder {} unwritable.", root_folder);
            exit(70);
        }
    }
}

pub fn determine_media_folders(root_folders: Vec<String>) -> Vec<PathBuf> {
    let mut new_vector: Vec<PathBuf> = Vec::new();
    for root_folder in &root_folders {
        for entry in fs::read_dir(root_folder) {
            entry.for_each(|path| {
                let path = path.expect("Cannot read file or directory.");
                if path.path().is_dir() {
                    new_vector.push(path.path());
                }
            })
        }
    }

    new_vector
}

pub fn determine_theme_songs(theme_folder: String) -> Vec<PathBuf> {
    let mut theme_paths: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(&theme_folder) {
        entry.for_each(|path| {
            let path = path.expect("Cannot read file or directory.");
            if path.path().is_file() && path.path().extension().unwrap() == "mp3" {
                theme_paths.push(path.path());
            }
        })
    }

    theme_paths
}
