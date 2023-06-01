impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // imagine target - 0.5 -> for equal case it gives greater than
        let first = nums
            .binary_search_by(|probe| match probe.cmp(&target) {
                std::cmp::Ordering::Equal => std::cmp::Ordering::Greater,
                x => x,
            })
            .unwrap_err() as i32;
        // imagine target + 0.5 -> for equal case it gives less than
        let second = nums
            .binary_search_by(|probe| match probe.cmp(&target) {
                std::cmp::Ordering::Equal => std::cmp::Ordering::Less,
                x => x,
            })
            .unwrap_err() as i32;
        if first < second {
            vec![first, second - 1]
        } else {
            vec![-1, -1]
        }
    }
}
pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(vec![5,7,7,8,8,10], 8, [3, 4])]
    #[case::leet1(vec![5,7,7,8,8,10], 6, [-1,-1])]
    #[case::leet1(vec![], 0, [-1,-1])]
    fn test(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: [i32; 2]) {
        dbg!(&nums, target);
        assert_eq!(Solution::search_range(nums, target), expected);
    }
}
