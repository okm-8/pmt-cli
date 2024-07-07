use crate::choose;

pub trait Context {
    fn scan_usize(&self, message: String) -> Result<usize, String>;
    fn scan_string(&self, message: String) -> Result<String, String>;
    fn print(&self, message: String) -> Result<(), String>;
    fn choose_context(&mut self) -> &mut dyn choose::Context;
}
