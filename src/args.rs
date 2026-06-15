use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
pub struct Args {
    #[clap(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
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

    /// Runs the cmd as the author specified, then reverts to previous config
    Doas {
        /// The id of the author to run the cmd as, i.e. `tph`
        id: String,

        /// The command to run, i.e. `git commit -m "v0.1.0"`
        cmd: Vec<String>,
    },

    /// Commits the code as the author specified, then reverts to previous config
    Commit {
        /// The id of the author to commit as, i.e. `tph`. Defaults to current user
        #[arg(short, long)]
        author: Option<String>,

        /// Id of co-authors, i.e. `git aum commit -c mtk -c tph -- -m "v0.1.0"`
        #[arg(short, long)]
        co_authors: Vec<String>,

        /// Additional flags, i.e. `git aum commit tph -- -am "hello"`
        flags: Vec<String>,
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
