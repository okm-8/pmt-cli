use crate::choose;
use std::ops::Range;

pub fn random_int(range: Range<isize>) -> isize {
    return range.start
        + ((range.end - range.start) as f64 * rand::random::<f64>()).floor() as isize;
}
