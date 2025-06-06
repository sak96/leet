//! Solution for https://leetcode.com/problems/count-days-without-meetings
//! 3169. Count Days Without Meetings

impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_unstable_by_key(|m| m[0]);
        let mut last_meeting_day = 1;
        let mut free_day = 0;
        for m in meetings {
            free_day += (m[0] - last_meeting_day).max(0);
            last_meeting_day = last_meeting_day.max(m[1] + 1);
        }
        free_day + (days + 1 - last_meeting_day).max(0)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(10, vec![vec![5,7],vec![1,3],vec![9,10]], 2)]
    #[case(5, vec![vec![2,4],vec![1,3]], 1)]
    #[case(6, vec![vec![1,6]], 0)]
    fn case(#[case] days: i32, #[case] meetings: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::count_days(days, meetings);
        assert_eq!(actual, expected);
    }
}
