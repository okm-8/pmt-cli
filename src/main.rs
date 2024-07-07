mod choose;
mod math;
mod random;
mod cli;
mod terminal;
mod context;

use std::process::ExitCode;

fn main() -> ExitCode {
    let mut ctx = context::Context::new();

    return match cli::execute(&mut ctx) {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            terminal::print_value(error);

            return ExitCode::FAILURE;
        }
    };
}
