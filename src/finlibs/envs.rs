use std::path::PathBuf;

use serde::Deserialize;

lazy_static! {
    static ref CONFIG: Configuration = init_envs();
}

fn init_envs() -> Configuration {
    let config = envy::prefixed("FINAPP__")
        .from_env::<Configuration>()
        .expect("Please provide all necessary ENV-Variables.");

    config
}

pub fn get_config() -> Configuration {
    return CONFIG.clone();
}

#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
    #[serde(default = "default_path_cache")]
    pub cache_file: PathBuf,
    #[serde(default = "default_path_out")]
    pub output_path: PathBuf,
    #[serde(default = "default_path_diffout")]
    pub output_path_diff: PathBuf,
    #[serde(default = "default_prefix_stocklist")]
    pub prefix_stocklist: String,
    #[serde(default = "default_prefix_diff")]
    pub prefix_diff: String,
    #[serde(default = "default_suffix")]
    pub suffix: String,
}

fn default_path_cache() -> PathBuf {
    let path = PathBuf::from("C:\\temp\\symbol_cache.json");
    path
}

fn default_path_out() -> PathBuf {
    let path = PathBuf::from("C:\\temp");
    path
}

fn default_path_diffout() -> PathBuf {
    let path = PathBuf::from("C:\\temp\\diffs");
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
