/// Find solution of equation by using bisection method.
pub fn bisection<F>(function: F, left: f64, right: f64, acceptable_error: f64) -> Option<f64>
where
    F: Fn(f64) -> f64,
{
    if function(left) * function(right) > 0.0 {
        None
    } else if left > right {
        // Todo: return error
        None
    } else {
        bisection_rec(function, left, right, acceptable_error)
    }
}

fn bisection_rec<F>(function: F, left: f64, right: f64, acceptable_error: f64) -> Option<f64>
where
    F: Fn(f64) -> f64,
{
    let mid = (left + right) / 2.0;
    if function(mid).abs() < acceptable_error {
        return Some(mid);
    } else if function(left) * function(mid) < 0.0 {
        bisection_rec(function, left, mid, acceptable_error)
    } else {
        bisection_rec(function, mid, right, acceptable_error)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_almost_eq;
    use crate::equation::bisection::*;
    const ACCEPTABLE_ERROR: f64 = 1e-6;

    #[test]
    fn bisection_test1() {
        let function = |x: f64| x.powf(3.0) - 2.0 * x.powf(2.0) - x + 2.0;
        let expected = 1.0;
        let actual = bisection(function, 0.9, 1.2, ACCEPTABLE_ERROR).unwrap();
        assert_almost_eq!(expected, actual);
    }

    #[test]
    fn bisection_test2() {
        let function = |x: f64| x.powf(2.0) - x.cos();
        let expected = 0.824132;
        let actual = bisection(function, 0.5, 1.2, ACCEPTABLE_ERROR).unwrap();
        assert_almost_eq!(expected, actual);
    }

    #[test]
    fn bisection_test3() {
        let function = |x: f64| x.exp() * x.sin() - 2.0 * x;
        let expected = 2.792311;
        let actual = bisection(function, 2.5, 3.0, ACCEPTABLE_ERROR).unwrap();
        assert_almost_eq!(expected, actual);
    }

    #[test]
    fn bisection_test4() {
        let function = |x: f64| x.tanh() + x + 1.0;
        let expected = -0.521298;
        let actual = bisection(function, -1.0, 0.0, ACCEPTABLE_ERROR).unwrap();
        assert_almost_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn bisection_test5() {
        let function = |x: f64| x.tanh() + x + 1.0;
        bisection(function, 0.0, 1.0, ACCEPTABLE_ERROR).unwrap();
    }
}
