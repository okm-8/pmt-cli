use std::process::ExitCode;
use std::str::FromStr;
use clap::{Parser as ClapParser, Subcommand};
use crate::cli::{choose as chooseCmd, math};
use crate::cli::context::Context;

#[derive(Subcommand)]
enum Command {
    Choose(chooseCmd::Args),
    Math(math::Args)
}

#[derive(ClapParser)]
struct Parser {
    #[clap(subcommand)]
    command: Command
}

pub fn execute(ctx: &mut dyn Context) -> Result<(), String> {
    let args = Parser::parse();

    return match args.command {
        Command::Choose(args) => chooseCmd::execute(ctx, args),
        Command::Math(args) => math::execute(ctx, args)
    };
}
