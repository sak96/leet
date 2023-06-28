//! Solution for https://leetcode.com/problems/find-k-pairs-with-smallest-sums
//! 373. Find K Pairs with Smallest Sums

impl Solution {
    pub fn k_smallest_pairs(mut nums1: Vec<i32>, mut nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums1.truncate(k as usize);
        nums2.truncate(k as usize);
        let mut heap = std::collections::BinaryHeap::new();
        for (i, &n1) in nums1.iter().enumerate() {
            heap.push((-n1 - nums2[0], i, 0));
        }
        let mut output = Vec::with_capacity(k as usize);
        for _ in 0..k {
            if let Some((_, i, j)) = heap.pop() {
                output.push(vec![nums1[i], nums2[j]]);
                if let Some(value) = nums2.get(j + 1) {
                    heap.push((-nums1[i] - value, i, j + 1));
                }
            } else {
                break;
            }
        }
        output
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,7,11], vec![2,4,6], 3, vec![vec![1,2],vec![1,4],vec![1,6]])]
    #[case(vec![1,1,2], vec![1,2,3], 2, vec![vec![1,1],vec![1,1]])]
    #[case(vec![1,2], vec![3], 3, vec![vec![1,3],vec![2,3]])]
    fn case(
        #[case] nums1: Vec<i32>,
        #[case] nums2: Vec<i32>,
        #[case] k: i32,
        #[case] expected: Vec<Vec<i32>>,
    ) {
        let actual = Solution::k_smallest_pairs(nums1, nums2, k);
        assert_eq!(actual, expected);
    }
}
