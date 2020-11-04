pub fn newton<F, D>(function: F, derivative: D, initial_value: f64, acceptable_error: f64) -> f64
where
    F: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    let mut start = initial_value;
    let mut solution = start;
    while function(solution).abs() > acceptable_error {
        solution = start - function(start) / derivative(start);
        start = solution
    }
    solution
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_almost_eq;
    const ACCEPTABLE_ERROR: f64 = 1e-6;

    #[test]
    fn newton_test1() {
        let function = |x: f64| x.powf(2.0) - x.cos();
        let derivative = |x: f64| 2.0 * x + x.sin();
        let actual = newton(function, derivative, 1.0, ACCEPTABLE_ERROR);
        assert_almost_eq!(0.824132, actual);
    }

    #[test]
    fn newton_multiple_root() {
        let function = |x: f64| (x - 3.0).powf(3.0);
        let derivative = |x: f64| 3.0 * (x - 3.0).powf(2.0);
        let actual = newton(function, derivative, 1.0, ACCEPTABLE_ERROR);
        assert_almost_eq!(3.00000, actual, 1e-2);
    }
}
