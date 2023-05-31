impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let at = nums.partition_point(|x| x >= &nums[0]);
        let (left, right) = nums.split_at(at);
        if Some(&target) > right.last() {
            left.binary_search(&target)
        } else {
            right.binary_search(&target).map(|x| x + at)
        }
        .map_or_else(|_| -1, |x| x as _)
    }
}

pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![4,5,6,7,0,1,2],0,4)]
    #[case(vec![4,5,6,7,0,1,2],3,-1)]
    #[case(vec![1],0,-1)]
    #[case(vec![0,1,2,4,5,6,7],2,2)]
    #[case(vec![1,2,4,5,6,7,0],2,1)]
    #[case(vec![1,2,4,5,6,7,0],0,6)]
    #[case(vec![4,5,6,7,0,1,2],2,6)]
    #[case(vec![4,5,6,7,0,1,2],1,5)]
    #[case(vec![4,5,6,7,0,1,2],7,3)]
    #[case(vec![3,1], 3,0)]
    #[case(vec![7,8,1,2,3,4,5,6], 2,3)]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::search(nums, target);
        assert_eq!(actual, expected);
    }
}
