use clap::{Subcommand as ClapSubcommand, Args as ClapArgs};
use crate::cli::math::discrete::{average, variance, Context, variances_by_modes};

#[derive(ClapSubcommand)]
enum Subcommand {
    Avg(average::Args),
    Variance(variance::Args),
    VariancesByModes(variances_by_modes::Args)
}

#[derive(ClapArgs)]
pub struct Args {
    #[clap(subcommand)]
    command: Subcommand
}

pub fn execute(ctx: &dyn Context, args: Args) -> Result<(), String> {
    return match args.command {
        Subcommand::Avg(args) => average::execute(ctx, args),
        Subcommand::Variance(args) => variance::execute(ctx, args),
        Subcommand::VariancesByModes(args) => variances_by_modes::execute(ctx, args)
    };
}
