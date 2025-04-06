//! Solution for https://leetcode.com/problems/maximum-candies-allocated-to-k-children
//! 2226. Maximum Candies Allocated to K Children

impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut max = candies.iter().cloned().sum::<i32>() as i64 / k;
        let mut min = 0;
        while max > min {
            let middle = (max + min + 1) / 2;
            if candies.iter().map(|&c| (c as i64 / middle)).sum::<i64>() >= k {
                min = middle;
            } else {
                max = middle - 1;
            }
        }
        min as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![5,8,6], 3, 5)]
    #[case(vec![2,5], 11, 0)]
    fn case(#[case] candies: Vec<i32>, #[case] k: i64, #[case] expected: i32) {
        let actual = Solution::maximum_candies(candies, k);
        assert_eq!(actual, expected);
    }
}
