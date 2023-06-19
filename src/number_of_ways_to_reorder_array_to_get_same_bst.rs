//! Solution for https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst
impl Solution {
    const MOD: i64 = 10i64.pow(9) + 7;

    pub fn combination(len: usize) -> Vec<Vec<i32>> {
        let mut combinations = Vec::with_capacity(len);
        combinations.push(vec![1]);
        for n in 1..len {
            let vec = (0..=n / 2)
                .into_iter()
                .map(|r| {
                    if r == 0 {
                        1
                    } else {
                        (combinations[n - 1][r.min(n - 1 - r)]
                            + combinations[n - 1][(r - 1).min(n - r)])
                            % Self::MOD as i32
                    }
                })
                .collect();
            combinations.push(vec);
        }
        combinations
    }

    pub fn nums_of_ways_(nums: &mut [i32], combinations: &mut [Vec<i32>]) -> i64 {
        if nums.is_empty() {
            1
        } else {
            let (x, nums) = nums.split_at_mut(1);
            let n = nums.len();
            nums.sort_by_key(|i| x[0] < *i);
            let (left, right) = nums.split_at_mut(nums.partition_point(|i| x[0] > *i));
            let r = left.len().min(right.len());
            let mut result: i64 = combinations[n][r] as _;
            result *= Self::nums_of_ways_(left, combinations);
            result %= Self::MOD;
            result *= Self::nums_of_ways_(right, combinations);
            result %= Self::MOD;
            result as _
        }
    }
    pub fn num_of_ways(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        Self::nums_of_ways_(&mut nums, &mut Self::combination(len)) as i32 - 1
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::leet1([2,1,3],1)]
    #[case::leet2([3,4,5,1,2],5)]
    #[case::leet3([1,2,3],0)]
    #[case::leet4([9,4,2,1,3,6,5,7,8,14,11,10,12,13,16,15,17,18],216212978)]
    fn test(#[case] nums: impl AsRef<[i32]>, #[case] output: i32) {
        assert_eq!(Solution::num_of_ways(nums.as_ref().to_vec()), output);
    }
}
