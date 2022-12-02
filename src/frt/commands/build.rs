use anyhow::Result;

use clap::Parser;
use libfrt::{Context, profile::Profile};
use libfrt::backend::BackendArguments;

#[derive(Parser, Debug)]
pub struct SubCommandBuild {
    /// Extra arguments passed to render backend.
    #[clap(short = 'a', long = "argument")]
    args: Vec<String>,

    /// Directory for file-system output of full build.
    /// Same as use -a output=...
    #[clap(short = 'o', long, default_value = "output")]
    output: String,

}

pub fn cli(profile: Profile, sub_args: &SubCommandBuild) -> Result<()> {
    let mut context = Context::new(profile)?;
    context.full_init()?;

    let mut backend_args = BackendArguments::new();

    backend_args.insert("output".to_owned(), sub_args.output.clone());

    context.invoke_backend(backend_args)?;

    Ok(())
}