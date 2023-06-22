//! Solution for https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut free = 0;
        let mut paid = prices[0];
        for price in prices {
            let paid_prev = paid;
            paid = paid.min(price - free);
            free = free.max(price - paid_prev - fee);
        }
        free
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::leet1([1,3,2,8,4,9],2,8)]
    #[case::leet1([1,3,7,5,10,3],3,6)]
    fn test(#[case] prices: impl AsRef<[i32]>, #[case] fee: i32, #[case] output: i32) {
        assert_eq!(Solution::max_profit(prices.as_ref().into(), fee), output);
    }
}
