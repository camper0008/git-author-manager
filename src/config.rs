use std::{
    collections::HashMap,
    fs::{self, read_to_string},
    path::PathBuf,
};

use crate::author::Author;
pub const CONFIG_FILE_NAME: &'static str = ".git-authors.toml";

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub identities: HashMap<String, Author>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            identities: HashMap::new(),
        }
    }
    pub fn write<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        fs::write(
            path,
            toml::to_string_pretty(&self).expect("should be valid data structure"),
        )
    }

    pub fn config_path() -> Result<PathBuf, String> {
        let cwd = std::env::current_dir().map_err(|e| format!("error: unable to get cwd: {e}"))?;
        let config = cwd.ancestors().find_map(|ancestor| {
            let path = ancestor.join(CONFIG_FILE_NAME);
            let content = read_to_string(&path).ok()?;
            let result = toml::from_str::<Config>(&content)
                .err()
                .map(|err| {
                    Err(format!(
                        "tried to parse invalid config file at '{}': '{}'",
                        path.display(),
                        err.message()
                    ))
                })
                .unwrap_or_else(|| Ok(path));
            Some(result)
        });
        match config {
            Some(result) => result,
            None => Ok(cwd.join(CONFIG_FILE_NAME)),
        }
    }

    pub fn config_from_file() -> Result<Option<Self>, String> {
        let cwd = std::env::current_dir().map_err(|e| format!("error: unable to get cwd: {e}"))?;
        let config = cwd.ancestors().find_map(|ancestor| {
            let path = ancestor.join(CONFIG_FILE_NAME);
            let content = read_to_string(&path).ok()?;
            let config = toml::from_str::<Config>(&content).map_err(|err| {
                format!(
                    "tried to parse invalid config file at '{}': '{}'",
                    path.display(),
                    err.message()
                )
            });
            Some(config)
        });
        match config {
            Some(result) => Ok(Some(result?)),
            None => Ok(None),
        }
    }
}
