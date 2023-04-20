use std::path::PathBuf;
use std::process::Command;

// Check if the directory is in a git repository. If dir is not set, 
// check the current directory.
pub fn is_git_repo(dir: &str) -> bool {
    // Add default argument for current directory
    let dir = if dir.is_empty() {
        "."
    } else {
        dir
    };

    // Use "git rev-parse" to check if the directory is a git repository
    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .current_dir(dir)
        .output();

    // Check if the command was successful
    if let Ok(output) = output {
        // Check if the output was empty
        if output.stdout.is_empty() {
            return false;
        }
        return true;
    }
    false
}
