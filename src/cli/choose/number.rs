use std::ops::DerefMut;
use std::process::ExitCode;
use clap::Args as ClapArgs;
use crate::choose;
use crate::cli::choose::scan::scan_rolls;
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

    #[clap(
        short = 'p',
        long = "precision",
        default_value = "0",
        help = "Precision of result"
    )]
    precision: i32,

    #[clap(required = true, help = "Minimum value")]
    min: f64,

    #[clap(required = true, help = "Maximum value")]
    max: f64,
}

fn print_numbers(ctx: &dyn Context, numbers: Vec<f64>, precision: i32) -> Result<(), String> {
    return match ctx.print(format!(
        "The numbers is {}",
        numbers.iter()
            .map(|number| format!("{:.*}", precision as usize, number)).collect::<Vec<String>>()
            .join(", ")
    )) {
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

    let min = args.min;
    let max = args.max;

    return match choose::numbers(
        ctx.choose_context(),
        min,
        max,
        args.precision,
        &choose::Opts::new(rolls, args.count)
    ) {
        Ok(numbers) => print_numbers(ctx, numbers, args.precision),
        Err(error) => Err(error)
    };
}
