//! Solution for https://leetcode.com/problems/longest-arithmetic-subsequence
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap();
        let diff = max - *nums.iter().min().unwrap();
        let mut memory = vec![0; max as usize + 1];
        (-diff..=diff)
            .map(|diff| {
                memory.iter_mut().for_each(|x| *x = 0);
                for i in &nums {
                    memory[*i as usize] =
                        memory[*i as usize].max(memory.get((i - diff) as usize).unwrap_or(&0) + 1);
                }
                *memory.iter().max().unwrap()
            })
            .max()
            .unwrap()
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::leet1([3,6,9,12],4)]
    #[case::leet2([9,4,7,2,10],3)]
    #[case::leet3([20,1,15,3,10,5,8],4)]
    #[case::leet3([3,6],2)]
    fn test(#[case] nums: impl AsRef<[i32]>, #[case] output: i32) {
        assert_eq!(
            Solution::longest_arith_seq_length(nums.as_ref().into()),
            output
        );
    }
}
