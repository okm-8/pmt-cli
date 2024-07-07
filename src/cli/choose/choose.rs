use std::process::ExitCode;
use clap::{Args as ClapArgs, Subcommand as ClapSubcommand};
use crate::cli::choose::number;
use crate::cli::choose::variant;
use crate::cli::context::Context;

#[derive(ClapSubcommand)]
enum Subcommand {
    Number(number::Args),
    Variant(variant::Args)
}

#[derive(ClapArgs)]
pub struct Args {
    #[clap(subcommand)]
    command: Subcommand
}

pub fn execute(ctx: &mut dyn Context, args: Args) -> Result<(), String> {
    return match args.command {
        Subcommand::Number(args) => number::execute(ctx, args),
        Subcommand::Variant(args) => variant::execute(ctx, args)
    };
}
