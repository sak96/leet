//! Solution for https://leetcode.com/problems/best-time-to-buy-and-sell-stock
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut sell = prices[0];
        let mut cost = prices[0];
        for price in prices {
            if sell < price {
                sell = price;
            }
            if cost > price {
                profit = profit.max(sell - cost);
                sell = price;
                cost = price;
            }
        }
        profit.max(sell - cost)
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::leet1([7,1,5,3,6,4],5)]
    #[case::leet2([7,6,4,3,1],0)]
    fn test(#[case] prices: impl AsRef<[i32]>, #[case] output: i32) {
        assert_eq!(Solution::max_profit(prices.as_ref().to_vec()), output);
    }
}
