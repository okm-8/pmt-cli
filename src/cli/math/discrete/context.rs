use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum AvgMethod {
    ArithmeticMean,
    GeometricMean,
    HarmonicMean,
    Median,
    Midrange,
    Range
}

pub trait Context {
    fn print(&self, message: String);
    fn average(&self, values: Vec<f64>, method: AvgMethod) -> f64;
    fn modes(&self, values: Vec<f64>) -> Vec<f64>;
    fn variance(&self, values: Vec<f64>, expectation: f64) -> f64;
    fn variances_by_modes(&self, values: Vec<f64>) -> Vec<f64>;
}
