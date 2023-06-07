//! Solution for https://leetcode.com/problems/minimum-bit-flips-to-convert-number
impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        (start ^ goal).count_ones() as _
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(10, 7, 3)]
    #[case::leet2(3, 4, 3)]
    fn test(#[case] start: i32, #[case] goal: i32, #[case] flip: i32) {
        assert_eq!(Solution::min_bit_flips(start, goal), flip);
    }
}
