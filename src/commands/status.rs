use crate::errors::Error;
use clap::Args;

use std::path::PathBuf;

use crate::components::component::walk_components;
use crate::components::component::ComponentArena;
use crate::constants::CRANE_FILE;
use crate::visitors::status_visitor::StatusVisitor;

#[derive(Args, Debug)]
pub struct CommandArgs {
    pub dir: Option<PathBuf>,
}

async fn show_status(root_dir: &PathBuf, mut output: impl std::io::Write) -> Result<(), Error> {
    log::debug!("Show status in {:?}", root_dir);
    if let Err(err) = writeln!(output, "status:") {
        return Err(err.into());
    }

    let deps_file = PathBuf::from(CRANE_FILE);
    let commponent_ids = walk_components(&StatusVisitor::new(), &root_dir, &deps_file).await?;

    let mut nodes = vec![];
    for i in 0..commponent_ids.len() {
        nodes.push((0, i == 0, commponent_ids[i]));
    }
    while !nodes.is_empty() {
        let (depth, tail, current_id) = nodes.pop().unwrap();
        let comp = ComponentArena::instance().get(current_id).unwrap();

        let bifurcation = if tail { "└──" } else { "├──" };
        if let Err(err) = write!(output, "{:>width$}{}", "", bifurcation, width = depth * 2) {
            return Err(err.into());
        }
        if let Err(err) = writeln!(output, "{}", current_id) {
            return Err(err.into());
        }
        if !comp.children.is_empty() {
            for i in 0..comp.children.len() {
                nodes.push((depth + 1, i == 0, comp.children[i]));
            }
        }
    }

    let _ = output.flush();
    Ok(())
}

pub async fn run(args: &CommandArgs) -> Result<(), Error> {
    println!("{:?}", args);

    if let Some(target_dir) = &args.dir {
        show_status(target_dir, std::io::stdout()).await
    } else {
        println!("Syncing current directory");
        show_status(&PathBuf::from("."), std::io::stdout()).await
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use tempdir::TempDir;

    use crate::utils::test_utils;

    use super::show_status;

    // #[test]
    #[tokio::test]
    async fn test_show_status() {
        // Create repositories, the structure should be like this:
        // main
        //  |-sub1
        //    |-sub1_sub1
        //    |-sub1_sub2
        //  |-sub2
        //    |-sub2_sub1
        let err_msg = "Failed to create temporary directory";
        let main_repo_temp_dir = TempDir::new("main_repo").expect(err_msg);
        let main_repo_dir = main_repo_temp_dir.into_path();
        // let main_repo_dir = PathBuf::from("test_status");
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

        let mut output = Vec::new();
        show_status(&main_repo_dir, &mut output).await.unwrap();

        let expected_output = "status:
├──1
  └──4
└──0
  ├──3
  └──2
";
        assert_eq!(
            std::str::from_utf8(output.as_slice()).unwrap(),
            expected_output
        );
    }
}
