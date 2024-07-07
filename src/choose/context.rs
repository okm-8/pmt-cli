use std::ops::Range;

pub trait Context {
    fn random_int(&mut self, range: Range<isize>) -> Result<isize, String>;
}
