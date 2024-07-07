use crate::cli::math::discrete::Context;
use crate::cli::math::format::format_float;
use clap::{Args as ClapArgs, ValueEnum};

#[derive(ClapArgs)]
pub struct Args {
    #[clap(
        short = 'p',
        long = "precision",
        default_value = "2",
        help = "Precision of result"
    )]
    precision: usize,

    #[clap(required = true, help = "Values list")]
    values: Vec<f64>,
}

pub fn execute(ctx: &dyn Context, args: Args) -> Result<(), String> {
    let result = ctx
        .modes(args.values)
        .iter()
        .map(|mode| format_float(*mode, args.precision))
        .collect::<Vec<String>>()
        .join(", ");

    ctx.print(result);

    return Ok(());
}
