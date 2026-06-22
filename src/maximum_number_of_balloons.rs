//! Solution for https://leetcode.com/problems/maximum-number-of-balloons
//! 1189. Maximum Number of Balloons

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut counter = [0; (b'z' - b'a') as usize + 1];
        let text = text.as_bytes();
        for &i in text {
            counter[(i - b'a') as usize] += 1
        }
        let mut min = i32::MAX;
        for i in [b'b', b'a', b'n'] {
            min = min.min(counter[(i - b'a') as usize]);
        }
        for i in [b'l', b'o'] {
            min = min.min(counter[(i - b'a') as usize] / 2);
        }
        min
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("nlaebolko", 1)]
    #[case("loonbalxballpoon", 2)]
    #[case("leetcode", 0)]
    fn case(#[case] text: String, #[case] expected: i32) {
        let actual = Solution::max_number_of_balloons(text);
        assert_eq!(actual, expected);
    }
}
