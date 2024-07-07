use crate::cli::{choose, math};

pub trait Context {
    fn choose(&mut self) -> &mut dyn choose::Context;
    fn math(&self) -> &dyn math::Context;
}
