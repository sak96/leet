impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let at = nums.partition_point(|x| x >= &nums[0]);
        let (left, right) = nums.split_at(at);
        *right.first().or_else(|| left.first()).unwrap()
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![3,4,5,1,2],1)]
    #[case(vec![4,5,6,7,0,1,2],0)]
    #[case(vec![11,13,15,17],11)]
    fn test(#[case] nums: Vec<i32>, #[case] output: i32) {
        assert_eq!(Solution::find_min(nums), output)
    }
}
