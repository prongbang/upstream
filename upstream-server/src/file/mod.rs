use std::{env, io};

pub mod route;

pub fn get_current_working_directory() -> io::Result<String> {
    let path = env::current_dir()?;
    Ok(path.display().to_string())
}
