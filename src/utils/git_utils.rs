use git2::{FetchOptions, ObjectType, ProxyOptions, RemoteCallbacks, Repository};
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};
use url::Url;

use crate::errors::Error;

fn is_valid_refspec(refspec: &str) -> bool {
    let parts: Vec<&str> = refspec.split(':').collect();
    if parts.len() > 2 {
        return false;
    }

    let src = parts[0];
    if parts.len() == 2 {
        let dst = parts[1];
        if !dst.starts_with("refs/") {
            return false;
        }
    }

    src.starts_with("refs/")
        || src.starts_with("+refs")
        || src == "HEAD"
        || src.chars().all(char::is_alphanumeric)
        || (src.len() == 40 && src.chars().all(|c| c.is_digit(16)))
}

fn get_git_dir_path(repo_path: &Path) -> std::io::Result<PathBuf> {
    if repo_path.join(".git").is_dir() {
        Ok(repo_path.join(".git"))
    } else {
        let git_file_content = read_to_string(repo_path.join(".git"))?;
        if git_file_content.starts_with("gitdir: ") {
            let relative_git_dir_path = git_file_content.trim_start_matches("gitdir: ").trim();
            Ok(repo_path.join(relative_git_dir_path))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid .git file",
            ))
        }
    }
}

pub fn open_or_create_repo(path: &Path) -> Result<Repository, Error> {
    match Repository::open(&path) {
        Ok(repo) => Ok(repo),
        Err(_) => match Repository::init(&path) {
            Ok(repo) => Ok(repo),
            Err(err) => return Err(err.into()),
        },
    }
}

pub fn add_alternate(repo_path: &Path, alternate_path: &Path) -> std::io::Result<()> {
    log::debug!(
        "add alternate: {} -> {}",
        repo_path.to_str().unwrap(),
        alternate_path.to_str().unwrap()
    );
    let git_dir_path = get_git_dir_path(repo_path)?;
    let info_dir = git_dir_path.join("objects/info");

    if !info_dir.exists() {
        fs::create_dir_all(&info_dir)?;
    }

    let mut alternates_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(info_dir.join("alternates"))?;
    writeln!(
        alternates_file,
        "{}",
        alternate_path.join("objects").display()
    )
}

pub fn checkout_to_target(repo: &Repository, target: &str) -> Result<(), Error> {
    // Parse target as a branch
    if let Ok(branch) = repo.find_branch(target, git2::BranchType::Local) {
        let commit = branch.get().peel_to_commit()?;
        let object = commit.into_object();
        repo.checkout_tree(&object, None)?;
        repo.set_head(branch.get().name().unwrap())?;
    } else {
        // If target is not a branch, try to parse it as a commit
        if let Ok(oid) = git2::Oid::from_str(target) {
            let object = repo.find_object(oid, Some(ObjectType::Commit))?;
            repo.checkout_tree(&object, None)?;
            repo.set_head_detached(oid)?;
        } else {
            return Err(Error {
                message: format!(
                    "Invalid target: must be a branch name or commit id, got {}",
                    target
                ),
            });
        }
    }
    Ok(())
}

