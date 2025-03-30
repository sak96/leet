//! Solution for https://leetcode.com/problems/reverse-integer
//! 7. Reverse Integer

impl Solution {
    pub fn reverse_(x: i32) -> Option<i32> {
        let sign = x.signum();
        let mut x = x.abs();
        let mut output = 0;
        while x > 0 {
            output = 10i32.checked_mul(output)?.checked_add(x % 10)?;
            x = x / 10;
        }
        sign.checked_mul(output)
    }

    pub fn reverse(x: i32) -> i32 {
        Self::reverse_(x).unwrap_or(0)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(123, 321)]
    #[case(-123, -321)]
    #[case(120, 21)]
    #[case(1534236469, 0)]
    #[case(i32::MIN, 0)]
    fn case(#[case] x: i32, #[case] expected: i32) {
        let actual = Solution::reverse(x);
        assert_eq!(actual, expected);
    }
}
