//! Solution for https://leetcode.com/problems/zero-array-transformation-ii
//! 3356. Zero Array Transformation II

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut queries = queries.into_iter();
        let mut delta = vec![0; nums.len() + 1];
        let mut output = 0;
        let mut cur_diff = 0;
        for (i, &n) in nums.iter().enumerate() {
            // current diff not empty
            while cur_diff + delta[i] < n {
                output += 1;
                let Some(diff) = queries.next() else {
                    return -1;
                };
                let left = diff[0] as usize;
                let right = diff[1] as usize;
                let d = diff[2];
                if right >= i {
                    delta[left.max(i)] += d;
                    delta[right + 1] -= d;
                }
            }
            cur_diff += delta[i];
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
    #[case(vec![2,0,2], vec![vec![0,2,1],vec![0,2,1],vec![1,1,3]], 2)]
    #[case(vec![4,3,2,1], vec![vec![1,3,2],vec![0,2,1]], -1)]
    fn case(#[case] nums: Vec<i32>, #[case] queries: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::min_zero_array(nums, queries);
        assert_eq!(actual, expected);
    }
}
