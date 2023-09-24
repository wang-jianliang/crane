use crate::errors::Error;

pub fn run() -> Result<(), Error> {
    let version = get_version();
    println!("{}", version);
    Ok(())
}

// read version from cargo.toml
pub fn get_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    version.to_string()
}
