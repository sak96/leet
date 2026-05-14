//! Solution for https://leetcode.com/problems/check-if-array-is-good
//! 2784. Check if Array is Good

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let mut seen = vec![0u8; nums.len()];
        let len = nums.len() - 1;
        for i in nums {
            let i = i as usize;
            match i.cmp(&len) {
                std::cmp::Ordering::Less if seen[i] != 0 => return false,
                std::cmp::Ordering::Equal if seen[i] > 1 => return false,
                std::cmp::Ordering::Greater => return false,
                _ => {
                    seen[i] += 1;
                }
            }
        }
        true
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2, 1, 3], false)]
    #[case(vec![1, 3, 3, 2], true)]
    #[case(vec![1, 1], true)]
    #[case(vec![3, 4, 4, 1, 2, 1], false)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::is_good(nums);
        assert_eq!(actual, expected);
    }
}
