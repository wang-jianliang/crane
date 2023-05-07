use crate::utils;
use std::path::PathBuf;

// Check if the directory is in a git repository. If dir is not set,
// check the current directory.
pub fn is_git_repo(dir: Option<&PathBuf>) -> bool {
    let current_dir = PathBuf::from(".");
    // Add default argument for current directory
    let dir = match dir {
        Some(dir) => dir,
        None => &current_dir,
    };

    // Use "git rev-parse" to check if the directory is a git repository
    let output = utils::process::Command::new("git")
        .arg("rev-parse")
        .current_dir(dir)
        .output();

    // Check if the command was successful
    if let Ok(_) = output {
        return true;
    }
    false
}
