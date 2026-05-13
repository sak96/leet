//! Solution for https://leetcode.com/problems/minimum-moves-to-make-array-complementary
//! 1674. Minimum Moves to Make Array Complementary

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let limit = limit as usize;
        let mut toggle_diff = vec![0; 2 * limit + 2];
        let mut nums = std::collections::VecDeque::from(nums);
        while let Some(a) = nums.pop_front() {
            let b = nums.pop_back().expect("n is even");
            let min = a.min(b) as usize;
            let max = a.max(b) as usize;
            // (1, 1) + .. (min - 1, 1) (both min and max are subtracted)
            toggle_diff[2] += 2;
            // (min, 1) .. (min, max -1) (only max is decremented)
            toggle_diff[min + 1] -= 1;
            // (min, max)  (no need of any changes)
            toggle_diff[min + max] -= 1;
            // (min + 1, max) .. (limit, max) (only min is added)
            toggle_diff[min + max + 1] += 1;
            // (limit, max) .. (both max is added and min need to be added)
            toggle_diff[max + limit + 1] += 1;
        }
        let mut toggle_min = i32::MAX;
        let mut toggles = toggle_diff.drain(..2).sum();
        for diff in toggle_diff {
            toggles += diff;
            toggle_min = toggle_min.min(toggles);
        }
        toggle_min
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,4,3], 4, 1)]
    #[case(vec![1,2,2,1], 2, 2)]
    #[case(vec![1,2,1,2], 2, 0)]
    #[case(vec![1,4,4,3], 4, 2)]
    fn case(#[case] nums: Vec<i32>, #[case] limit: i32, #[case] expected: i32) {
        let actual = Solution::min_moves(nums, limit);
        assert_eq!(actual, expected);
    }
}
