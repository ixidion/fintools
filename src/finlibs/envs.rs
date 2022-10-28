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
}

fn default_path_cache() -> PathBuf {
    let path = PathBuf::from("C:\\temp\\symbol_cache.json");
    path
}

fn default_path_out() -> PathBuf {
    let path = PathBuf::from("C:\\temp");
    path
}
