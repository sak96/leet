//! Solution for https://leetcode.com/problems/jump-game-ix
//! 3660. Jump Game IX

impl Solution {
    pub fn get_jump(i: usize, jump_idx: &[usize], nums: &[i32], answer: &mut [i32]) -> i32 {
        if jump_idx[i] == i {
            answer[i] = nums[i];
            nums[i]
        } else if answer[i] != 0 {
            answer[i]
        } else {
            answer[i] = Self::get_jump(jump_idx[i], jump_idx, nums, answer);
            answer[i]
        }
    }
    pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut jump_idx = vec![0; n];
        let mut max_ids = vec![];
        for i in 0..n {
            let val = nums[i];
            let mut max_id = i;
            let mut split_at = 0;
            for (idx, id) in max_ids.iter().enumerate().rev() {
                let val2 = nums[*id];
                if val2 <= val {
                    split_at = idx + 1;
                    break;
                } else {
                    if max_id == i {
                        max_id = *max_ids.last().unwrap();
                    }
                    jump_idx[*id] = max_id;
                }
            }
            if split_at < max_ids.len() {
                max_ids.truncate(split_at);
            }
            jump_idx[i] = max_id;
            max_ids.push(max_id);
        }
        let mut answer = vec![0; n];
        for i in 0..n {
            Self::get_jump(i, &jump_idx, &nums, &mut answer);
        }
        answer
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,1,3], vec![2,2,3])]
    #[case(vec![2,3,1], vec![3,3,3])]
    #[case(vec![8,12], vec![8,12])]
    #[case(vec![11,18,11], vec![11, 18,18])]
    #[case(vec![2,1,1,1,1,3], vec![2,2,2,2,2,3])]
    #[case(vec![2,3,3,3,3,1], vec![3,3,3,3,3,3])]
    #[case(vec![2,2,1,3], vec![2,2,2,3])]
    #[case(vec![2,2,3,1,1], vec![3,3,3,3,3])]
    #[case(vec![30,21,5,35,24], vec![35,35,35,35,35])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::max_value(nums);
        assert_eq!(actual, expected);
    }
}
