//! Solution for https://leetcode.com/problems/search-in-rotated-sorted-array
//! 33. Search in Rotated Sorted Array

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let last = nums.last().expect("1 <= nums.length");
        let part = nums.partition_point(|x| x > last);
        if part > 0 && target >= nums[0] && target <= nums[part - 1] {
            nums[0..part]
                .binary_search(&target)
                .map(|x| x as i32)
                .unwrap_or(-1)
        } else {
            nums[part..]
                .binary_search(&target)
                .map(|x| (x + part) as i32)
                .unwrap_or(-1)
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![4,5,6,7,0,1,2], 0, 4)]
    #[case(vec![4,5,6,7,0,1,2], 3, -1)]
    #[case(vec![1], 0, -1)]
    #[case(vec![1], 1, 0)]
    #[case(vec![3, 1], 3, 0)]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::search(nums, target);
        assert_eq!(actual, expected);
    }
}
