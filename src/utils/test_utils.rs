use git2::{Repository, RepositoryInitOptions, Signature};
use std::fs::{self, OpenOptions};
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
    index.write()?;

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

pub fn modify_file_in_repo(
    repo_path: &Path,
    file_path: &PathBuf,
    content: &str,
    append: bool,
    add: bool,
    commit: bool,
) -> Result<(), Box<dyn Error>> {
    let repo = Repository::open(repo_path)?;
    let full_file_path = repo_path.join(file_path);
    let mut file;
    if full_file_path.exists() {
        file = OpenOptions::new()
            .write(true)
            .append(append)
            .open(full_file_path)?;
    } else {
        file = File::create(repo_path.join(file_path))?;
    }
    write!(file, "{}", content)?;

    if !add && !commit {
        return Ok(());
    }
    let mut index = repo.index()?;
    index.add_path(&file_path)?;
    let oid = index.write_tree()?;
    index.write()?;

    if !commit {
        return Ok(());
    }
    let head = repo.head()?;
    let parent = head.peel_to_commit()?;
    let tree = repo.find_tree(oid)?;
    let signature = Signature::now("test", "test@example.com")?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        format!("Add file {:?}", file_path).as_str(),
        &tree,
        &[&parent],
    )?;
    Ok(())
}

pub fn delete_file_from_repo(
    repo_path: &Path,
    file_path: &PathBuf,
    add: bool,
    commit: bool,
) -> Result<(), Box<dyn Error>> {
    let repo = Repository::open(repo_path)?;
    let full_file_path = repo_path.join(file_path);

    fs::remove_file(full_file_path)?;

    if !add && !commit {
        return Ok(());
    }
    let mut index = repo.index()?;
    index.remove_path(&file_path)?;
    let oid = index.write_tree()?;
    index.write()?;

    if !commit {
        return Ok(());
    }
    let head = repo.head()?;
    let parent = head.peel_to_commit()?;
    let tree = repo.find_tree(oid)?;
    let signature = Signature::now("test", "test@example.com")?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        format!("Delete file {:?}", file_path).as_str(),
        &tree,
        &[&parent],
    )?;
    Ok(())
}
