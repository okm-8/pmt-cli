use crate::cli::math::{discrete, Context};
use clap::{Args as ClapArgs, Subcommand as ClapSubcommand};

#[derive(ClapSubcommand)]
enum Subcommand {
    Discrete(discrete::Args),
}

#[derive(ClapArgs)]
pub struct Args {
    #[clap(subcommand)]
    command: Subcommand,
}

pub fn execute(ctx: &dyn Context, args: Args) -> Result<(), String> {
    return match args.command {
        Subcommand::Discrete(args) => discrete::execute(ctx.discrete(), args),
    };
}
