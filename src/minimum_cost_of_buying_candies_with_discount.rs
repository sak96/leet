//! Solution for https://leetcode.com/problems/minimum-cost-of-buying-candies-with-discount
//! 2144. Minimum Cost of Buying Candies With Discount

impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable();
        cost.reverse();
        let mut total = 0;
        for (c, &i) in cost.into_iter().zip([2, 1, 0].iter().cycle()) {
            if i == 0 {
                continue;
            }
            total += c;
        }
        total
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3], 5)]
    #[case(vec![6,5,7,9,2,2], 23)]
    #[case(vec![5,5], 10)]
    fn case(#[case] cost: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::minimum_cost(cost);
        assert_eq!(actual, expected);
    }
}
