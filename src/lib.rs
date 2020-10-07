pub mod equation;

#[macro_export]
macro_rules! assert_almost_eq {
    ($expected: expr, $actual: expr, $acceptable_error: expr) => {
        if ($expected - $actual).abs() >= $acceptable_error {
            panic!(
                r#"assertion failed: `(left == right)`
  left: `{:?}`,
 right: `{:?}`"#,
                $expected, $actual
            )
        }
    };
    ($expected: expr, $actual: expr) => {
        assert_almost_eq!($expected, $actual, 1e-6);
    };
}
