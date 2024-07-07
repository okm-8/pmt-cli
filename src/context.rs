use crate::cli::math_cli::discrete_cli;
use crate::cli::{choose_cli, math_cli};
use crate::{choose, cli, math, random, terminal};
use std::ops::Range;

pub struct Context;

impl Context {
    pub fn new() -> Context {
        return Context;
    }
}

impl choose::Context for Context {
    fn random_int(&mut self, range: Range<isize>) -> Result<isize, String> {
        return Ok(random::random_int(range));
    }
}

impl choose_cli::Context for Context {
    fn scan_usize(&self, message: String) -> Result<usize, String> {
        return terminal::scan_value(message);
    }

    fn scan_string(&self, message: String) -> Result<String, String> {
        return terminal::scan_value(message);
    }

    fn print(&self, message: String) {
        terminal::print_value(message);
    }

    fn numbers(
        &mut self,
        min: f64,
        max: f64,
        precision: i32,
        rolls: usize,
        count: usize,
        unique: bool,
        sort: bool,
    ) -> Result<Vec<f64>, String> {
        return choose::numbers(
            self,
            min,
            max,
            precision,
            &choose::Opts::new(rolls, count, unique, sort),
        );
    }

    fn indexes(
        &mut self,
        variants: Vec<String>,
        rolls: usize,
        count: usize,
        unique: bool,
        sort: bool,
    ) -> Result<Vec<usize>, String> {
        return choose::indexes(
            self,
            variants,
            &choose::Opts::new(rolls, count, unique, sort),
        );
    }
}

impl discrete_cli::Context for Context {
    fn print(&self, message: String) {
        terminal::print_value(message);
    }

    fn average(&self, values: Vec<f64>, method: discrete_cli::AvgMethod) -> f64 {
        return match method {
            discrete_cli::AvgMethod::ArithmeticMean => math::discrete::arithmetic_mean(values),
            discrete_cli::AvgMethod::GeometricMean => math::discrete::geometric_mean(values),
            discrete_cli::AvgMethod::HarmonicMean => math::discrete::harmonic_mean(values),
            discrete_cli::AvgMethod::Median => math::discrete::median(values),
            discrete_cli::AvgMethod::Midrange => math::discrete::midrange(values),
            discrete_cli::AvgMethod::Range => math::discrete::range(values),
        };
    }

    fn modes(&self, values: Vec<f64>) -> Vec<f64> {
        return math::discrete::modes(values);
    }

    fn variance(&self, values: Vec<f64>, expectation: f64) -> f64 {
        return math::discrete::variance(values, expectation);
    }

    fn variances_by_modes(&self, values: Vec<f64>) -> Vec<f64> {
        return math::discrete::modes(values.clone())
            .iter()
            .map(|mode| math::discrete::variance(values.clone(), *mode))
            .collect();
    }
}

impl math_cli::Context for Context {
    fn discrete(&self) -> &dyn discrete_cli::Context {
        return self;
    }
}

impl cli::Context for Context {
    fn choose(&mut self) -> &mut dyn choose_cli::Context {
        return self;
    }

    fn math(&self) -> &dyn math_cli::Context {
        return self;
    }
}
