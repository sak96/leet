use std::cmp::Reverse;
impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut nums: Vec<_> = nums1.into_iter().zip(nums2.into_iter()).collect();
        nums.sort_unstable_by_key(|&(_, b)| b);
        let mut binary_heap = std::collections::BinaryHeap::new();
        let mut ans = 0i64;
        let mut sum = 0i64;
        // Use first k - 1 numbers
        for _ in 0..k {
            let x = nums.pop().expect("1 <= k <= n");
            sum += x.0 as i64;
            ans = x.1 as i64;
            binary_heap.push(Reverse(x.0));
        }
        ans *= sum;
        // remove the smallest one from sum
        while let Some(x) = nums.pop() {
            // update sum with max of k of k+1 number
            binary_heap.push(Reverse(x.0));
            sum += (x.0 - binary_heap.pop().expect("1 <= k").0) as i64;
            // update answer with max possibility
            ans = ans.max(sum * x.1 as i64);
        }
        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leetcode1(
        [1,3,3,2].to_vec(),
        [2,1,3,4].to_vec(),
        3,
        12
    )]
    #[case::leetcode2(
        [4,2,3,1,1].to_vec(),
        [7,5,10,9,6].to_vec(),
        1,
        30
    )]
    #[case::leetcode3(
        [2,1,14,12].to_vec(),
        [11,7,13,6].to_vec(),
        3,
        168
    )]
    #[case::leetcode4(
        [2,1,14,12].to_vec(),
        [11,7,13,6].to_vec(),
        3,
        168
    )]
    #[case::leetcode5(
        [7,6,1,10].to_vec(),
        [0,0,10,0].to_vec(),
        2,
        0
    )]
    #[case::mine1(
        [1,12,3,2].to_vec(),
        [2,1,3,4].to_vec(),
        3,
        17
    )]
    fn test(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] k: i32, #[case] output: i64) {
        assert_eq!(Solution::max_score(nums1, nums2, k), output);
    }
}
