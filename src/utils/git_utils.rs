use git2::{AnnotatedCommit, FetchOptions, ObjectType, ProxyOptions, RemoteCallbacks, Repository};
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};
use url::Url;

use crate::errors::Error;

pub fn get_repo_name(repo_url: &str) -> Option<String> {
    let repo_url = if repo_url.starts_with("git@") {
        repo_url.replace(":", "/").replace("git@", "https://")
    } else {
        repo_url.to_string()
    };

    let url = Url::parse(repo_url.as_str()).ok()?;
    let segments: Vec<&str> = url
        .path_segments()?
        .filter(|s| *s != ".git" && *s != "")
        .collect();
    let repo_name = segments.last()?;

    let repo_name = if repo_name.ends_with(".git") {
        &repo_name[..repo_name.len() - 4]
    } else {
        repo_name
    };

    Some(repo_name.to_string())
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

pub fn get_remote_default_branch(url: &str, remote_name: Option<&str>) -> Option<String> {
    let temp_dir = env::temp_dir();
    let temp_repo_path = temp_dir.join("repo");
    let repo = Repository::init_bare(temp_repo_path.to_str().unwrap()).ok()?;

    let remote = match repo.find_remote(remote_name.unwrap_or("origin")) {
        Ok(r) => r,
        Err(_) => match repo.remote(remote_name?, url) {
            Ok(remote) => remote,
            Err(_) => return None,
        },
    };
    match remote.default_branch() {
        Ok(branch) => Some(String::from_utf8(branch.to_vec()).unwrap()),
        Err(_) => None,
    }
}

pub fn fetch_repository<'a>(
    repo: &'a Repository,
    url: &'a str,
    refs: &[&str],
    remote_name: Option<&str>,
) -> Result<AnnotatedCommit<'a>, Error> {
    log::debug!("set remote url to {}", url);
    let remote_name = remote_name.unwrap_or("origin");
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

    log::debug!("fetch refspec \"{:?}\"", refs);
    // Check if the refspec is valid
    remote.fetch(refs, Some(&mut fetch_option), None)?;
    remote.disconnect()?;

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    Ok(repo.reference_to_annotated_commit(&fetch_head)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils;
    use git2::Repository;
    use tempdir::TempDir;

    #[test]
    fn test_get_repo_name() {
        assert_eq!(
            get_repo_name("https://github.com/user/repo.git"),
            Some("repo".to_string())
        );
        assert_eq!(
            get_repo_name("https://github.com/user/repo"),
            Some("repo".to_string())
        );
        assert_eq!(
            get_repo_name("https://github.com/user/repo/"),
            Some("repo".to_string())
        );
        assert_eq!(
            get_repo_name("git@github.com/user/repo.git"),
            Some("repo".to_string())
        );
        assert_eq!(
            get_repo_name("file://github.com/user/repo.git"),
            Some("repo".to_string())
        );
        assert_eq!(
            get_repo_name("file:///tmp/repo/.git"),
            Some("repo".to_string())
        );
        assert_eq!(get_repo_name("not a url"), None);
    }

    #[test]
    fn test_fetch_repository_positive() {
        let remote_repo_dir =
            TempDir::new("remote_repo").expect("Failed to create temporary directory");
        let repo_url = test_utils::create_git_repo_in_dir(
            remote_repo_dir.path(),
            &PathBuf::from("test.txt"),
            "Hello, world!",
        )
        .unwrap();

        let temp_dir = TempDir::new("test_repo").expect("Failed to create temporary directory");
        let target_dir = temp_dir.path().to_path_buf();

        let repo = Repository::init(&target_dir).unwrap();
        let result = fetch_repository(&repo, repo_url.as_str(), &["main"], Some("origin"));
        assert!(result.is_ok());

        // The temporary directory will be automatically deleted when `temp_dir` goes out of scope
    }
}
