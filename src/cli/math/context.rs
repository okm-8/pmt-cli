use crate::cli::math::discrete;

pub trait Context {
    fn discrete(&self) -> &dyn discrete::Context;
}
