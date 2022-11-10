use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(unused)]
pub struct Paths {
    #[serde(default = "default_path_cache")]
    pub cache_file: PathBuf,
    #[serde(default = "default_path_out")]
    pub output_path: PathBuf,
    #[serde(default = "default_path_diffout")]
    pub output_path_diff: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(unused)]
pub struct Vars {
    #[serde(default = "default_prefix_diff")]
    pub prefix_diff: String,
    #[serde(default = "default_prefix_stocklist")]
    pub prefix_stocklist: String,
    #[serde(default = "default_suffix")]
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
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("settings"))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("ressources/settings_local").required(false))
            .build()
            .unwrap_or_default();

        s.try_deserialize()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self { paths: Default::default(), vars: Default::default() }
    }
}

impl Default for Paths {
    fn default() -> Self {
        Self { cache_file: default_path_cache(), output_path: default_path_out(), output_path_diff: default_path_diffout() }
    }
}

impl Default for Vars {
    fn default() -> Self {
        Self { prefix_diff: default_prefix_diff(), prefix_stocklist: default_prefix_stocklist(), suffix: default_suffix() }
    }
}

fn default_path_cache() -> PathBuf {
    let path = PathBuf::from(r#".\cache_dir\symbol_cache.json"#);
    path
}

fn default_path_out() -> PathBuf {
    let path = PathBuf::from(r#".\stocks"#);
    path
}

fn default_path_diffout() -> PathBuf {
    let path = PathBuf::from(r#".\diffs"#);
    path
}

fn default_prefix_stocklist() -> String {
    "stocks".to_string()
}

fn default_prefix_diff() -> String {
    "diff".to_string()
}

fn default_suffix() -> String {
    ".txt".to_string()
}

lazy_static! {
    static ref CONFIG: Settings = init_envs();
}

fn init_envs() -> Settings {
    let mut settings: Settings = Settings::new().unwrap_or_default();
    create_settings(&settings);
    settings = canonicalize(settings);
    settings
}

fn canonicalize(mut settings: Settings) -> Settings {
    settings.paths.cache_file = settings.paths.cache_file.canonicalize().unwrap();
    settings.paths.output_path = settings.paths.output_path.canonicalize().unwrap();
    settings.paths.output_path_diff = settings.paths.output_path_diff.canonicalize().unwrap();
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