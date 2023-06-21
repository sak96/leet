//! Solution for https://leetcode.com/problems/minimum-cost-to-make-array-equal
impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let mut min = *nums.iter().min().expect("1 <= nums.length");
        let mut max = *nums.iter().max().expect("1 <= nums.length");
        let cost_fn = |mid: i32| {
            nums.iter()
                .zip(cost.iter())
                .map(|(i, &c)| (i - mid).abs() as i64 * c as i64)
                .sum::<i64>()
        };
        let mut result = 0;

        while min < max {
            let mid = (min + max) / 2;
            let cost = cost_fn(mid);
            let cost_plus_1 = cost_fn(mid + 1);
            if cost > cost_plus_1 {
                min = mid + 1;
                result = cost_plus_1;
            } else {
                max = mid;
                result = cost;
            }
        }

        result
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::leet1([1,3,5,2],[2,3,1,14],8)]
    #[case::leet2([2,2,2,2,2],[4,2,8,1,3],0)]
    fn test(
        #[case] nums: impl AsRef<[i32]>,
        #[case] cost: impl AsRef<[i32]>,
        #[case] min_cost: i64,
    ) {
        assert_eq!(
            Solution::min_cost(nums.as_ref().into(), cost.as_ref().into()),
            min_cost
        );
    }
}
