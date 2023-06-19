//! Solution for https://leetcode.com/problems/find-the-highest-altitude
impl Solution {
    pub fn largest_altitude(mut gain: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut cur = 0;
        for gain in gain.drain(..) {
            cur += gain;
            max = max.max(cur);
        }
        max
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::leet1([-5,1,5,0,-7], 1)]
    #[case::leet1([-4,-3,-2,-1,4,3,2], 0)]
    fn case(#[case] gain: impl AsRef<[i32]>, #[case] output: i32) {
        assert_eq!(Solution::largest_altitude(gain.as_ref().to_vec()), output);
    }
}
