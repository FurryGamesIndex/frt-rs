#![allow(dead_code, unused_variables)]

use libfrt::profile::Profile;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Build FGI website
    Build {
        #[clap(short = 'P', long)]
        /// Specify a profile file to override default profile configurations
        profile: Option<String>,

        /// Output directory
        #[clap(short = 'o', long, default_value = "output")]
        output: String,
    },
    /// Validate and check source(s)
    Lint {
        game_bundle: Option<String>
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
/// frt - FRT - 2nd Gen FGI Rendering Tool
struct Cli {
    /// Verbose log
    #[clap(short = 'v', long, default_value_t = false)]
    verbose: bool,

    /// Override profile configuration values (in TOML format)
    #[clap( long)]
    config: Option<String>,

    #[clap(subcommand)]
    command: SubCommand,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        SubCommand::Build { .. } => todo!(),
        SubCommand::Lint { .. } => todo!(),
    }

    let profile = Profile::from_config("1.toml").unwrap();

    println!("{:?}", profile);
}