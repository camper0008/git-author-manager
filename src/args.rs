use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
pub struct Args {
    #[clap(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get the current author, and authors available.
    Get,

    /// Set current author
    Set {
        /// The id the author is saved as, i.e. `tph`
        id: String,
    },

    /// Adds an author
    Add {
        /// The id to save the author as, i.e. `tph`
        id: String,
        /// The name of the author, i.e. `Theis Pieter Hollebeek`
        name: String,
        /// The email of the author, i.e. `tphollebeek@example.org`
        email: String,
    },

    /// Removes an author
    Remove {
        /// The id the author is saved as, i.e. `tph`
        id: String,
    },

    /// Runs the code as the author specified, then reverts to previous config
    Doas {
        /// The id of the author to run the cmd as, i.e. `tph`
        id: String,

        /// The command to run
        cmd: Vec<String>,
    },

    /// Adds an author based on `git config` (`user.name`, `user.email`)
    AddFromGit {
        /// The id to save the author as, i.e. `tph`
        id: String,
    },

    /// Copies the first found config file to the specified path
    CopyConfig {
        /// The path to copy the config file to. Defaults to current working directory
        destination: Option<PathBuf>,
    },
}

pub fn args() -> Args {
    Args::parse()
}
