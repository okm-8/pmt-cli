use clap::{Subcommand as ClapSubcommand, Args as ClapArgs};
use crate::cli::Context;
use crate::cli::math::discrete::{average, variance};

#[derive(ClapSubcommand)]
enum Subcommand {
    Avg(average::Args),
    Variance(variance::Args)
}

#[derive(ClapArgs)]
pub struct Args {
    #[clap(subcommand)]
    command: Subcommand
}

pub fn execute(ctx: &mut dyn Context, args: Args) -> Result<(), String> {
    return match args.command {
        Subcommand::Avg(args) => average::execute(ctx, args),
        Subcommand::Variance(args) => variance::execute(ctx, args)
    };
}
