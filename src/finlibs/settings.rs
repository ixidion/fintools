use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use super::utils;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(unused)]
pub struct Paths {
    pub cache_file: PathBuf,
    pub output_path: PathBuf,
    pub output_path_diff: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(unused)]
pub struct Vars {
    pub prefix_diff: String,
    pub prefix_stocklist: String,
    pub suffix: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub paths: Paths,
    pub vars: Vars,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {

        let s = Config::builder()
            .set_default("paths.cache_file", String::from(r#"./cache_dir/symbol_cache.json"#))?
            .set_default("paths.output_path", String::from(r#"./stocks"#))?
            .set_default("paths.output_path_diff", String::from(r#"./diffs"#))?
            .set_default("vars.prefix_diff", String::from("diff"))?
            .set_default("vars.prefix_stocklist", String::from("stocks"))?
            .set_default("vars.suffix", String::from(".txt"))?
            .add_source(File::with_name("settings"))
            .add_source(File::with_name("ressources/settings_local").required(false))
            .build()
            .unwrap_or_default();

        s.try_deserialize()
    }
}

lazy_static! {
    static ref CONFIG: Settings = init_envs();
}

fn init_envs() -> Settings {
    let mut settings: Settings = Settings::new().unwrap();
    create_settings(&settings);
    settings = canonicalize(settings);
    settings
}

fn canonicalize(mut settings: Settings) -> Settings {
    settings.paths.cache_file = utils::absolute_path(settings.paths.cache_file).unwrap();
    settings.paths.output_path = utils::absolute_path(settings.paths.output_path).unwrap();
    settings.paths.output_path_diff = utils::absolute_path(settings.paths.output_path_diff).unwrap();
    println!("{:?}", &settings);
    settings
}

fn create_settings(settings: &Settings) {
    let contents = toml::to_string(settings).unwrap();
    let file_path = Path::new("settings.toml");
    if !file_path.is_file() {
        fs::write(&file_path, &contents).expect("Should have been able to write the file");
    }
}

pub fn get_config() -> Settings {
    return CONFIG.clone();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config() {
        println!("{:?}", get_config());
        println!("{:?}", toml::to_string(&get_config()).unwrap());
        let contents = toml::to_string(&get_config()).unwrap();
        let file_path = Path::new("settings.toml");
        if !file_path.is_file() {
            fs::write(&file_path, &contents).expect("Should have been able to write the file");
        }

    }
}