impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|i| i != &val);
        nums.len() as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(vec![3,2,2,3],3,2)]
    #[case::leet2(vec![0,1,2,2,3,0,4,2],2,5)]
    fn test(#[case] mut input: Vec<i32>, #[case] val: i32, #[case] expected: i32) {
        let mut nums = input.clone();
        assert_eq!(Solution::remove_element(&mut nums, val), expected);

        let expected = expected as usize;
        assert!(nums.len() >= expected);
        nums[0..expected].as_mut().sort_unstable();
        input.sort_unstable();
        let mut j = 0;
        for x in input.into_iter() {
            if j < expected && x != nums[j] {
                assert_eq!(x, val);
            } else {
                j += 1;
            }
        }
    }
}
