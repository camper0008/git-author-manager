use git2::{Config, Repository};

pub struct GitConfig {
    global: Config,
    local: Config,
}

impl GitConfig {
    pub fn new() -> Result<Self, String> {
        let repo = match Repository::open_from_env() {
            Ok(repo) => repo,
            Err(_) => Err("not a git repository")?,
        };

        let global = match repo.config() {
            Ok(config) => config,
            Err(e) => Err(format!("failed to get config from repository: {e}"))?,
        };

        let local = global
            .open_level(git2::ConfigLevel::Local)
            .map_err(|e| format!("failed to get local config for repository: {e}"))?;

        Ok(Self { local, global })
    }

    fn key_to_git_config_key(key: &GitConfigKey) -> &str {
        match key {
            GitConfigKey::Name => "user.name",
            GitConfigKey::Email => "user.email",
        }
    }

    fn get_global_config(&self, key: &str) -> Result<String, String> {
        self.global
            .get_string(key)
            .ok()
            .ok_or_else(|| format!("tried to get global `git config '{key}'`, but value was empty"))
    }

    fn set_local_config(&mut self, key: &str, value: &str) -> Result<(), String> {
        self.local.set_str(key, value).map_err(|e| {
            format!(
                "tried to set `git config '{key}'` to '{}', but failed: {e}",
                value
            )
        })
    }

    pub fn get(&self, key: &GitConfigKey) -> Result<String, String> {
        self.get_global_config(Self::key_to_git_config_key(key))
    }

    pub fn set(&mut self, key: &GitConfigKey, value: &str) -> Result<(), String> {
        self.set_local_config(Self::key_to_git_config_key(key), value)
    }
}

pub enum GitConfigKey {
    Name,
    Email,
}
