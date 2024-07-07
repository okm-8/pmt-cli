use clap::{Args as ClapArgs, ValueEnum};
use crate::cli::math::discrete::{Context, AvgMethod};
use crate::cli::math::format::format_float;

#[derive(ClapArgs)]
pub struct Args {
    #[clap(
        short = 'p',
        long = "precision",
        default_value = "2",
        help = "Precision of result"
    )]
    precision: usize,

    #[clap(
        short = 'm',
        long = "method",
        default_value = "arithmetic-mean",
        help = "Method of calculation"
    )]
    method: AvgMethod,

    #[clap(required = true, help = "Values list")]
    values: Vec<f64>
}

pub fn execute(ctx: &dyn Context, args: Args) -> Result<(), String> {
    let result = ctx.average(args.values.clone(), args.method);

    ctx.print(format_float(result, args.precision));

    return Ok(());
}
