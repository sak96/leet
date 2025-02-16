//! Solution for https://leetcode.com/problems/string-to-integer-atoi
//! 8. String to Integer (atoi)

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim_start().as_bytes();
        let mut it = s.iter().peekable();
        let sign = match it.peek() {
            Some(b'+') => {
                it.next();
                1
            }
            Some(b'-') => {
                it.next();
                -1
            }
            _ => 1,
        };
        let mut num = 0i32;
        while let Some(a) = it.next() {
            if (b'0'..=b'9').contains(a) {
                num = num
                    .saturating_mul(10)
                    .saturating_add((a - b'0') as i32 * sign);
            } else {
                break;
            }
        }
        num
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("42", 42)]
    #[case("   -042", -42)]
    #[case("1337c0d3", 1337)]
    #[case("0-1", 0)]
    #[case("words and 987", 0)]
    #[case("   --042", 0)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::my_atoi(s);
        assert_eq!(actual, expected);
    }
}
