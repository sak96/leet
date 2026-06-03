//! Solution for https://leetcode.com/problems/earliest-finish-time-for-land-and-water-rides-ii
//! 3635. Earliest Finish Time for Land and Water Rides II

impl Solution {
    pub fn earliest_finish_time_(
        first_start_time: &[i32],
        first_duration: &[i32],
        second_start_time: &[i32],
        second_duration: &[i32],
    ) -> i32 {
        let mut first_time = i32::MAX;
        for (t, d) in first_start_time.iter().zip(first_duration.iter()) {
            first_time = first_time.min(t + d);
        }
        let mut second_time = i32::MAX;
        for (&t, d) in second_start_time.iter().zip(second_duration.iter()) {
            second_time = second_time.min(first_time.max(t) + d);
        }
        second_time
    }

    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        Self::earliest_finish_time_(
            &land_start_time,
            &land_duration,
            &water_start_time,
            &water_duration,
        )
        .min(Self::earliest_finish_time_(
            &water_start_time,
            &water_duration,
            &land_start_time,
            &land_duration,
        ))
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,8], vec![4,1], vec![6], vec![3], 9)]
    #[case(vec![5], vec![3], vec![1], vec![10], 14)]
    fn case(
        #[case] land_start_time: Vec<i32>,
        #[case] land_duration: Vec<i32>,
        #[case] water_start_time: Vec<i32>,
        #[case] water_duration: Vec<i32>,
        #[case] expected: i32,
    ) {
        let actual = Solution::earliest_finish_time(
            land_start_time,
            land_duration,
            water_start_time,
            water_duration,
        );
        assert_eq!(actual, expected);
    }
}
