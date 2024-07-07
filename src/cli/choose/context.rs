pub trait Context {
    fn scan_usize(&self, message: String) -> Result<usize, String>;
    fn scan_string(&self, message: String) -> Result<String, String>;
    fn print(&self, message: String);
    fn numbers(
        &mut self,
        min: f64,
        max: f64,
        precision: i32,
        rolls: usize,
        count: usize
    ) -> Result<Vec<f64>, String>;
    fn indexes(
        &mut self,
        variants: Vec<String>,
        rolls: usize,
        count: usize
    ) -> Result<Vec<usize>, String>;
}
