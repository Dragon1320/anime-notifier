use std::{env, path};

use serde::{Deserialize, Serialize};

const PATH_ENV: &str = "NEKO_CONFIG_PATH";
const DEFAULT_PATH: &str = "/etc/neko/config.yaml";
const PREFIX: &str = "NEKO";
const SEPARATOR: &str = "_";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
  pub message: String,
  pub cat: Cat,
}

// example nested config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cat {
  pub colour: String,
}

impl Config {
  pub fn load() -> Result<Self, config::ConfigError> {
    // try to load .env file and fail silently on error
    dotenv::dotenv().ok();

    let cfg_path = env::var(PATH_ENV).unwrap_or_else(|_| DEFAULT_PATH.into());
    let cfg_path = path::Path::new(&cfg_path);

    let config = {
      let mut builder = config::Config::builder();

      // its fine if we dont find a config file, as long as all the required fields
      // are specified in some other way (eg. through env vars)
      if cfg_path.exists() {
        builder = builder.add_source(config::File::from(cfg_path));
      }

      builder = builder.add_source(config::Environment::with_prefix(PREFIX).separator(SEPARATOR));

      builder.build()?
    };

    config.try_deserialize()
  }
}
