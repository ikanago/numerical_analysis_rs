use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum BisectionError {
    #[error("Solution not found in a given range")]
    SolutionNotFound,
    #[error("Invalid specification of a solution range")]
    InvalidRange,
}

/// Find solution of equation by using bisection method.
///
/// # Examples
///
/// ```
/// use numerical_analysis_rs::equation::bisection::*;
///
/// let function = |x: f64| x.powf(3.0) - 2.0 * x.powf(2.0) - x + 2.0;
/// let solution = bisection(function, 0.9, 1.2, 1e-6).unwrap();
/// if (solution - 1.0).abs() >= 1e-6 {
///     panic!()
/// }
/// ```
pub fn bisection<F>(
    function: F,
    left: f64,
    right: f64,
    acceptable_error: f64,
) -> Result<f64, BisectionError>
where
    F: Fn(f64) -> f64,
{
    if left > right {
        Err(BisectionError::InvalidRange)
    } else if function(left) * function(right) > 0.0 {
        Err(BisectionError::SolutionNotFound)
    } else {
        bisection_rec(function, left, right, acceptable_error)
    }
}

fn bisection_rec<F>(
    function: F,
    left: f64,
    right: f64,
    acceptable_error: f64,
) -> Result<f64, BisectionError>
where
    F: Fn(f64) -> f64,
{
    let mid = (left + right) / 2.0;
    if function(mid).abs() < acceptable_error {
        Ok(mid)
    } else if function(left) * function(mid) < 0.0 {
        bisection_rec(function, left, mid, acceptable_error)
    } else {
        bisection_rec(function, mid, right, acceptable_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_almost_eq;
    const ACCEPTABLE_ERROR: f64 = 1e-6;

    #[test]
    fn bisection_test1() {
        let function = |x: f64| x.powf(2.0) - x.cos();
        let actual = bisection(function, 0.5, 1.2, ACCEPTABLE_ERROR).unwrap();
        assert_almost_eq!(0.824132, actual);
    }

    #[test]
    fn bisection_test2() {
        let function = |x: f64| x.exp() * x.sin() - 2.0 * x;
        let actual = bisection(function, 2.5, 3.0, ACCEPTABLE_ERROR).unwrap();
        assert_almost_eq!(2.792311, actual);
    }

    #[test]
    fn bisection_test3() {
        let function = |x: f64| x.tanh() + x + 1.0;
        let actual = bisection(function, -1.0, 0.0, ACCEPTABLE_ERROR).unwrap();
        assert_almost_eq!(-0.521298, actual);
    }

    #[test]
    fn solution_not_found() {
        let function = |x: f64| x.tanh() + x + 1.0;
        let actual = bisection(function, 0.0, 1.0, ACCEPTABLE_ERROR);
        assert_eq!(Err(BisectionError::SolutionNotFound), actual);
    }

    #[test]
    fn invalid_range() {
        let function = |x: f64| x.tanh() + x + 1.0;
        let actual = bisection(function, 2.0, 1.0, ACCEPTABLE_ERROR);
        assert_eq!(Err(BisectionError::InvalidRange), actual);
    }
}
