use crate::constants::CACHE_DIR;
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn ensure_cache_dir() -> PathBuf {
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
    use super::*;
    #[test]
    fn test_ensure_cache_dir_creates_directory_if_not_exist() {
        let temp_home = "/tmp";
        // Save the current value of HOME
        let original_home = env::var("HOME").ok();
        // Set HOME to a mock value
        env::set_var("HOME", temp_home);
        if !PathBuf::from(temp_home).exists() {
            fs::create_dir_all(temp_home);
        }

        let cache_dir = PathBuf::from(env::var("HOME").unwrap()).join(CACHE_DIR);
        if cache_dir.exists() {
            remove_path_all(&cache_dir);
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
    #[should_panic(expected = "Path")]
    fn test_ensure_cache_dir_panics_if_path_exists_but_not_dir() {
        let temp_home = "/tmp";
        // Save the current value of HOME
        let original_home = env::var("HOME").ok();
        // Set HOME to a mock value
        env::set_var("HOME", temp_home);
        if !PathBuf::from(temp_home).exists() {
            fs::create_dir_all(temp_home);
        }

        let cache_dir = PathBuf::from(env::var("HOME").unwrap()).join(CACHE_DIR);
        if cache_dir.exists() {
            remove_path_all(&cache_dir);
        }
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
