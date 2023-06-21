use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::{env, path::PathBuf};

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ImportDiscovery {
    pub python_executable: PathBuf,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub import_discoverty: ImportDiscovery,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("examples/hierarchical-env/config/default"))
            .build()?;

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));

        s.try_deserialize()
    }
}