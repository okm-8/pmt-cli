use crate::math::discrete::average::arithmetic_mean;

pub fn variance(values: Vec<f64>, expectation: f64) -> f64 {
    values
        .iter()
        .map(|value| {
            let sample = *value - expectation;

            sample.powi(2)
        })
        .sum::<f64>()
        / (values.len() as f64 - 1.0)
}

pub fn standard_deviation(values: Vec<f64>, expectation: f64) -> f64 {
    variance(values, expectation).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variance() {
        let sequence = vec![1.0, 2.0, 3.0];
        let expectation = arithmetic_mean(sequence.clone());
        let variance_value = variance(sequence.clone(), expectation.clone());

        let sum_of_diff_squares: f64 = sequence
            .iter()
            .map(|value| (value - expectation).powi(2))
            .sum();
        assert_eq!(variance_value, sum_of_diff_squares / 2.0);
    }

    #[test]
    fn test_standard_deviation() {
        let sequence = vec![1.0, 2.0, 3.0];
        let expectation = arithmetic_mean(sequence.clone());
        let standard_deviation_value = standard_deviation(sequence.clone(), expectation.clone());

        let sum_of_diff_squares: f64 = sequence
            .iter()
            .map(|value| (value - expectation).powi(2))
            .sum();
        assert_eq!(standard_deviation_value, (sum_of_diff_squares / 2.0).sqrt());
    }
}
