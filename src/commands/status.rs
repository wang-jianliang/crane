use crate::components::component::visit_root_solution;
use crate::errors::Error;
use clap::Args;
use git2::Repository;
use git2::Status;
use git2::StatusOptions;

use std::path::Path;
use std::path::PathBuf;

use crate::components::component::walk_components;
use crate::components::component::ComponentArena;
use crate::constants::CRANE_FILE;
use crate::visitors::status_visitor::StatusVisitor;

const TAB_SIZE: usize = 2;

#[derive(Args, Debug)]
pub struct CommandArgs {
    pub dir: Option<PathBuf>,
}

fn write_with_depth(
    output: &mut impl std::io::Write,
    depth: usize,
    content: &str,
) -> Result<(), Error> {
    write!(output, "{:>width$}", "", width = (depth + 1) * TAB_SIZE)?;
    write!(output, "{}", content)?;
    Ok(())
}

fn writeln_with_depth(
    output: &mut impl std::io::Write,
    depth: usize,
    content: &str,
) -> Result<(), Error> {
    write_with_depth(output, depth, content)?;
    writeln!(output, "")?;
    Ok(())
}

fn format_status(status: git2::Status) -> &'static str {
    match status {
        Status::INDEX_NEW | Status::WT_NEW => "new",
        Status::INDEX_MODIFIED | Status::WT_MODIFIED => "modified",
        Status::INDEX_DELETED | Status::WT_DELETED => "deleted",
        Status::INDEX_RENAMED | Status::WT_RENAMED => "renamed",
        Status::INDEX_TYPECHANGE | Status::WT_TYPECHANGE => "typechanged",
        _ => "unknown",
    }
}

fn format_diff(diff: git2::DiffDelta) -> String {
    let old_file = diff.old_file().path();
    let new_file = diff.new_file().path();
    match (old_file, new_file) {
        (Some(old), Some(new)) if old != new => {
            format!("{} -> {}", old.display(), new.display())
        }
        (old, new) => {
            format!("{}", old.or(new).unwrap().display())
        }
    }
}

fn show_status_in_repo<F>(
    output: &mut impl std::io::Write,
    repo_dir: &Path,
    depth: usize,
    path_filter: F,
) -> Result<(), Error>
where
    F: Fn(&Path) -> bool,
{
    log::debug!("show status in repo {:?}", repo_dir);
    let repo = Repository::open(repo_dir)?;

    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut opts))?;

    // show diff between head and index
    let mut filtered_statuses = statuses
        .iter()
        .filter(|e| e.status() != Status::CURRENT && !e.head_to_index().is_none())
        .peekable();

    if filtered_statuses.peek().is_some() {
        writeln_with_depth(output, depth, "Changes to be committed:")?;
        for entry in filtered_statuses {
            writeln_with_depth(
                output,
                depth + 1,
                format!(
                    "{}: {}",
                    format_status(entry.status()),
                    format_diff(entry.head_to_index().unwrap())
                )
                .as_str(),
            )?;
        }
        writeln!(output, "")?;
    }

    let mut filtered_statuses = statuses
        .iter()
        .filter(|e| e.status() != Status::WT_NEW && !e.index_to_workdir().is_none())
        .peekable();
    // show diff between index and worktree
    if filtered_statuses.peek().is_some() {
        writeln_with_depth(output, depth, "Changes not staged:")?;
        for entry in filtered_statuses {
            writeln_with_depth(
                output,
                depth + 1,
                format!(
                    "{}: {}",
                    format_status(entry.status()),
                    format_diff(entry.index_to_workdir().unwrap())
                )
                .as_str(),
            )?;
        }
        writeln!(output, "")?;
    }

    let mut filtered_statuses = statuses
        .iter()
        .filter(|e| {
            e.status() == Status::WT_NEW
                && !e.index_to_workdir().is_none()
                && !path_filter(e.index_to_workdir().unwrap().new_file().path().unwrap())
        })
        .peekable();
    // show untracked paths
    if filtered_statuses.peek().is_some() {
        writeln_with_depth(output, depth, "Changes untracked:")?;
        for entry in filtered_statuses {
            writeln_with_depth(
                output,
                depth + 1,
                format!("{}", format_diff(entry.index_to_workdir().unwrap())).as_str(),
            )?;
        }
        writeln!(output, "")?;
    }

    Ok(())
}

