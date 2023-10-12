use git2::{Repository, RepositoryInitOptions, Signature};
use std::{
    error::Error,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

// Function to create a git repository with a single commit in the specified directory
pub fn create_git_repo_in_dir(
    repo_path: &Path,
    file_path: &PathBuf,
    content: &str,
) -> Result<String, Box<dyn Error>> {
    // Initialize a new git repository
    let mut opts = RepositoryInitOptions::new();
    opts.initial_head("main"); // Set the initial branch to "main"
    opts.bare(false); // Create a non-bare repository
    let repo = Repository::init_opts(repo_path, &opts)?;

    // Create a new file in the repository
    let mut file = File::create(repo_path.join(file_path))?;
    write!(file, "{}", content)?;

    // Add the new file to the git index
    let mut index = repo.index()?;
    index.add_path(&file_path)?;
    let oid = index.write_tree()?;

    // Create a new commit
    let tree = repo.find_tree(oid)?;
    let signature = Signature::now("test", "test@example.com")?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )?;

    // Return the local clone URL of the repository
    Ok(format!("file://{}", repo_path.to_string_lossy()))
}
