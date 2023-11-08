mod tests {
    use assert_cmd::prelude::*;
    use assert_fs::{prelude::*, TempDir};
    use predicates::prelude::*;
    use std::{path::PathBuf, process::Command};
    use test_log::test;

    use crane::utils::test_utils;

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

    #[test]
    fn test_sync_nested_repositories() {
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
        let _sub2_repo_dir = TempDir::new().expect(err_msg);
        let _sub2_sub1_repo_dir = TempDir::new().expect(err_msg);

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
                r#"{{ \
                "sub1_sub1": {{"type": "solution", "url": "{}", "branch": "main"}}, \
                "sub1_sub2": {{"type": "solution", "url": "{}", "branch": "main"}} \
            }}"#,
                sub1_sub1_repo, sub1_sub2_repo
            ),
        )
        .unwrap();

        test_utils::create_git_repo_in_dir(
            main_repo_dir.path(),
            &PathBuf::from(".crane"),
            &format!(
                r#"{{"sub1": {{"type": "solution", "url": "{}", "branch": "main"}} }}"#,
                sub1_repo
            ),
        )
        .unwrap();
        // TODO: implement the test logic
    }
}
