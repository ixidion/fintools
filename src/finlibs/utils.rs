use chrono::{Local};
use std::path::{Path, PathBuf};


pub fn formatted_timestamp() -> String {
    let now = Local::now();
    now.format("%Y%m%d%H%M%S").to_string()
}

pub fn change_extension(path: impl AsRef<Path>, name: &str, ) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    if let Some(ext) = path.extension() {
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