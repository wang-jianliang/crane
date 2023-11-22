mod tests {
    use assert_cmd::prelude::*;
    use assert_fs::{prelude::*, TempDir};
    use git2::Repository;
    use predicates::prelude::*;
    use std::{path::PathBuf, process::Command};
    use test_log::test;

    use crane::utils::{fs::copy_dir_to, test_utils};

    #[test(tokio::test)]
    async fn test_sync_simple_with_url_and_without_dir() -> Result<(), Box<dyn std::error::Error>> {
        // Create bare repo
        let source_repo_dir = TempDir::new()?;
        let _ = test_utils::create_git_repo_in_dir(
            &source_repo_dir.path(),
            &PathBuf::from("README.md"),
            "test",
        )
        .unwrap();

        let target_dir = source_repo_dir
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        log::debug!(
            "Sync from {} to {}",
            source_repo_dir.path().display(),
            target_dir
        );

        let mut cmd = Command::cargo_bin("crane")?;
        cmd.arg("sync")
            .arg("--url")
            .arg(format!("file://{}/.git", source_repo_dir.path().display()))
            .arg("--branch")
            .arg("main");

        let workdir = &TempDir::new()?;
        cmd.current_dir(workdir)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Sync solution to {}",
                workdir.path().join(target_dir).display()
            )));

        workdir.child(target_dir).assert(predicate::path::exists());
        workdir
            .child(target_dir)
            .child("README.md")
            .assert(predicate::path::exists());
        workdir
            .child(target_dir)
            .child(".git")
            .assert(predicate::path::exists());

        Ok(())
    }

    #[test(tokio::test)]
    async fn test_sync_simple_with_url_and_dir() -> Result<(), Box<dyn std::error::Error>> {
        // Create bare repo
        let source_repo_dir = TempDir::new()?;
        let _ = test_utils::create_git_repo_in_dir(
            &source_repo_dir.path(),
            &PathBuf::from("README.md"),
            "test",
        )
        .unwrap();

        let target_dir = "test_sync_simple_with_url_and_dir";

        log::debug!(
            "Sync from {} to {}",
            source_repo_dir.path().display(),
            target_dir
        );

        let mut cmd = Command::cargo_bin("crane")?;
        cmd.arg("sync")
            .arg("--url")
            .arg(format!("file://{}/.git", source_repo_dir.path().display()))
            .arg(target_dir)
            .arg("--branch")
            .arg("main");

        let workdir = &TempDir::new()?;
        cmd.current_dir(workdir)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Sync solution to {}",
                workdir.path().join(target_dir).display()
            )));

        workdir.child(target_dir).assert(predicate::path::exists());
        workdir
            .child(target_dir)
            .child("README.md")
            .assert(predicate::path::exists());
        workdir
            .child(target_dir)
            .child(".git")
            .assert(predicate::path::exists());

        Ok(())
    }

    #[test(tokio::test)]
    async fn test_sync_simple_without_url_but_with_dir() -> Result<(), Box<dyn std::error::Error>> {
        // Create a source repository with 1 commit
        let source_repo_dir = TempDir::new()?;
        let _ = test_utils::create_git_repo_in_dir(
            &source_repo_dir.path(),
            &PathBuf::from("README.md"),
            "test",
        )
        .unwrap();

        let workdir = &TempDir::new()?;
        let target_dir = "test_sync_simple_without_url_and_dir";

        // Sync the source repository to target directory
        log::debug!(
            "Sync from {} to {}",
            source_repo_dir.path().display(),
            target_dir
        );
        let mut cmd = Command::cargo_bin("crane")?;
        cmd.arg("sync")
            .arg("--url")
            .arg(format!("file://{}/.git", source_repo_dir.path().display()))
            .arg(target_dir)
            .arg("--branch")
            .arg("main");

        cmd.current_dir(workdir)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Sync solution to {}",
                workdir.path().join(target_dir).display()
            )));

        // Add another commit to source repository
        test_utils::modify_file_in_repo(
            &source_repo_dir,
            &PathBuf::from("README.2.md"),
            "test",
            true,
            true,
            true,
        )?;

        // Sync again without a url
        let mut cmd = Command::cargo_bin("crane")?;
        cmd.arg("sync").arg(target_dir);

        cmd.current_dir(workdir)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Sync solution to {}",
                workdir.path().join(target_dir).display()
            )));

        workdir.child(target_dir).assert(predicate::path::exists());
        workdir
            .child(target_dir)
            .child("README.2.md")
            .assert(predicate::path::exists());
        workdir
            .child(target_dir)
            .child(".git")
            .assert(predicate::path::exists());

        Ok(())
    }

    #[test(tokio::test)]
    async fn test_sync_simple_without_url_and_without_dir() -> Result<(), Box<dyn std::error::Error>>
    {
        // Create a source repository with 1 commit
        let source_repo_dir = TempDir::new()?;
        let _ = test_utils::create_git_repo_in_dir(
            &source_repo_dir.path(),
            &PathBuf::from("README.md"),
            "test",
        )
        .unwrap();

        let workdir = &TempDir::new()?;
        let target_dir = "test_sync_simple_without_url_and_without_dir";

        // Copy source repo to target_dir
        copy_dir_to(&source_repo_dir.path(), &workdir.join(target_dir))?;

        // Set remote url
        let repo = Repository::open(&workdir.join(target_dir))?;
        repo.remote(
            "origin",
            format!("file://{}/.git", source_repo_dir.path().display()).as_str(),
        )?;

        // Add another commit to source repository
        test_utils::modify_file_in_repo(
            &source_repo_dir,
            &PathBuf::from("README.2.md"),
            "test",
            true,
            true,
            true,
        )?;

        // Sync again without a url
        let mut cmd = Command::cargo_bin("crane")?;
        cmd.arg("sync");

        cmd.current_dir(&workdir.join(target_dir))
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Sync solution to {}",
                workdir.path().join(target_dir).display()
            )));

        workdir.child(target_dir).assert(predicate::path::exists());
        workdir
            .child(target_dir)
            .child("README.2.md")
            .assert(predicate::path::exists());
        workdir
            .child(target_dir)
            .child(".git")
            .assert(predicate::path::exists());

        Ok(())
    }

    #[test(tokio::test)]
    async fn test_sync_with_commit_id() -> Result<(), Box<dyn std::error::Error>> {
        // Create a source repository with 1 commit
        let source_repo_dir = TempDir::new()?;
        let _ = test_utils::create_git_repo_in_dir(
            &source_repo_dir.path(),
            &PathBuf::from("README.md"),
            "test",
        )
        .unwrap();

        let workdir = &TempDir::new()?;
        let target_dir = "test_sync_simple_without_url_and_without_dir";

        // Copy source repo to target_dir
        copy_dir_to(&source_repo_dir.path(), &workdir.join(target_dir))?;

        // Set remote url
        let target_repo = Repository::open(&workdir.join(target_dir))?;
        target_repo.remote(
            "origin",
            format!("file://{}/.git", source_repo_dir.path().display()).as_str(),
        )?;

        // Add 2nd commit to source repository
        test_utils::modify_file_in_repo(
            &source_repo_dir,
            &PathBuf::from("README.2.md"),
            "test",
            true,
            true,
            true,
        )?;
        // Get the head commit in source repo
        let source_repo = Repository::open(&source_repo_dir)?;
        let source_head = source_repo.head()?;
        let source_commit = source_head.target().unwrap();

        // Add another commit to source repository
        test_utils::modify_file_in_repo(
            &source_repo_dir,
            &PathBuf::from("README.3.md"),
            "test",
            true,
            true,
            true,
        )?;

        // Sync to a commit
        let mut cmd = Command::cargo_bin("crane")?;
        cmd.arg("sync")
            .arg("--commit")
            .arg(&source_commit.to_string());

        cmd.current_dir(&workdir.join(target_dir))
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Sync solution to {}",
                workdir.path().join(target_dir).display()
            )));

        workdir.child(target_dir).assert(predicate::path::exists());
        workdir
            .child(target_dir)
            .child("README.2.md")
            .assert(predicate::path::exists());
        workdir
            .child(target_dir)
            .child("README.3.md")
            .assert(predicate::path::missing());

        let target_head = target_repo.head()?;
        assert_eq!(source_commit, target_head.target().unwrap());

        Ok(())
    }

    #[test]
    fn test_sync_nested_repositories() -> Result<(), Box<dyn std::error::Error>> {
        // main
        //  |-sub1
        //    |-sub1_sub1
        //    |-sub1_sub2
        //  |-sub2
        //    |-sub2_sub1
        let err_msg = "Failed to create temporary directory";
        let main_repo_dir = TempDir::new().expect(err_msg);
        let sub1_repo_dir = TempDir::new().expect(err_msg);
        let sub1_sub1_repo_dir = TempDir::new().expect(err_msg);
        let sub1_sub2_repo_dir = TempDir::new().expect(err_msg);
        let sub2_repo_dir = TempDir::new().expect(err_msg);
        let sub2_sub1_repo_dir = TempDir::new().expect(err_msg);

        // Create repositories
        let sub1_sub1_repo = test_utils::create_git_repo_in_dir(
            sub1_sub1_repo_dir.path(),
            &PathBuf::from("README.md"),
            "sub1 sub1",
        )
        .unwrap();
        let sub1_sub2_repo = test_utils::create_git_repo_in_dir(
            sub1_sub2_repo_dir.path(),
            &PathBuf::from("README.md"),
            "sub1 sub2",
        )
        .unwrap();

        let sub1_repo = test_utils::create_git_repo_in_dir(
            sub1_repo_dir.path(),
            &PathBuf::from(".crane"),
            &format!(
                r#"deps = {{ \
                "sub1_sub1": {{"type": "solution", "url": "{}", "branch": "main"}}, \
                "sub1_sub2": {{"type": "solution", "url": "{}", "branch": "main"}} \
            }}"#,
                sub1_sub1_repo, sub1_sub2_repo
            ),
        )
        .unwrap();

        let sub2_sub1_repo = test_utils::create_git_repo_in_dir(
            sub2_sub1_repo_dir.path(),
            &PathBuf::from("README.md"),
            "sub2 sub1",
        )
        .unwrap();

        let sub2_repo = test_utils::create_git_repo_in_dir(
            sub2_repo_dir.path(),
            &PathBuf::from(".crane"),
            &format!(
                r#"deps = {{ \
                "sub2_sub1": {{"type": "solution", "url": "{}", "branch": "main"}}, \
            }}"#,
                sub2_sub1_repo
            ),
        )
        .unwrap();

        test_utils::create_git_repo_in_dir(
            main_repo_dir.path(),
            &PathBuf::from(".crane"),
            &format!(
                r#"deps = {{ \
                    "sub1": {{"type": "solution", "url": "{}", "branch": "main", "deps_file": ".crane" }}, \
                    "sub2": {{"type": "solution", "url": "{}", "branch": "main", "deps_file": ".crane" }} \
                }}"#,
                sub1_repo, sub2_repo
            ),
        )
        .unwrap();
        // TODO: implement the test logic

        let workdir = &TempDir::new()?;
        let target_dir = "test_sync_nested_repositories";
        // Sync to a commit
        let mut cmd = Command::cargo_bin("crane")?;
        cmd.arg("sync")
            .arg("--url")
            .arg(format!("file://{}/.git", main_repo_dir.path().display()))
            .arg(target_dir)
            .arg("--branch")
            .arg("main");

        cmd.current_dir(&workdir)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Sync solution to {}",
                workdir.path().join(target_dir).display()
            )));

        let expected_sub1_repo_dir = workdir.child(&target_dir).child("sub1");
        expected_sub1_repo_dir.assert(predicate::path::exists());
        let expected_sub1_sub1_repo_dir = expected_sub1_repo_dir.child("sub1_sub1");
        expected_sub1_sub1_repo_dir.assert(predicate::path::exists());
        expected_sub1_sub1_repo_dir
            .child("README.md")
            .assert(predicate::path::exists());

        let expected_sub2_repo_dir = workdir.child(&target_dir).child("sub2");
        expected_sub2_repo_dir.assert(predicate::path::exists());
        expected_sub2_repo_dir
            .child("sub2_sub1")
            .child("README.md")
            .assert(predicate::path::exists());

        Ok(())
    }
}
