//! Solution for https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst

use std::collections::HashMap;
impl Solution {
    const MOD: i64 = 10i64.pow(9) + 7;

    pub fn combination(n: i64, r: i64, combinations: &mut HashMap<(i64, i64), i64>) -> i64 {
        let key = (n, r);
        if let Some(&value) = combinations.get(&key) {
            value
        } else {
            let value = if r == 0 || n == r {
                1
            } else if r == 1 {
                n
            } else {
                Self::combination(n - 1, r - 1, combinations)
                    + Self::combination(n - 1, r, combinations)
            } % Self::MOD;
            combinations.insert(key, value);
            value
        }
    }

    pub fn nums_of_ways_(nums: &mut [i32], combinations: &mut HashMap<(i64, i64), i64>) -> i64 {
        if nums.is_empty() {
            1
        } else {
            let (x, nums) = nums.split_at_mut(1);
            let n = nums.len() as i64;
            nums.sort_by_key(|i| x[0] < *i);
            let (left, right) = nums.split_at_mut(nums.partition_point(|i| x[0] > *i));
            let r = left.len().min(right.len()) as i64;
            let mut result = Self::combination(n, r, combinations);
            result *= Self::nums_of_ways_(left, combinations);
            result %= Self::MOD;
            result *= Self::nums_of_ways_(right, combinations);
            result % Self::MOD
        }
    }
    pub fn num_of_ways(mut nums: Vec<i32>) -> i32 {
        Self::nums_of_ways_(&mut nums, &mut HashMap::new()) as i32 - 1
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
    #[case::leet5([10,23,12,18,4,29,2,8,41,31,25,21,14,35,26,5,19,43,22,37,9,20,44,28,1,39,30,38,36,6,13,16,27,17,34,7,15,3,11,24,42,33,40,32],182440977)]
    #[case::leet5([74,24,70,11,6,4,59,9,36,82,80,30,46,31,22,34,8,69,32,57,18,21,37,83,55,38,41,72,48,65,27,60,73,58,68,50,16,77,75,20,81,3,61,13,10,29,62,49,12,66,39,45,28,40,42,52,78,56,44,17,14,67,35,26,19,5,63,51,43,23,79,2,54,47,76,53,7,25,64,33,1,15,71],901891111)]
    #[case::leet5([81,210,76,102,218,37,154,157,114,123,221,145,198,65,117,189,239,206,93,168,48,159,187,214,101,180,173,222,58,72,10,75,60,223,64,227,103,107,11,33,153,116,146,13,50,122,236,161,127,82,163,38,201,194,8,207,80,31,53,217,104,164,174,128,110,200,18,3,131,197,9,71,35,41,151,130,79,195,69,24,73,133,216,36,167,186,243,208,67,28,137,108,183,152,150,61,98,176,16,232,215,115,118,74,182,25,12,226,66,188,51,112,244,229,29,178,191,43,111,225,230,91,241,19,142,181,21,126,78,140,68,228,141,169,2,49,202,4,233,45,119,32,85,158,20,136,96,14,15,165,237,213,220,7,100,88,106,26,57,47,143,162,56,94,95,179,105,192,149,5,44,193,135,109,139,196,22,234,70,199,55,1,235,39,89,34,23,62,166,184,171,92,170,59,129,83,242,172,177,240,132,175,190,138,120,185,212,52,77,84,121,87,86,54,97,224,203,155,144,17,40,134,6,30,42,231,113,238,219,211,205,125,147,90,99,156,46,209,27,124,148,160,204,63],126227162)]
    fn test(#[case] nums: impl AsRef<[i32]>, #[case] output: i32) {
        assert_eq!(Solution::num_of_ways(nums.as_ref().to_vec()), output);
    }
}
