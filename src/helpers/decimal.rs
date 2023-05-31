#[macro_export]
macro_rules! assert_float_eq {
    ($left:expr, $right:expr  $(, $($arg:tt)+)?) => {
        assert_eq!(
            format!("{:.5}", ($left)),
            format!("{:.5}", ($right))
            $(, $($arg)+)?
        )
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_int() {
        assert_float_eq!(1, 1);
    }

    #[test]
    fn test_float() {
        assert_float_eq!(1, 1);
    }

    #[test]
    fn test_float_different() {
        assert_float_eq!(1.000_001, 1.000_000);
    }

    #[test]
    #[should_panic(expected = "fail float")]
    fn test_float_different_ne() {
        assert_float_eq!(1.000_01, 1.000_00, "fail {}", "float");
    }
}
