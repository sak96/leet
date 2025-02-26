//! Solution for https://leetcode.com/problems/first-missing-positive
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let mut index = 0;
        let mut positives = nums.len();
        let len = nums.len() as i32;
        while index < len {
            let item = nums[index as usize] - 1;
            if item >= 0 && item < len && nums.get(index as usize) != nums.get(item as usize) {
                nums.swap(index as usize, item as usize);
            } else {
                if item < 0 {
                    positives -= 1;
                }
                if item != index {
                    nums[index as usize] = 0;
                }
                index += 1;
            }
        }
        nums.into_iter()
            .enumerate()
            .find_map(|(i, x)| 0.eq(&x).then_some(i))
            .unwrap_or(positives) as i32
            + 1
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::leet1([1,2,0], 3)]
    #[case::leet2([3,4,-1,1], 2)]
    #[case::leet3([7,8,9,11,12], 1)]
    #[case::leet3([1,1], 2)]
    #[case::leet3([1,2], 3)]
    #[case::leet3([2], 1)]
    fn test<const N: usize>(#[case] nums: [i32; N], #[case] output: i32) {
        assert_eq!(Solution::first_missing_positive(nums.to_vec()), output);
    }
}
