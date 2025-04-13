//! Solution for https://leetcode.com/problems/count-good-numbers
//! 1922. Count Good Numbers

impl Solution {
    const MOD: i64 = 1000000007;
    pub fn quick_exp(n: i32, mut pow: i64) -> i64 {
        let mut res = 1i64;
        let mut mul = n as i64;
        while pow > 0 {
            if pow & 1 == 1 {
                res = (res * mul) % Self::MOD;
            }
            mul = (mul * mul) % Self::MOD;
            std::ops::ShrAssign::shr_assign(&mut pow, 1);
        }
        res
    }
    pub fn count_good_numbers(n: i64) -> i32 {
        (Self::quick_exp(5, (n + 1) / 2) * Self::quick_exp(4, n / 2) % Self::MOD) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, 5)]
    #[case(4, 400)]
    #[case(50, 564908303)]
    fn case(#[case] n: i64, #[case] expected: i32) {
        let actual = Solution::count_good_numbers(n);
        assert_eq!(actual, expected);
    }
}
