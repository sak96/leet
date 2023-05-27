impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut nums = nums.as_slice();
        let mut nearest = i32::MAX;
        while nums.len() >= 3 {
            let (num1, rest) = nums.split_first().unwrap();
            nums = rest;
            let first_diff = target - num1;
            let mut l = 0;
            let mut h = nums.len() - 1;
            while h > l {
                match (nums[h] + nums[l]) - first_diff {
                    0 => return target,
                    second_diff => {
                        if second_diff.abs() < (target - nearest).abs() {
                            nearest = nums[h] + nums[l] + num1;
                        }
                        if second_diff > 0 {
                            h -= 1;
                        } else {
                            l += 1;
                        }
                    }
                }
            }
        }
        nearest
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(vec![-1,2,1,-4], 1, 2)]
    #[case::leet1(vec![0,0,0], 1, 0)]
    fn test(#[case] nums: Vec<i32>, #[case] target: i32, #[case] output: i32) {
        assert_eq!(Solution::three_sum_closest(nums, target), output);
    }
}
