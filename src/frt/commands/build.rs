use anyhow::Result;

use clap::Parser;
use libfrt::{Context, profile::Profile};

#[derive(Parser, Debug)]
pub struct SubCommandBuild {
    /// Extra arguments passed to render backend.
    #[clap(short = 'a', long)]
    args: Vec<String>,

    /// Directory for file-system output.
    /// Same as use -a output=...
    #[clap(short = 'o', long, default_value = "output")]
    output: String,

}

pub fn cli(profile: Profile, sub_args: &SubCommandBuild) -> Result<()> {
    let mut context = Context::new(profile);
    context.full_init()?;
    Ok(())
}