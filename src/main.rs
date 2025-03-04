use author::Author;
use config::Config;
use git_config::{GitConfig, GitConfigKey};
mod args;
mod author;
mod config;
mod git_config;
use crate::args::args;

fn print_author(author: &Author) {
    println!("     name: '{}'", author.name);
    println!("     mail: '{}'", author.email);
}

fn eprint_author(author: &Author) {
    println!("     name: '{}'", author.name);
    println!("     mail: '{}'", author.email);
}

fn current_author(config: &GitConfig) -> Result<Author, String> {
    let name = config.get(&GitConfigKey::Name)?;
    let email = config.get(&GitConfigKey::Email)?;
    Ok(Author { name, email })
}

fn get(author: Author, verbose: bool) -> Result<(), String> {
    let config_path = Config::path()?;
    let Some(config) = Config::from_file(&config_path)? else {
        if verbose {
            eprintln!("no config file found");
        }
        println!("current author:");
        print_author(&author);
        return Ok(());
    };

    if verbose {
        eprintln!("reading from config file '{}'", config_path.display());
    }
    println!("current author:");
    print_author(&author);
    println!();
    println!("available authors:");
    for (id, author) in config.authors.into_iter() {
        println!("  {id}:");
        print_author(&author);
    }

    Ok(())
}

fn set(id: String, git_config: &mut GitConfig, verbose: bool) -> Result<(), String> {
    let config_path = Config::path()?;
    let Some(config) = Config::from_file(&config_path)? else {
        return Err(format!(
            "no config file found at '{}'",
            config_path.display()
        ));
    };

    let Some(author) = config.authors.get(&id) else {
        return Err(format!(
            "config file '{}' does not contain any authors identified by '{id}'",
            config_path.display()
        ));
    };

    if verbose {
        eprintln!("reading from config file '{}'", config_path.display());
    }

    git_config.set(&GitConfigKey::Name, &author.name)?;
    git_config.set(&GitConfigKey::Email, &author.email)?;

    println!("current author:");
    print_author(&author);

    Ok(())
}

fn add(id: String, Author { name, email }: Author, verbose: bool) -> Result<(), String> {
    let config_path = Config::path()?;

    if verbose {
        eprintln!("reading from config file '{}'", config_path.display());
    }

    let Some(mut config) = Config::from_file(&config_path)? else {
        if verbose {
            eprintln!(
                "no config file found, creating new at '{}'",
                config_path.display()
            );
        }
        let mut config = Config::new();
        config.authors.insert(id, Author { name, email });
        config.write(&config_path)?;
        return Ok(());
    };

    config.authors.insert(id, Author { name, email });
    config.write(&config_path)?;

    println!("available authors:");
    for (id, author) in config.authors.into_iter() {
        println!("  {id}:");
        print_author(&author);
    }

    Ok(())
}

fn remove(id: String, verbose: bool) -> Result<(), String> {
    let config_path = Config::path()?;

    if verbose {
        eprintln!("reading from config file '{}'", config_path.display());
    }

    let Some(mut config) = Config::from_file(&config_path)? else {
        return Err(format!(
            "no config file found at '{}'",
            config_path.display()
        ));
    };

    if config.authors.remove(&id).is_none() {
        return Err(format!(
            "config file '{}' does not contain any authors identified by '{id}'",
            config_path.display()
        ));
    };

    config.write(&config_path)?;

    Ok(())
}

fn execute_cmd(input: String) {
    let cmd = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", &input])
            .spawn()
    } else {
        std::process::Command::new("sh")
            .args(["-c", &input])
            .spawn()
    };
    match cmd {
        Ok(mut child) => match child.wait() {
            Ok(code) => eprintln!("cmd '{input}' returned {code}"),
            Err(e) => eprintln!("an error occurred waiting for cmd '{input}' to exit: {e}"),
        },
        Err(e) => eprintln!("unable to run cmd '{input}': {e}"),
    }
}

fn doas(
    old_author: &Author,
    git_config: &mut GitConfig,
    id: String,
    cmd: String,
    verbose: bool,
) -> Result<(), String> {
    let config_path = Config::path()?;

    if verbose {
        eprintln!("reading from config file '{}'", config_path.display());
    }

    let Some(config) = Config::from_file(&config_path)? else {
        return Err(format!(
            "no config file found at '{}'",
            config_path.display()
        ));
    };

    let Some(author) = config.authors.get(&id) else {
        return Err(format!(
            "config file '{}' does not contain any authors identified by '{id}'",
            config_path.display()
        ));
    };
    git_config.set(&GitConfigKey::Name, &author.name)?;
    git_config.set(&GitConfigKey::Email, &author.email)?;

    if verbose {
        eprintln!("current author:");
        eprint_author(&author);
        eprintln!("executing cmd");
    }

    execute_cmd(cmd);

    git_config.set(&GitConfigKey::Name, &old_author.name)?;
    git_config.set(&GitConfigKey::Email, &old_author.email)?;

    if verbose {
        eprintln!("cmd executed");
        eprintln!("current author:");
        eprint_author(&author);
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let mut repo_config = GitConfig::new()?;

    let current_author = current_author(&repo_config)?;

    let args = args();

    let verbose = args.verbose;

    match args.command.unwrap_or(args::Commands::Get) {
        args::Commands::Get => get(current_author, verbose),
        args::Commands::Set { id } => set(id.to_string(), &mut repo_config, verbose),
        args::Commands::Add { id, name, email } => add(id, Author { name, email }, verbose),
        args::Commands::AddFromGit { id } => add(id, current_author, verbose),
        args::Commands::Remove { id } => remove(id, verbose),
        args::Commands::Doas { id, cmd } => {
            doas(&current_author, &mut repo_config, id, cmd, verbose)
        }
    }
}