pub fn fetch_repository(repo: &Repository, url: &str, refspec: &str) -> Result<(), Error> {
    log::debug!("set remote url to {}", url);
    let remote_name = "origin";
    let mut remote = match repo.find_remote(remote_name) {
        Ok(r) => r,
        Err(_) => match repo.remote(remote_name, url) {
            Ok(remote) => remote,
            Err(err) => return Err(err.into()),
        },
    };

    let mut fetch_option = FetchOptions::new();

    // Set up proxy
    let parsed_url = match Url::parse(&url) {
        Ok(u) => u,
        Err(_) => {
            return Err(Error {
                message: format!("invalid remote url {}", url),
            })
        }
    };
    let schema = parsed_url.scheme().to_owned();
    match env::var(schema.clone() + "_proxy").or(env::var(schema.clone().to_uppercase() + "_PROXY"))
    {
        Ok(proxy_url) => {
            log::debug!("operations will be under a proxy: {}", proxy_url);
            let mut proxy_option = ProxyOptions::new();
            proxy_option.url(&proxy_url);
            fetch_option.proxy_options(proxy_option);
        }
        Err(_) => {}
    }

    // Set up authentication
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        git2::Cred::ssh_key(
            username_from_url.unwrap(),
            Some(home::home_dir().unwrap().join(".ssh/id_rsa.pub").as_path()),
            home::home_dir().unwrap().join(".ssh/id_rsa").as_path(),
            None,
        )
    });
    fetch_option.remote_callbacks(callbacks);

    log::debug!("fetch refspec \"{}\"", refspec);
    // Check if the refspec is valid
    if is_valid_refspec(refspec) {
        remote.fetch(&[refspec], Some(&mut fetch_option), None)?;
    } else {
        return Err(Error {
            message: String::from("Invalid refspec"),
        });
    }
    remote.disconnect()?;

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::{Repository, RepositoryInitOptions, Signature};
    use std::{error::Error, fs::File, io::Write, path::Path};
    use tempdir::TempDir;

    // Function to create a git repository with a single commit in the specified directory
    fn create_git_repo_in_dir(repo_path: &Path) -> Result<String, Box<dyn Error>> {
        // Initialize a new git repository
        let mut opts = RepositoryInitOptions::new();
        opts.initial_head("main"); // Set the initial branch to "main"
        opts.bare(false); // Create a non-bare repository
        let repo = Repository::init_opts(repo_path, &opts)?;

        // Create a new file in the repository
        let mut file = File::create(repo_path.join("test.txt"))?;
        write!(file, "Hello, world!")?;

        // Add the new file to the git index
        let mut index = repo.index()?;
        index.add_path(Path::new("test.txt"))?;
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

    #[test]
    fn test_fetch_repository_positive() {
        let remote_repo_dir =
            TempDir::new("remote_repo").expect("Failed to create temporary directory");
        let repo_url = create_git_repo_in_dir(remote_repo_dir.path()).unwrap();

        let temp_dir = TempDir::new("test_repo").expect("Failed to create temporary directory");
        let target_dir = temp_dir.path().to_path_buf();

        let repo = Repository::init(&target_dir).unwrap();
        let result = fetch_repository(&repo, repo_url.as_str(), "refs/heads/main");
        assert!(result.is_ok());
        // Add assertions to verify the expected behavior

        // The temporary directory will be automatically deleted when `temp_dir` goes out of scope
    }

    #[test]
    fn test_fetch_repository_negative_invalid_url() {
        let temp_dir = TempDir::new("test_repo").expect("Failed to create temporary directory");
        let target_dir = temp_dir.path().to_path_buf();

        let repo = Repository::init(&target_dir).unwrap();
        let result = fetch_repository(&repo, "invalid_url", "refs/heads/master");
        assert!(result.is_err());
        // Add assertions to verify the expected behavior

        // The temporary directory will be automatically deleted when `temp_dir` goes out of scope
    }

    #[test]
    fn test_fetch_repository_negative_invalid_refspec() {
        let remote_repo_dir =
            TempDir::new("remote_repo").expect("Failed to create temporary directory");
        let repo_url = create_git_repo_in_dir(remote_repo_dir.path()).unwrap();

        let temp_dir = TempDir::new("test_repo").expect("Failed to create temporary directory");
        let target_dir = temp_dir.path().to_path_buf();

        let repo = Repository::init(&target_dir).unwrap();
        let result = fetch_repository(&repo, repo_url.as_str(), "invalid_refspec");
        assert!(result.is_err());
        // Add assertions to verify the expected behavior

        // The temporary directory will be automatically deleted when `temp_dir` goes out of scope
    }

    // Add more test cases to cover different scenarios
}
