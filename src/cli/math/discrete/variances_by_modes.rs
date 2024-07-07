use crate::cli::math::discrete::{AvgMethod, Context};
use crate::cli::math::format::format_float;
use clap::Args as ClapArgs;

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
        short = 'q',
        long = "standard-deviation",
        help = "Standard deviation instead of variance"
    )]
    standard_deviation: bool,

    #[clap(required = true, num_args = 1.., help = "Values list")]
    values: Vec<f64>,
}

pub fn execute(ctx: &dyn Context, args: Args) -> Result<(), String> {
    let result = ctx
        .variances_by_modes(args.values.clone())
        .iter()
        .map(|variance_value| {
            if args.standard_deviation {
                format_float(variance_value.sqrt(), args.precision)
            } else {
                format_float(*variance_value, args.precision)
            }
        })
        .collect::<Vec<String>>()
        .join(", ");

    ctx.print(result);

    return Ok(());
}
