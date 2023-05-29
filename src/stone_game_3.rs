impl Solution {
    pub fn stone_game_iii(piles: Vec<i32>) -> String {
        let mut sum = 0;
        let mut last_3_tries = std::collections::VecDeque::from(vec![0; 4]);
        for stone in piles.into_iter().rev() {
            // drop 4th try
            last_3_tries.pop_back();
            sum += stone;
            // maximize your score (sum - opponent_score)
            last_3_tries.push_front(
                last_3_tries
                    .iter()
                    .map(|opponent_score| sum - opponent_score)
                    .max()
                    .unwrap(),
            );
        }
        // if Alice has more than half of total she wins
        match (last_3_tries.pop_front().unwrap() * 2).cmp(&sum) {
            std::cmp::Ordering::Less => "Bob",
            std::cmp::Ordering::Equal => "Tie",
            std::cmp::Ordering::Greater => "Alice",
        }
        .into()
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(vec![1,2,3,7], "Bob")]
    #[case::leet2(vec![1,2,3,-9], "Alice")]
    #[case::leet3(vec![9,-12,8,12,-12,-16,-13,-11,2,2,-10,-5,13,12,-4,13,4,-4,-16,6,-2,13,-8], "Alice")]
    #[case::simple_1(vec![4], "Alice")]
    #[case::simple_2(vec![1, 3], "Alice")]
    #[case::simple_2(vec![1,2, 3], "Alice")]
    #[case::neg_1(vec![-9], "Bob")]
    #[case::neg_2(vec![-10, -9], "Bob")]
    #[case::neg_3(vec![-9, -10], "Alice")]
    #[case::neg_3(vec![-2, 13, -8], "Alice")]
    fn test(#[case] piles: Vec<i32>, #[case] output: &str) {
        assert_eq!(Solution::stone_game_iii(piles), output)
    }
}
