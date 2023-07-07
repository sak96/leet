//! Solution for https://leetcode.com/problems/median-of-two-sorted-arrays
//! 4. Median of Two Sorted Arrays

impl Solution {
    pub fn get_kth_element<'a>(mut nums1: &'a [i32], mut nums2: &'a [i32], mut k: usize) -> i32 {
        loop {
            if nums1.len() > nums2.len() {
                std::mem::swap(&mut nums1, &mut nums2);
            } else if nums1.is_empty() {
                break nums2[k - 1];
            } else if k == 1 {
                break nums1[0].min(nums2[0]);
            } else {
                let idx1 = nums1.len().min(k / 2) - 1;
                let idx2 = nums2.len().min(k / 2) - 1;
                if nums1[idx1] <= nums2[idx2] {
                    nums1 = &nums1[idx1 + 1..];
                    k -= idx1 + 1;
                } else {
                    nums2 = &nums2[idx2 + 1..];
                    k -= idx2 + 1;
                }
            }
        }
    }
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n) = (nums1.len(), nums2.len());
        if (m + n) % 2 == 1 {
            Self::get_kth_element(&nums1, &nums2, (m + n) / 2 + 1) as _
        } else {
            (Self::get_kth_element(&nums1, &nums2, (m + n) / 2 + 1)
                + Self::get_kth_element(&nums1, &nums2, (m + n) / 2)) as f64
                / 2.0
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
    #[case(vec![1,3], vec![2], 2.0)]
    #[case(vec![1,2], vec![3,4], 2.5)]
    fn case(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] expected: f64) {
        let actual = Solution::find_median_sorted_arrays(nums1, nums2);
        assert!(
            (actual - expected).abs() < 1e-5,
            "Assertion failed: actual {:.5} but expected {:.5}. Diff is more than 1e-5.",
            actual,
            expected
        );
    }
}
