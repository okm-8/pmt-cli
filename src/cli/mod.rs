#![allow(dead_code)]
#![allow(unused_imports)]

mod choose;
mod cli;
mod context;
mod math;

pub use cli::execute;
pub use context::Context;

pub mod choose_cli {
    pub use super::choose::Context;
}

pub mod math_cli {
    pub use super::math::{discrete_cli, Context};
}
