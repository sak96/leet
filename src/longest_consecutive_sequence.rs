//! Solution for https://leetcode.com/problems/longest-consecutive-sequence
//! 128. Longest Consecutive Sequence

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::<i32, (i32, i32)>::new();
        for num in nums {
            let max = map.get(&(num + 1)).map(|z| z.1).unwrap_or(num);
            let min = map.get(&(num - 1)).map(|z| z.0).unwrap_or(num);

            let tmp = map.entry(max).or_insert((min, max));
            tmp.0 = tmp.0.min(min);

            let tmp = map.entry(min).or_insert((min, max));
            tmp.1 = tmp.1.max(max);
        }
        map.into_values()
            .map(|(min, max)| max - min + 1)
            .max()
            .unwrap_or(0)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![100,4,200,1,3,2], 4)]
    #[case(vec![0,3,7,2,5,8,4,6,0,1], 9)]
    #[case(vec![1,0,1,2], 3)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::longest_consecutive(nums);
        assert_eq!(actual, expected);
    }
}
