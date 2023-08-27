use lazy_static::lazy_static;
use std::env;

pub const CRANE_FILE: &str = ".crane";
pub const DEFAULT_LOG_LEVEL: &str = "debug";
pub const CACHE_DIR: &str = ".crane_cache";

lazy_static! {
    pub static ref CRANE_DEBUG: bool = match env::var("CRANE_DEBUG") {
        Ok(val) => {
            match val.to_lowercase().as_str() {
                "true" | "1" => true,
                _ => false,
            }
        }
        Err(_) => false,
    };
}
