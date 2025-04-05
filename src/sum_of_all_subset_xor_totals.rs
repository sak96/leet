//! Solution for https://leetcode.com/problems/sum-of-all-subset-xor-totals
//! 1863. Sum of All Subset XOR Totals

impl Solution {
    pub fn _subset_xor_sum(nums: &[i32], mem: &mut Vec<i32>) -> i32 {
        if let Some((n, nums)) = nums.split_first() {
            let mut count = *n;
            for i in 0..mem.len() {
                let n = mem[i] ^ n;
                count += n;
                mem.push(n);
            }
            mem.push(*n);
            count + Self::_subset_xor_sum(nums, mem)
        } else {
            0
        }
    }

    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        Self::_subset_xor_sum(&nums, &mut vec![])
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3], 6)]
    #[case(vec![5,1,6], 28)]
    #[case(vec![3,4,5,6,7,8], 480)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::subset_xor_sum(nums);
        assert_eq!(actual, expected);
    }
}
