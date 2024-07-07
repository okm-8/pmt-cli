pub fn format_float(value: f64, precision: usize) -> String {
    return format!("{:.*}", precision, value);
}
