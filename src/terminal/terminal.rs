use std::fmt::{Debug, Display};
use std::str::FromStr;
use text_io::try_read;

pub fn scan_value<T: FromStr + Display>(message: String) -> Result<T, String>
where
    <T as FromStr>::Err: Debug,
{
    println!("{}:", message);

    return match try_read!("{}\n") {
        Ok(value) => Ok(value),
        Err(error) => Err(error.to_string()),
    };
}

pub fn print_value<T: Display>(value: T) {
    println!("{}", value);
}
