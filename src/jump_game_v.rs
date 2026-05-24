//! Solution for https://leetcode.com/problems/jump-game-v
//! 1340. Jump Game V

impl Solution {
    pub fn max_jumps_(arr: &[i32], i: usize, dp: &mut [i32], d: usize) -> i32 {
        let mut max = dp[i];
        if max > 0 {
            return max;
        }
        max = 1;
        let value = arr[i];

        for i in (i.saturating_sub(d)..i).rev() {
            if arr[i] < value {
                max = max.max(Self::max_jumps_(arr, i, dp, d) + 1);
            } else {
                break;
            }
        }

        for i in (i + 1)..((i + d + 1).min(arr.len())) {
            if arr[i] < value {
                max = max.max(Self::max_jumps_(arr, i, dp, d) + 1);
            } else {
                break;
            }
        }
        dp[i] = max;
        max
    }

    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;
        let mut dp = vec![-1; arr.len()];
        (0..arr.len())
            .map(|i| Self::max_jumps_(&arr, i, &mut dp, d))
            .max()
            .unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![6,4,14,6,8,13,9,7,10,6,12], 2, 4)]
    #[case(vec![3,3,3,3,3], 3, 1)]
    #[case(vec![7,6,5,4,3,2,1], 1, 7)]
    fn case(#[case] arr: Vec<i32>, #[case] d: i32, #[case] expected: i32) {
        let actual = Solution::max_jumps(arr, d);
        assert_eq!(actual, expected);
    }
}
