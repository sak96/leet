//! Solution for https://leetcode.com/problems/count-subarrays-with-fixed-bounds
//! 2444. Count Subarrays With Fixed Bounds

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut min = 0;
        let mut max = 0;
        let mut left = 0;
        let mut count = 0i64;
        for (i, n) in nums.into_iter().enumerate() {
            match n.cmp(&min_k) {
                std::cmp::Ordering::Less => left = i + 1,
                std::cmp::Ordering::Equal => min = i + 1,
                std::cmp::Ordering::Greater => {}
            }
            match n.cmp(&max_k) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => max = i + 1,
                std::cmp::Ordering::Greater => left = i + 1,
            }
            count += min.min(max).saturating_sub(left) as i64
        }
        count
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,5,2,7,5], 1, 5, 2)]
    #[case(vec![1,1,1,1], 1, 1, 10)]
    fn case(#[case] nums: Vec<i32>, #[case] min_k: i32, #[case] max_k: i32, #[case] expected: i64) {
        let actual = Solution::count_subarrays(nums, min_k, max_k);
        assert_eq!(actual, expected);
    }
}
