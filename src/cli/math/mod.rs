mod discrete;
mod format;
mod math;
mod context;

pub use math::{execute, Args};
pub use context::Context;

pub mod discrete_cli {
    pub use super::discrete::{Context, AvgMethod};
}
