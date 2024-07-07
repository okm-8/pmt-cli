use std::process::ExitCode;
use clap::Args as ClapArgs;
use crate::choose;
use crate::cli::choose::scan::{scan_rolls, scan_variants};
use crate::cli::context::Context;

#[derive(ClapArgs)]
pub struct Args {
    #[clap(
        short = 'r',
        long = "rolls",
        default_value = "0",
        help = "Number of rolls the dice for each number"
    )]
    rolls: usize,

    #[clap(
        short = 'c',
        long = "count",
        default_value = "1",
        help = "Number of values in result"
    )]
    count: usize,

    #[clap()]
    variants: Vec<String>
}

fn print_variants(ctx: &dyn Context, variants: Vec<String>) -> Result<(), String> {
    return match ctx.print(format!("The variants are {}", variants.join(", "))) {
        Ok(_) => Ok(()),
        Err(error) => Err(error)
    };
}

pub fn execute(ctx: &mut dyn Context, args: Args) -> Result<(), String> {
    let mut rolls = args.rolls;

    if rolls == 0 {
        rolls = match scan_rolls(ctx) {
            Ok(rolls) => rolls,
            Err(error) => return Err(error)
        };
    }

    let mut variants = args.variants;

    if variants.len() == 0 {
        variants = match scan_variants(ctx) {
            Ok(variants) => variants,
            Err(error) => return Err(error)
        };
    }

    if variants.len() == 1 {
        return match print_variants(ctx, vec![variants[0].clone(); args.count]) {
            Ok(_) => Ok(()),
            Err(error) => Err(error)
        }
    }

    return match choose::indexes(
        ctx.choose_context(),
        variants.clone(),
        &choose::Opts::new(rolls, args.count)
    ) {
        Ok(indexes) => match print_variants(
            ctx,
            indexes.iter().map(|index| variants[*index].clone()).collect()
        ) {
            Ok(_) => Ok(()),
            Err(error) => Err(error)
        },
        Err(error) => Err(error)
    };
}
