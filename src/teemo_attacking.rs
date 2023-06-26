//! Solution for https://leetcode.com/problems/teemo-attacking
//! 495. Teemo Attacking

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        time_series.windows(2).map(|x| (x[1] - x[0]).min(duration)).sum::<i32>() + duration
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,4], 2, 4)]
    #[case(vec![1,2], 2, 3)]
    fn case(#[case] time_series: Vec<i32>, #[case] duration: i32, #[case] expected: i32) {
        let actual = Solution::find_poisoned_duration(time_series, duration);
        assert_eq!(actual, expected);
    }
}
