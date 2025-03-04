//! Solution for https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three
//! 1780. Check if Number is a Sum of Powers of Three

impl Solution {
    pub fn check_powers_of_three(mut n: i32) -> bool {
        while n > 0 {
            if n % 3 == 2 {
                return false;
            }
            n /= 3;
        }

        true
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(12, true)]
    #[case(91, true)]
    #[case(21, false)]
    fn case(#[case] n: i32, #[case] expected: bool) {
        let actual = Solution::check_powers_of_three(n);
        assert_eq!(actual, expected);
    }
}
