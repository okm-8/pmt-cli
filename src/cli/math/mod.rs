mod context;
mod discrete;
mod format;
mod math;

pub use context::Context;
pub use math::{execute, Args};

pub mod discrete_cli {
    pub use super::discrete::{AvgMethod, Context};
}
