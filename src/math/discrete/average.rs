pub fn arithmetic_mean(values: Vec<f64>) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

pub fn geometric_mean(values: Vec<f64>) -> f64 {
    let product = values.iter().fold(1.0, |acc, value| acc * value);

    return product.powf(1.0 / values.len() as f64);
}

pub fn harmonic_mean(values: Vec<f64>) -> f64 {
    values.len() as f64 / values.iter().fold(0.0, |acc, value| acc + 1.0 / value)
}

pub fn arbitrary_median(values: Vec<f64>) -> f64 {
    let middle = values.len() / 2;

    if values.len() % 2 == 0 {
        return arithmetic_mean(vec![values[middle - 1], values[middle]]);
    }

    return values[middle];
}

pub fn median(values: Vec<f64>) -> f64 {
    let mut sorted_values = values.clone();
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    return arbitrary_median(sorted_values);
}

pub fn modes(values: Vec<f64>) -> Vec<f64> {
    let mut counters: Vec<u16> = vec![0; values.len()];

    for index in 0..values.len() {
        for value in values.iter() {
            if values[index] == *value {
                counters[index] += 1;
            }
        }
    }

    let mut modes: Vec<f64> = Vec::new();
    let mut max_counter = 0;
    for index in 0..counters.len() {
        if counters[index] > max_counter {
            modes.clear();
            modes.push(values[index]);
            max_counter = counters[index];
        } else if counters[index] == max_counter {
            if !modes.contains(&values[index]) {
                modes.push(values[index]);
            }
        }
    }

    return modes;
}

pub fn midrange(values: Vec<f64>) -> f64 {
    arithmetic_mean(vec![
        values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone(),
        values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone()
    ])
}

pub fn range(values: Vec<f64>) -> f64 {
    values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
        - values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_mean() {
        let sequence = vec![1.0, 2.0, 3.0];
        let mean_value = arithmetic_mean(sequence.clone());

        let sum: f64 = sequence.iter().sum();
        assert_eq!(mean_value, sum / 3.0);
    }

    #[test]
    fn test_geometric_mean() {
        let mean_value = geometric_mean(vec![1.0, 2.0, 3.0]);

        let product: f64 = vec![1.0, 2.0, 3.0].iter().product();
        assert_eq!(mean_value, product.powf(1.0 / 3.0));
    }

    #[test]
    fn test_harmonic_mean() {
        let sequence = vec![1.0, 2.0, 3.0];
        let mean_value = harmonic_mean(sequence.clone());

        let harmonic_sum: f64 = sequence.iter().map(|value| 1.0 / value).sum();
        assert_eq!(mean_value, 3.0 / harmonic_sum);
    }

    #[test]
    fn test_arbitrary_median() {
        let arbitrary_median_value = arbitrary_median(vec![1.0, 2.0, 3.0]);

        assert_eq!(arbitrary_median_value, 2.0);
    }

    #[test]
    fn test_arbitrary_median_even() {
        let arbitrary_median_value = arbitrary_median(vec![1.0, 2.0, 3.0, 4.0]);

        assert_eq!(arbitrary_median_value, 2.5);
    }

    #[test]
    fn test_median() {
        let median_value = median(vec![1.0, 3.0, 2.0]);

        assert_eq!(median_value, 2.0);
    }

    #[test]
    fn test_modes() {
        let modes_value = modes(vec![1.0, 2.0, 3.0, 2.0]);

        assert_eq!(modes_value, vec![2.0]);
    }

    #[test]
    fn test_modes_multiple() {
        let modes_value = modes(vec![1.0, 2.0, 3.0, 2.0, 3.0]);

        assert_eq!(modes_value, vec![2.0, 3.0]);
    }

    #[test]
    fn test_midrange() {
        let midrange_value = midrange(vec![1.0, 3.0, 2.0]);

        assert_eq!(midrange_value, (1.0 + 3.0) / 2.0);
    }

    #[test]
    fn test_range() {
        let range_value = range(vec![1.0, 3.0, 2.0]);

        assert_eq!(range_value, 3.0 - 1.0);
    }
}
