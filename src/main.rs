use author::Author;
use config::Config;
use git2::Repository;
mod args;
mod author;
mod config;
use crate::args::args;

fn git_config_author(config: &git2::Config) -> Result<Author, String> {
    let name = config
        .get_string("user.name")
        .ok()
        .ok_or("tried to get `git config user.name`, but value was empty")?;
    let email = config
        .get_string("user.email")
        .ok()
        .ok_or("tried to get `git config user.email`, but value was None")?;
    Ok(Author { name, email })
}

fn main() -> Result<(), String> {
    let args = args();

    let config = Config::config_from_file()?;

    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(_) => Err("not a git repository")?,
    };

    let config = match repo.config() {
        Ok(config) => config,
        Err(e) => Err(format!("failed to get config from repository: {e}"))?,
    };
    let identity = git_config_author(&config)?;

    println!("{identity}");

    Ok(())
}
