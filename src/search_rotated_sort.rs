impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums.as_slice();
        let mut pos = 0;
        while !nums.is_empty() {
            let mid_pos = nums.len() / 2;
            let mid = nums[mid_pos];
            // Note: it will either be or right or left or equal
            let is_on_left = match target.cmp(&mid) {
                std::cmp::Ordering::Equal => return (pos + mid_pos) as i32,
                std::cmp::Ordering::Less => {
                    // target < first and sorted on left => go to right (not left) else left
                    !(Some(&target) < nums.first() && nums.first() <= Some(&mid))
                }
                std::cmp::Ordering::Greater => {
                    // target > first and sorted on right => go to left else right
                    Some(&target) > nums.last() && nums.last() >= Some(&mid)
                }
            };
            if is_on_left {
                nums = &nums[..mid_pos];
            } else {
                pos += mid_pos + 1;
                nums = &nums[mid_pos + 1..];
            }
        }
        -1
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
