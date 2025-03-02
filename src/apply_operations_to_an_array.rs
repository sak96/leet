//! Solution for https://leetcode.com/problems/apply-operations-to-an-array
//! 2460. Apply Operations to an Array

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let mut output = vec![0; nums.len()];
        nums.reverse();
        let mut prev = nums.pop().unwrap();
        let mut i = 0;
        while let Some(mut next) = nums.pop() {
            if prev == 0 {
            } else if prev == next {
                output[i] = prev + next;
                i += 1;
                next = 0;
            } else {
                output[i] = prev;
                i += 1;
            }
            prev = next;
        }
        output[i] = prev;
        output
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,2,1,1,0], vec![1,4,2,0,0,0])]
    #[case(vec![0,1], vec![1,0])]
    #[case(vec![847,847,0,0,0,399,416,416,879,879,206,206,206,272], vec![1694,399,832,1758,412,206,272,0,0,0,0,0,0,0])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::apply_operations(nums);
        assert_eq!(actual, expected);
    }
}