async fn show_status(root_dir: &PathBuf, mut output: impl std::io::Write) -> Result<(), Error> {
    log::debug!("show status in {:?}", root_dir);
    writeln!(output, "")?;

    let root_id = visit_root_solution(
        &StatusVisitor::new(),
        root_dir,
        Some(CRANE_FILE.to_string()),
    )
    .await?;

    // Vec(depth, tail, current_id)
    let mut nodes = vec![(1, true, root_id)];

    while !nodes.is_empty() {
        let (depth, tail, current_id) = nodes.pop().unwrap();
        let comp = ComponentArena::instance().get(current_id).unwrap();

        let bifurcation = if comp.parent_id.is_none() {
            ""
        } else if tail {
            "└─ "
        } else {
            "├─ "
        };
        write!(
            output,
            "{:>width$}{}",
            "",
            bifurcation,
            width = depth * TAB_SIZE
        )?;
        writeln!(output, "{}", comp.name)?;

        // Directories of children should not be seen
        let mut children_names: Vec<String> = vec![];
        if !comp.children.is_empty() {
            for i in 0..comp.children.len() {
                nodes.push((depth + 1, i == 0, comp.children[i]));
                if let Some(child) = ComponentArena::instance().get(comp.children[i]) {
                    children_names.push(child.name.clone());
                }
            }
        }

        show_status_in_repo(&mut output, &comp.target_dir, depth + 1, |path| -> bool {
            children_names.iter().any(|n| path.starts_with(n))
        })?;
    }

    let _ = output.flush();
    Ok(())
}

