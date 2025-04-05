//! Solution for https://leetcode.com/problems/sliding-window-median
//! 480. Sliding Window Median

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let (a, b) = if k % 2 == 0 {
            (k / 2, (k / 2) - 1)
        } else {
            (k / 2, k / 2)
        };
        let mut window: Vec<_> = nums.iter().take(k).cloned().collect();
        window.sort_unstable();
        let mut output = vec![(window[a] as f64 + window[b] as f64) / 2.0];
        for (i, n) in nums.iter().enumerate().take(nums.len() - k) {
            window.remove(window.binary_search(n).unwrap());
            let i_ = window.binary_search(&nums[i + k]).unwrap_or_else(|x| x);
            window.insert(i_, nums[i + k]);
            output.push((window[a] as f64 + window[b] as f64) / 2.0);
        }
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
    #[case(vec![1], 1, vec![1.0])]
    #[case(vec![1, 2], 1, vec![1.0, 2.0])]
    #[case(vec![1, 2], 2, vec![1.5])]
    #[case(vec![1,3,-1,-3,5,3,6,7], 3, vec![1.00000,-1.00000,-1.00000,3.00000,5.00000,6.00000])]
    #[case(vec![1,2,3,4,2,3,1,4,2], 3, vec![2.00000,3.00000,3.00000,3.00000,2.00000,3.00000,2.00000])]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: Vec<f64>) {
        let actual = Solution::median_sliding_window(nums, k);
        assert_eq!(actual, expected);
    }
}
