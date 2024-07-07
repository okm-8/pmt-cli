use clap::Args as ClapArgs;
use crate::cli::context::Context;
use crate::cli::math::discrete::average;
use crate::cli::math::format::format_float;
use crate::math::discrete::{arithmetic_mean, geometric_mean, harmonic_mean, median, midrange, modes, range, variance};

#[derive(ClapArgs)]
pub struct Args {
    #[clap(short = 'p', long = "precision", default_value = "2")]
    precision: usize,

    #[clap(short = 'm', long = "method", default_value = "arithmetic-mean")]
    expectation_method: average::Method,

    #[clap(short = 'e', long = "expectation")]
    expectation: Option<f64>,

    #[clap(short = 'q', long = "standard-deviation")]
    standard_deviation: bool,

    #[clap(required = true, num_args = 1..)]
    values: Vec<f64>
}

pub fn execute(ctx: &mut dyn Context, args: Args) -> Result<(), String> {
    let variance_value = match args.expectation {
        Some(expectation) => {
            variance(args.values.clone(), expectation)
        }
        None => match args.expectation_method {
            average::Method::ArithmeticMean =>
                variance(args.values.clone(), arithmetic_mean(args.values.clone())),
            average::Method::GeometricMean =>
                variance(args.values.clone(), geometric_mean(args.values.clone())),
            average::Method::HarmonicMean =>
                variance(args.values.clone(), harmonic_mean(args.values.clone())),
            average::Method::Median =>
                variance(args.values.clone(), median(args.values.clone())),
            average::Method::Modes =>
                variance(args.values.clone(), modes(args.values.clone())[0].clone()),
            average::Method::Midrange =>
                variance(args.values.clone(), midrange(args.values.clone())),
            average::Method::Range =>
                variance(args.values.clone(), range(args.values.clone()))
        }
    };

    let result = if args.standard_deviation {
        format_float(variance_value.sqrt(), args.precision)
    } else {
        format_float(variance_value, args.precision)
    };

    return match ctx.print(result) {
        Ok(_) => Ok(()),
        Err(error) => Err(error)
    };
}