pub async fn run(args: &CommandArgs) -> Result<(), Error> {
    println!("{:?}", args);

    if let Some(target_dir) = &args.dir {
        show_status(target_dir, std::io::stdout()).await
    } else {
        show_status(&PathBuf::from("."), std::io::stdout()).await
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};
    use tempdir::TempDir;

    use crate::utils::test_utils;
    use test_log::test;

    use super::show_status;

    // #[test]
    #[test(tokio::test)]
    async fn test_show_status() {
        // Create repositories, the structure should be like this:
        // main
        //  ├── sub1
        //    ├── sub1_sub1
        //    └── sub1_sub2
        //  └── sub2
        //    └── sub2_sub1
        let err_msg = "Failed to create temporary directory";
        let main_repo_temp_dir = TempDir::new("main_repo").expect(err_msg);
        // let main_repo_dir = main_repo_temp_dir.into_path();
        std::fs::remove_dir_all("test_status");
        let main_repo_dir = PathBuf::from("test_status");
        let sub1_repo_dir = main_repo_dir.clone().join("sub1");
        let sub1_sub1_repo_dir = sub1_repo_dir.clone().join("sub1_sub1");
        let sub1_sub2_repo_dir = sub1_repo_dir.clone().join("sub1_sub2");
        let sub2_repo_dir = main_repo_dir.clone().join("sub2");
        let sub2_sub1_repo_dir = sub2_repo_dir.clone().join("sub2_sub1");

        // Create repositories
        test_utils::create_git_repo_in_dir(
            sub1_sub1_repo_dir.as_path(),
            &PathBuf::from("README.md"),
            "sub1 sub1",
        )
        .unwrap();
        test_utils::create_git_repo_in_dir(
            sub1_sub2_repo_dir.as_path(),
            &PathBuf::from("README.md"),
            "sub1 sub2",
        )
        .unwrap();

        test_utils::create_git_repo_in_dir(
            sub1_repo_dir.as_path(),
            &PathBuf::from(".crane"),
            &format!(
                r#"deps = {{
    "sub1_sub1": {{"type": "solution", "url": "https://test.git", "branch": "main"}},
    "sub1_sub2": {{"type": "solution", "url": "https://test.git", "branch": "main"}}
}}"#,
            ),
        )
        .unwrap();

        test_utils::create_git_repo_in_dir(
            sub2_sub1_repo_dir.as_path(),
            &PathBuf::from("README.md"),
            "sub2 sub1",
        )
        .unwrap();

        test_utils::create_git_repo_in_dir(
            sub2_repo_dir.as_path(),
            &PathBuf::from(".crane"),
            &format!(
                r#"deps = {{
    "sub2_sub1": {{"type": "solution", "url": "https://test.git", "branch": "main"}},
}}"#,
            ),
        )
        .unwrap();

        test_utils::create_git_repo_in_dir(
            main_repo_dir.as_path(),
            &PathBuf::from(".crane"),
            &format!(
r#"deps = {{
    "sub1": {{"type": "solution", "url": "https://test.git", "branch": "main", "deps_file": ".crane"}},
    "sub2": {{"type": "solution", "url": "https://test.git", "branch": "main", "deps_file": ".crane"}}
}}"#,
            )
        )
        .unwrap();

        // Make some changes
        // main: test2.txt(modified, not staged)
        let res = test_utils::modify_file_in_repo(
            &main_repo_dir,
            &PathBuf::from("test2.txt"),
            "test\n",
            true,
            true,
            true,
        );
        assert!(res.is_ok());
        let res = test_utils::modify_file_in_repo(
            &main_repo_dir,
            &PathBuf::from("test2.txt"),
            "test\n",
            true,
            false,
            false,
        );
        assert!(res.is_ok());
        // main: test.txt(new, to be committed)
        let res = test_utils::modify_file_in_repo(
            &main_repo_dir,
            &PathBuf::from("test.txt"),
            "test\n",
            true,
            true,
            false,
        );
        assert!(res.is_ok());
        // main: test3.txt(new, untracked)
        let res = test_utils::modify_file_in_repo(
            &main_repo_dir,
            &PathBuf::from("test3.txt"),
            "test\n",
            true,
            false,
            false,
        );
        assert!(res.is_ok());

        // sub1: test.txt(modified, not staged)
        let res = test_utils::modify_file_in_repo(
            &sub1_repo_dir,
            &PathBuf::from("test.txt"),
            "test\n",
            true,
            true,
            true,
        );
        assert!(res.is_ok());
        let res = test_utils::modify_file_in_repo(
            &sub1_repo_dir,
            &PathBuf::from("test.txt"),
            "test\n",
            true,
            false,
            false,
        );
        assert!(res.is_ok());
        // sub1: test2.txt(untracked)
        let res = test_utils::modify_file_in_repo(
            &sub1_repo_dir,
            &PathBuf::from("test2.txt"),
            "test\n",
            true,
            false,
            false,
        );
        assert!(res.is_ok());

        // sub1 sub1: test.txt(deleted, to be committed)
        let res = test_utils::modify_file_in_repo(
            &sub1_sub1_repo_dir,
            &PathBuf::from("test.txt"),
            "test\n",
            true,
            true,
            true,
        );
        assert!(res.is_ok());
        let res = test_utils::delete_file_from_repo(
            &sub1_sub1_repo_dir,
            &PathBuf::from("test.txt"),
            true,
            false,
        );
        assert!(res.is_ok());

        // sub2 sub1: test2.txt(modified, not staged)
        let res = test_utils::modify_file_in_repo(
            &sub2_sub1_repo_dir,
            &PathBuf::from("test.txt"),
            "test\n",
            true,
            true,
            true,
        );
        assert!(res.is_ok());
        let res = test_utils::modify_file_in_repo(
            &sub2_sub1_repo_dir,
            &PathBuf::from("test2.txt"),
            "test\n",
            true,
            true,
            true,
        );
        assert!(res.is_ok());
        let res = test_utils::modify_file_in_repo(
            &sub2_sub1_repo_dir,
            &PathBuf::from("test3.txt"),
            "test\n",
            true,
            true,
            true,
        );
        assert!(res.is_ok());
        let res = test_utils::modify_file_in_repo(
            &sub2_sub1_repo_dir,
            &PathBuf::from("test2.txt"),
            "test\n",
            true,
            false,
            false,
        );
        assert!(res.is_ok());

        // sub2 sub1: test3.txt(deleted, not staged)
        let res = test_utils::delete_file_from_repo(
            &sub2_sub1_repo_dir,
            &PathBuf::from("test3.txt"),
            false,
            false,
        );
        assert!(res.is_ok());

        // sub2 sub1: test.txt(modified, to be committed)
        let res = test_utils::modify_file_in_repo(
            &sub2_sub1_repo_dir,
            &PathBuf::from("test.txt"),
            "test\n",
            true,
            true,
            false,
        );
        assert!(res.is_ok());

        fs::remove_file("test3.txt");
        // sub2 sub1: test4.txt(untracked)
        let res = test_utils::modify_file_in_repo(
            &sub2_sub1_repo_dir,
            &PathBuf::from("test4.txt"),
            "test\n",
            true,
            false,
            false,
        );
        assert!(res.is_ok());

        let mut output = Vec::new();
        show_status(&main_repo_dir, &mut output).await.unwrap();
        let output_str = std::str::from_utf8(output.as_slice()).unwrap();
        println!("{}", output_str);

        let expected_output = "
  (main)
      Changes to be committed:
        new: test.txt

      Changes not staged:
        modified: test2.txt

      Changes untracked:
        test3.txt

    └─ sub2 (clean)
        └─ sub2_sub1
          Changes to be commited:
            modified: test.txt

          Changes not staged:
            modified: test2.txt
            deleted: test3.txt

          Changes untracked:
            test4.txt

    └─ sub1
      Changes not staged:
        modified: test.txt

      Changes untracked:
        test2.txt

      └─ sub1_sub1
        Changes to be committed:
          deleted: test.txt

      └─ sub1_sub2 (clean)
";
        assert_eq!(output_str, expected_output);
    }
}
