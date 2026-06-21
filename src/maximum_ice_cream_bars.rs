//! Solution for https://leetcode.com/problems/maximum-ice-cream-bars
//! 1833. Maximum Ice Cream Bars

impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, mut coins: i32) -> i32 {
        let max_cost = costs.iter().max().copied().unwrap();
        let mut dp = vec![0; max_cost as usize + 1];
        for &i in &costs {
            dp[i as usize] += 1
        }
        let mut total = 0;
        for (i, c) in dp.iter().enumerate() {
            if coins < (i as i32) * c {
                total += coins / (i as i32);
                break;
            }
            coins -= (i as i32) * c;
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
    #[case(vec![1,3,2,4,1], 7, 4)]
    #[case(vec![10,6,8,7,7,8], 5, 0)]
    #[case(vec![1,6,3,1,2,5], 20, 6)]
    fn case(#[case] costs: Vec<i32>, #[case] coins: i32, #[case] expected: i32) {
        let actual = Solution::max_ice_cream(costs, coins);
        assert_eq!(actual, expected);
    }
}
