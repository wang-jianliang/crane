use std::path::PathBuf;
use tempdir::TempDir;

use crane::utils::test_utils;

#[test]
fn test_sync_nested_repositories() {
    // main
    //  |-sub1
    //    |-sub1_sub1
    //    |-sub1_sub2
    //  |-sub2
    //    |-sub2_sub1
    let err_msg = "Failed to create temporary directory";
    let main_repo_dir = TempDir::new("main_repo").expect(err_msg);
    let sub1_repo_dir = TempDir::new("sub1_repo").expect(err_msg);
    let sub1_sub1_repo_dir = TempDir::new("sub1_sub1_repo").expect(err_msg);
    let sub1_sub2_repo_dir = TempDir::new("sub1_sub2_repo").expect(err_msg);
    let sub2_repo_dir = TempDir::new("sub2_repo").expect(err_msg);
    let sub2_sub1_repo_dir = TempDir::new("sub2_sub1_repo").expect(err_msg);

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
