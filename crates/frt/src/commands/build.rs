use anyhow::Result;

use clap::Parser;
use libfrt::backend::BackendArguments;
use libfrt::{profile::Profile, Context};

use crate::CliBackend;

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

pub fn cli(mut profile: Profile, sub_args: &SubCommandBuild, backend: &CliBackend) -> Result<()> {
    let backend = backend.new_backend(&mut profile)?;

    let mut context = Context::new(profile, backend)?;
    context.full_init()?;

    let mut backend_args = BackendArguments::default();

    backend_args.set_bool("fs_output".to_owned(), true);
    backend_args.set_string("output".to_owned(), sub_args.output.clone());

    context.resync_backend(&backend_args)?;

    context.invoke_backend()?;

    Ok(())
}
