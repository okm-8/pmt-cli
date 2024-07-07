use std::ops::Range;
use crate::{choose, cli, random, terminal};

pub struct Context;

impl Context {
    pub fn new() -> Context {
        return Context;
    }
}

impl choose::Context for Context {
    fn random_int(&mut self, range: Range<isize>) -> Result<isize, String>{
        return Ok(random::random_int(range));
    }
}

impl cli::Context for Context {
    fn scan_usize(&self, message: String) -> Result<usize, String> {
        return terminal::scan_value(message);
    }

    fn scan_string(&self, message: String) -> Result<String, String> {
        return terminal::scan_value(message);
    }

    fn print(&self, message: String) -> Result<(), String> {
        terminal::print_value(message);

        return Ok(());
    }

    fn choose_context(&mut self) -> &mut dyn choose::Context {
        return self;
    }
}