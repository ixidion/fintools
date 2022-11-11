use chrono::Local;
use std::path::{Path, PathBuf};

use std::env;
use std::io;

use path_clean::PathClean;

pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }.clean();

    Ok(absolute_path)
}

pub fn formatted_timestamp() -> String {
    let now = Local::now();
    now.format("%Y%m%d%H%M%S").to_string()
}

pub fn change_extension(path: impl AsRef<Path>, name: &str, ) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    if let Some(_ext) = path.extension() {
        result.set_extension(name);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_ts() {
        println!("{}", formatted_timestamp());
    } 
}