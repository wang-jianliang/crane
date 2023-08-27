use crate::constants::CACHE_DIR;
use lazy_static::lazy_static;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

lazy_static! {
    static ref CACHE_DIR_LOCK: Mutex<usize> = Mutex::new(0);
}

pub fn ensure_cache_dir() -> PathBuf {
    let _guard = CACHE_DIR_LOCK.lock();
    let cache_dir = PathBuf::from(env::var("HOME").unwrap()).join(CACHE_DIR);
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).unwrap();
    } else if !cache_dir.is_dir() {
        panic!("Path {:?} exists but is not a directory", cache_dir);
    }
    cache_dir
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;

    use super::*;

    lazy_static! {
        // This lock is used to ensure the test cases running serially because we mock the HOME environment variable in these cases
        static ref TEST_PROCESS_LOCK: Mutex<usize> = Mutex::new(0);
    }
    #[test]
    fn test_ensure_cache_dir_creates_directory_if_not_exist() {
        let _guard = TEST_PROCESS_LOCK.lock();

        let temp_home =
            TempDir::new("test_cache_dir_1").expect("Failed to create temporary directory");
        // Save the current value of HOME
        let original_home = env::var("HOME").ok();
        // Set HOME to a mock value
        env::set_var("HOME", temp_home.path().to_str().unwrap());

        let cache_dir = PathBuf::from(env::var("HOME").unwrap()).join(CACHE_DIR);
        if cache_dir.exists() {
            let _ = fs::remove_dir_all(&cache_dir).unwrap();
        }
        assert!(!cache_dir.exists());
        ensure_cache_dir();
        assert!(cache_dir.exists());
        assert!(cache_dir.is_dir());

        // Restore the original value of HOME
        if let Some(val) = original_home {
            env::set_var("HOME", val);
        } else {
            env::remove_var("HOME");
        }
    }
    #[test]
    #[should_panic(expected = "not a directory")]
    fn test_ensure_cache_dir_panics_if_path_exists_but_not_dir() {
        let _guard = TEST_PROCESS_LOCK.lock();

        let temp_home =
            TempDir::new("test_cache_dir_2").expect("Failed to create temporary directory");
        // Save the current value of HOME
        let original_home = env::var("HOME").ok();
        // Set HOME to a mock value
        env::set_var("HOME", temp_home.path().to_str().unwrap());

        let cache_dir = PathBuf::from(env::var("HOME").unwrap()).join(CACHE_DIR);
        if cache_dir.exists() {
            let _ = fs::remove_dir_all(&cache_dir).unwrap();
        }
        print!("cache_dir: {:?}", cache_dir);
        fs::File::create(cache_dir).unwrap();
        ensure_cache_dir();

        // Restore the original value of HOME
        if let Some(val) = original_home {
            env::set_var("HOME", val);
        } else {
            env::remove_var("HOME");
        }
    }
}
