use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about = "Poetry add without upper bound caps")]
pub struct Arguments {
    #[clap(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// Add a new dependency without upper bound caps.
    Add {
        #[clap(help = "Packages to add")]
        packages: String,

        #[clap(
            short,
            long,
            help = "If set the pyproject.toml file will be updated to pin dependencies"
        )]
        pin: bool,
    },

    /// Update the pyproject.toml file, removing upper bound caps, without adding any new
    /// dependencies.
    Fix {
        #[clap(
            short,
            long,
            help = "If set poetry will still run the add, but  the modified pyproject.toml file will be printed to the screen instead of saved"
        )]
        dry_run: bool,

        #[clap(
            short,
            long,
            help = "If set the pyproject.toml file will be updated to pin dependencies"
        )]
        pin: bool,
    },

    /// Update installed dependencies
    Update {
        #[clap(
            short,
            long,
            help = "If set the pyproject.toml file will be updated to pin dependencies"
        )]
        pin: bool,
    },
}
