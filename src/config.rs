use std::{
    collections::HashMap,
    fs::{self, read_to_string},
    path::PathBuf,
};

use crate::author::Author;
pub const CONFIG_FILE_NAME: &'static str = ".git-authors.toml";

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub authors: HashMap<String, Author>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            authors: HashMap::new(),
        }
    }
    pub fn write<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), String> {
        fs::write(
            path.as_ref(),
            toml::to_string_pretty(&self).expect("should be valid data structure"),
        )
        .map_err(|e| {
            format!(
                "unable to write to config file at '{}': {}",
                path.as_ref().display(),
                e
            )
        })
    }

    pub fn path() -> Result<PathBuf, String> {
        let cwd = std::env::current_dir().map_err(|e| format!("error: unable to get cwd: {e}"))?;
        let config = cwd.ancestors().find_map(|ancestor| {
            let path = ancestor.join(CONFIG_FILE_NAME);
            let content = read_to_string(&path).ok()?;
            let result = toml::from_str::<Config>(&content)
                .err()
                .map(|e| {
                    Err(format!(
                        "tried to parse invalid config file at '{}': '{}'",
                        path.display(),
                        e.message()
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

    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Option<Self>, String> {
        let Some(content) = read_to_string(&path).ok() else {
            return Ok(None);
        };
        toml::from_str::<Config>(&content).map(Some).map_err(|e| {
            format!(
                "tried to parse invalid config file at '{}': '{}'",
                path.as_ref().display(),
                e.message()
            )
        })
    }
}
