use clap::{Args as ClapArgs, ValueEnum};
use crate::cli::context::Context;
use crate::cli::math::format::format_float;
use crate::math::discrete::{
    arithmetic_mean,
    geometric_mean,
    harmonic_mean,
    median,
    modes,
    midrange,
    range
};

#[derive(ValueEnum, Clone)]
pub enum Method {
    ArithmeticMean,
    GeometricMean,
    HarmonicMean,
    Median,
    Modes,
    Midrange,
    Range
}

#[derive(ClapArgs)]
pub struct Args {
    #[clap(short = 'p', long = "precision", default_value = "2")]
    precision: usize,

    #[clap(short = 'm', long = "method", default_value = "arithmetic-mean")]
    method: Method,

    #[clap()]
    values: Vec<f64>
}

pub fn execute(ctx: &mut dyn Context, args: Args) -> Result<(), String> {
    let result = match args.method {
        Method::ArithmeticMean =>
            format_float(arithmetic_mean(args.values.clone()), args.precision),
        Method::GeometricMean =>
            format_float(geometric_mean(args.values.clone()), args.precision),
        Method::HarmonicMean =>
            format_float(harmonic_mean(args.values.clone()), args.precision),
        Method::Median =>
            format_float(median(args.values.clone()), args.precision),
        Method::Modes =>
            modes(args.values.clone()).iter()
                .map(|mode| format_float(*mode, args.precision))
                .collect::<Vec<String>>()
                .join(", "),
        Method::Midrange =>
            format_float(midrange(args.values.clone()), args.precision),
        Method::Range =>
            format_float(range(args.values.clone()), args.precision)
    };

    return match ctx.print(result) {
        Ok(_) => Ok(()),
        Err(error) => Err(error)
    };
}
