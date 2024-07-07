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
        short = 'm',
        long = "method",
        default_value = "arithmetic-mean",
        help = "Method of calculation expectation if --expectation is not set"
    )]
    expectation_method: AvgMethod,

    #[clap(short = 'e', long = "expectation", help = "Expectation")]
    expectation: Option<f64>,

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
    let variance_value = match args.expectation {
        Some(expectation) => ctx.variance(args.values.clone(), expectation),
        None => ctx.average(args.values.clone(), args.expectation_method),
    };

    let result = if args.standard_deviation {
        format_float(variance_value.sqrt(), args.precision)
    } else {
        format_float(variance_value, args.precision)
    };

    ctx.print(result);

    return Ok(());
}
