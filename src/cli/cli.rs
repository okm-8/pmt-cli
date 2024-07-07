use clap::{Parser as ClapParser, Subcommand};
use crate::cli::{choose, math};
use crate::cli::context::Context;

#[derive(Subcommand)]
enum Command {
    Choose(choose::Args),
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
        Command::Choose(args) => choose::execute(ctx.choose(), args),
        Command::Math(args) => math::execute(ctx.math(), args)
    };
}
