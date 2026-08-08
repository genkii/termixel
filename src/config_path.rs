use directories::ProjectDirs;
use std::path::PathBuf;

pub fn config_dir() -> Option<PathBuf> {
    ProjectDirs::from("", "", "termixel").map(|dirs| dirs.config_dir().to_path_buf())
}
