mod average;
mod variance;
mod discrete;
mod context;
mod variances_by_modes;

pub use discrete::{execute, Args};
pub use context::{Context, AvgMethod};
