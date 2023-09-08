//! Solution for https://leetcode.com/problems/coin-change-ii
//! 518. Coin Change II

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut memory = vec![0; amount as usize + 1];
        memory[0] = 1;
        for coin in coins {
            for i in coin..=amount {
                memory[i as usize] += memory[(i - coin) as usize];
            }
        }
        memory[amount as usize]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, vec![1,2,5], 4)]
    #[case(3, vec![2], 0)]
    #[case(10, vec![10], 1)]
    fn case(#[case] amount: i32, #[case] coins: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::change(amount, coins);
        assert_eq!(actual, expected);
    }
}
