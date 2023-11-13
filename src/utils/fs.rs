use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn copy_dir_to(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.is_dir() {
        fs::create_dir_all(dst)?;
    }

    for entry in WalkDir::new(src) {
        let entry = entry?;
        let src_path = entry.path();
        let relative_path = src_path
            .strip_prefix(src)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let dst_path = dst.join(relative_path);

        if src_path.is_dir() {
            fs::create_dir_all(&dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use tempfile::tempdir;

    #[test]
    fn test_copy_dir_to() {
        // Create a temporary directory.
        let src_dir = tempdir().unwrap();
        let dst_dir = tempdir().unwrap();

        // Create a file in the source directory.
        let file_path = src_dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        // Copy the source directory to the destination directory.
        copy_dir_to(src_dir.path(), dst_dir.path()).unwrap();

        // Check if the file exists in the destination directory.
        let copied_file_path = dst_dir.path().join("test.txt");
        assert!(copied_file_path.exists());

        // Check the content of the copied file.
        let mut copied_file = File::open(copied_file_path).unwrap();
        let mut contents = String::new();
        copied_file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "Hello, world!\n");
    }
}
