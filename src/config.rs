use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
// todo: want to read from `~/.worklog.toml` and parse it into a Config struct

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub editor_command: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor_command: "nvim".to_string(),
        }
    }
}

pub fn load_config() -> Result<Config, String> {
    let config_path = get_config_path();

    match fs::read_to_string(&config_path) {
        Ok(contents) => {
            let user_config: Config = serde_yaml::from_str(&contents)
                .expect("Failed to parse Config from `~/.worklog/config.yaml`");
            return Ok(user_config);
        }
        Err(_) => {
            set_config(Config::default());
            return Ok(Config::default());
        }
    }
}

fn set_config(config: Config) {
    let config_path = get_config_path();

    if let Some(parent_dir) = config_path.parent() {
        fs::create_dir_all(parent_dir).expect("Failed to create ~/.worklog directory");
    }

    let config_content =
        serde_yaml::to_string(&config).expect("Failed to save config - could not serialize");

    fs::write(config_path, config_content).expect("Failed to save config to ~/.worklog/config.yaml")
}

fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").expect("Failed to find HOME env variable");
    return PathBuf::from(&home).join(".worklog").join("config.yaml");
}
