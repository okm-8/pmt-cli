#![allow(dead_code)]
#![allow(unused_imports)]

mod cli;
mod choose;
mod math;
mod context;

pub use cli::execute;
pub use context::Context;

pub mod choose_cli {
    pub use super::choose::Context;
}

pub mod math_cli {
    pub use super::math::{Context, discrete_cli};
}