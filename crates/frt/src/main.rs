#[macro_use]
extern crate log;

pub mod backend;
pub mod commands;

use libfrt::profile::Profile;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::backend::CliBackend;

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Render FGI website, pages, components, etc
    Build(commands::build::SubCommandBuild),
    /// Validate and check source(s)
    Lint { game_bundle: Option<String> },
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
/// frt - FRT - 2nd Gen FGI Rendering Tool
struct Cli {
    /// Verbose log
    #[clap(short = 'v', long, default_value_t = false)]
    verbose: bool,

    #[clap(short = 'P', long)]
    /// Specify a profile file to override default profile configurations
    profile: Option<String>,

    /// Override profile configuration values (in TOML format)
    #[clap(long)]
    config: Option<String>,

    /// Select backend
    #[clap(short = 'b', long, default_value = "www")]
    backend: CliBackend,

    #[clap(subcommand)]
    command: SubCommand,
}

fn main() -> Result<()> {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "full");
    }

    if std::env::var("FRT_LOG").is_err() {
        std::env::set_var("FRT_LOG", "info");
    }

    let args = Cli::parse();

    if args.verbose {
        std::env::set_var("FRT_LOG", "debug");
    }

    pretty_env_logger::init_custom_env("FRT_LOG");

    debug!("FRT starting");

    let mut profiles = Vec::<String>::new();

    if let Some(profile) = args.profile {
        info!("Loading profile: {profile}");
        profiles.push(std::fs::read_to_string(profile)?.to_owned());
    }

    if let Some(config) = args.config {
        info!("Loading profile via command-line config");
        profiles.push(config);
    }

    let profile = if profiles.is_empty() {
        info!("No profile provided, fallback to default");
        Profile::default()
    } else {
        Profile::from_configs(profiles.iter().map(|s| &**s).collect())?
    };

    match args.command {
        SubCommand::Build(s) => {
            commands::build::cli(profile, &s, &args.backend)?;
        }
        SubCommand::Lint { .. } => todo!(),
    }

    Ok(())
}
