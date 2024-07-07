mod average;
mod context;
mod discrete;
mod modes;
mod variance;
mod variances_by_modes;

pub use context::{AvgMethod, Context};
pub use discrete::{execute, Args};
