//! Solution for https://leetcode.com/problems/4sum
//! 18. 4Sum

impl Solution {
    pub fn two_sum(result: &mut Vec<Vec<i32>>, mut nums: &[i32], x: i32, y: i32, target: i32) {
        if let Some((&a, rest)) = nums.split_first() {
            if let Some((&b, _)) = rest.split_last() {
                let (mut low, mut high) = (true, true);
                match (target
                    .saturating_sub(x)
                    .saturating_sub(y)
                    .saturating_sub(a)
                    .saturating_sub(b))
                .signum()
                {
                    1 => high = false,
                    0 => result.push(vec![x, y, a, b]),
                    -1 => low = false,
                    _ => unreachable!(),
                }
                if low {
                    while nums.first() == Some(&a) {
                        nums = &nums[1..];
                    }
                }
                if high {
                    while nums.last() == Some(&b) {
                        nums = nums.split_last().unwrap().1;
                    }
                }
                Self::two_sum(result, nums, x, y, target);
            }
        }
    }
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut output = vec![];
        let mut nums = nums.as_slice();
        while let Some((&first, mut first_rest)) = nums.split_first() {
            while let Some((&second, second_rest)) = first_rest.split_first() {
                Self::two_sum(&mut output, &second_rest, first, second, target);
                let pos = first_rest.partition_point(|&n| n == second);
                first_rest = &first_rest[pos..];
            }
            let pos = nums.partition_point(|&n| n == first);
            nums = &nums[pos..];
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
    #[case(vec![1,0,-1,0,-2,2], 0, vec![vec![-2,-1,1,2],vec![-2,0,0,2],vec![-1,0,0,1]])]
    #[case(vec![2,2,2,2,2], 8, vec![vec![2,2,2,2]])]
    #[case(vec![-2,-1,-1,1,1,2,2], 0, vec![vec![-2,-1,1,2],vec![-1,-1,1,1]])]
    #[case(vec![1000000000,1000000000,1000000000,1000000000], -294967296, vec![])]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::four_sum(nums, target);
        assert_eq!(actual, expected);
    }
}
