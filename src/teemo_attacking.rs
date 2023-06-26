//! Solution for https://leetcode.com/problems/teemo-attacking
//! 495. Teemo Attacking

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut total_time = 0;
        let mut reset_time = 0;
        for time in time_series {
            total_time += duration - (reset_time - time).max(0);
            reset_time = time + duration;
        }
        total_time
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
