impl Solution {
    pub fn is_number(s: String) -> bool {
        let is_sign = |&x: &char| ['+', '-'].contains(&x);
        let is_exp = |&x: &char| ['e', 'E'].contains(&x);
        let is_dot = |&x: &char| '.'.eq(&x);
        let mut chars = s.chars().peekable();

        // consume sign if present
        chars.next_if(is_sign);

        // consume digits
        let mut digits = false;
        while matches!(chars.peek(), Some(x) if x.is_ascii_digit()) {
            digits = true;
            chars.next();
        }

        if chars.next_if(is_dot).is_some() {
            // consume digits for decimal
            while matches!(chars.peek(), Some(x) if x.is_ascii_digit()) {
                digits = true;
                chars.next();
            }
        }

        if !digits {
            // is not valid integer or decimal
            return false;
        }

        let mut exp_valid = true;
        // consume exponent if present
        if chars.next_if(is_exp).is_some() {
            // mut have digits
            exp_valid = false;
            // consume sign if present
            chars.next_if(is_sign);
            while matches!(chars.peek(), Some(x) if x.is_ascii_digit()) {
                exp_valid = true;
                chars.next();
            }
        }
        digits && exp_valid && chars.next().is_none()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("2")]
    #[case("0089")]
    #[case("-0.1")]
    #[case("+3.14")]
    #[case("4.")]
    #[case("-.9")]
    #[case("2e10")]
    #[case("-90E3")]
    #[case("3e+7")]
    #[case("+6e-1")]
    #[case("53.5e93")]
    #[case("-123.456e789")]
    fn positive_test_case(#[case] input: &str) {
        assert!(Solution::is_number(input.to_string()), "{}", input);
    }

    #[rstest]
    #[case("abc")]
    #[case("1a")]
    #[case("1e")]
    #[case("e3")]
    #[case("99e2.5")]
    #[case("--6")]
    #[case("-+3")]
    #[case("95a54e53")]
    fn negative_test_case(#[case] input: &str) {
        assert!(!Solution::is_number(input.to_string()), "{}", input);
    }
}
