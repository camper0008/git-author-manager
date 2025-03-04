use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Args {
    pub fn command(&self) -> &Commands {
        self.command.as_ref().unwrap_or(&Commands::Get)
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get the current author, and authors available.
    Get,

    /// Set current author
    Set {
        /// The identifier the author is saved as, i.e. `tph`
        identifier: String,
    },

    /// Adds an author
    Add {
        /// The identifier to save the author as, i.e. `tph`
        identifier: String,
        /// The name of the author, i.e. `Theis Pieter Hollebeek`
        name: Option<String>,
        /// The email of the author, i.e. `tphollebeek@example.org`
        email: Option<String>,
    },

    /// Removes an author (alias: `rm`)
    #[clap(alias = "rm")]
    Remove {
        /// The identifier the author is saved as, i.e. `tph`
        identifier: String,
    },

    /// Adds an author based on `git config` (`user.name`, `user.email`)
    AddFromGit {
        /// The identifier to save the author as, i.e. `tph`
        identifier: String,
    },
}

pub fn args() -> Args {
    Args::parse()
}
