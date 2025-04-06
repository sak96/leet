//! Solution for https://leetcode.com/problems/largest-divisible-subset
//! 368. Largest Divisible Subset

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut dp_len = vec![1; nums.len()];
        let mut dp_prev = vec![];
        for i in 0..nums.len() {
            let mut index = None;
            for j in 0..i {
                if nums[i] % nums[j] == 0 && dp_len[i] <= dp_len[j] {
                    dp_len[i] = dp_len[j] + 1;
                    index = Some(j);
                }
            }
            dp_prev.push(index);
        }
        let mut index = dp_len
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(i, _)| i);
        let mut output = vec![];
        while let Some(i) = index {
            output.push(nums[i]);
            index = dp_prev[i];
        }
        output.reverse();
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
    #[case(vec![1,2,3], vec![1, 2])]
    #[case(vec![1,2,4,8], vec![1,2,4,8])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let mut actual = Solution::largest_divisible_subset(nums);
        actual.sort_unstable();
        assert_eq!(
            actual.len(),
            expected.len(),
            "{:?} has more len than {:?}",
            &expected,
            &actual
        );
        for z in actual.windows(2) {
            assert_eq!(
                z[1] % z[0],
                0,
                "{:?} has non divisible numbers {} {}",
                &actual,
                z[1],
                z[0]
            );
        }
    }
}
