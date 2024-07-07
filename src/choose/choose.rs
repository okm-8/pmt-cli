use crate::choose::context::Context;
use std::any::Any;
use std::error::Error;
use std::ops::Range;

fn random_int(ctx: &mut dyn Context, range: Range<isize>) -> Result<isize, String> {
    return match ctx.random_int(range) {
        Ok(value) => Ok(value),
        Err(error) => Err(error),
    };
}

pub struct Opts {
    rolls: usize,
    count: usize,
}

impl Opts {
    pub fn new(rolls: usize, count: usize) -> Self {
        return Opts { rolls, count };
    }

    pub fn default() -> Self {
        return Opts::new(1, 1);
    }
}

pub fn number(
    ctx: &mut dyn Context,
    min: f64,
    max: f64,
    precision: i32,
    opts: &Opts,
) -> Result<f64, String> {
    if opts.count == 0 {
        return Err("Count is zero".to_string());
    }

    if opts.rolls == 0 {
        return Err("Rolls is zero".to_string());
    }

    if min == max {
        return Ok(min);
    }

    let scale = 10_f64.powi(precision);
    let [mut min_int, mut max_int] = [(min * scale) as isize, (max * scale) as isize];

    if min_int > max_int {
        [min_int, max_int] = [max_int, min_int];
    }

    let values_count = max_int - min_int + 1;

    let mut counters = vec![0; values_count as usize];

    for _ in 0..opts.rolls {
        match random_int(ctx, 0..values_count) {
            Ok(index) => counters[index as usize] += 1,
            Err(error) => return Err(error),
        }
    }

    let mut most_repeated_index = 0;
    for index in 0..counters.len() {
        if counters[index] > counters[most_repeated_index] {
            most_repeated_index = index;
        }
    }

    return Ok((min_int + most_repeated_index as isize) as f64 / scale);
}

pub fn numbers(
    ctx: &mut dyn Context,
    min: f64,
    max: f64,
    precision: i32,
    opts: &Opts,
) -> Result<Vec<f64>, String> {
    if opts.count == 0 || opts.rolls == 0 {
        return Ok(Vec::new());
    }

    if min == max {
        return Ok(vec![min; opts.count]);
    }

    let mut values: Vec<f64> = Vec::with_capacity(opts.count);

    for _ in 0..opts.count {
        match number(ctx, min, max, precision, opts) {
            Ok(value) => values.push(value),
            Err(error) => return Err(error),
        }
    }

    return Ok(values);
}

pub fn index<T>(ctx: &mut dyn Context, variants: Vec<T>, opts: &Opts) -> Result<usize, String> {
    return match number(ctx, 0.0, (variants.len() - 1) as f64, 0, opts) {
        Ok(index) => Ok(index as usize),
        Err(error) => Err(error),
    };
}

pub fn indexes<T>(
    ctx: &mut dyn Context,
    variants: Vec<T>,
    opts: &Opts,
) -> Result<Vec<usize>, String> {
    return match numbers(ctx, 0.0, (variants.len() - 1) as f64, 0, opts) {
        Ok(indexes) => Ok(indexes.iter().map(|index| *index as usize).collect()),
        Err(error) => Err(error),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::ops::Range;

    struct ContextMock {
        sequence: VecDeque<isize>,
    }

    impl ContextMock {
        fn new(sequence: Vec<isize>) -> Self {
            return ContextMock {
                sequence: VecDeque::from(sequence),
            };
        }
    }

    impl Context for ContextMock {
        fn random_int(&mut self, _range: Range<isize>) -> Result<isize, String> {
            return Ok(self.sequence.pop_front().unwrap());
        }
    }

    #[test]
    fn test_choose_number_one_pass() {
        let mut context_mock = ContextMock::new(vec![1]);

        match number(&mut context_mock, 0.0, 2.0, 0, &Opts::new(1, 1)) {
            Ok(value) => assert_eq!(value, 1.0, "expected: 1.0, got: {}", value),
            Err(error) => panic!("expected result, got: {}", error),
        }
    }

    #[test]
    fn test_choose_number_multiple_pass() {
        let mut context_mock = ContextMock::new(vec![0, 2, 2]);

        match number(&mut context_mock, 0.0, 2.0, 0, &Opts::new(3, 1)) {
            Ok(value) => assert_eq!(value, 2.0, "expected: 2.0, got: {}", value),
            Err(error) => panic!("expected result, got: {}", error),
        }
    }

    #[test]
    fn test_choose_number_precision() {
        let mut context_mock = ContextMock::new(vec![1]);

        match number(&mut context_mock, 0.0, 2.0, 1, &Opts::new(1, 1)) {
            Ok(value) => assert_eq!(value, 0.1, "expected: 0.1, got: {}", value),
            Err(error) => panic!("expected result, got: {}", error),
        }
    }

    #[test]
    fn test_choose_numbers() {
        let mut context_mock = ContextMock::new(vec![0, 2, 1]);

        match numbers(&mut context_mock, 0.0, 2.0, 0, &Opts::new(1, 3)) {
            Ok(values) => assert_eq!(
                values,
                [0.0, 2.0, 1.0],
                "expected: [0.0, 2.0, 1.0], got: {:?}",
                values
            ),
            Err(error) => panic!("expected result, got: {}", error),
        }
    }

    #[test]
    fn test_choose_index() {
        let mut context_mock = ContextMock::new(vec![0]);

        match index(&mut context_mock, vec![0, 1, 2], &Opts::new(1, 1)) {
            Ok(index) => assert_eq!(index, 0, "expected: 0, got: {}", index),
            Err(error) => panic!("expected result, got: {}", error),
        }
    }

    #[test]
    fn test_choose_indexes() {
        let mut context_mock = ContextMock::new(vec![0, 2, 1]);

        match indexes(&mut context_mock, vec![0, 1, 2], &Opts::new(1, 3)) {
            Ok(indexes) => assert_eq!(
                indexes,
                [0, 2, 1],
                "expected: [0, 2, 1], got: {:?}",
                indexes
            ),
            Err(error) => panic!("expected result, got: {}", error),
        }
    }
}
