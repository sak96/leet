impl Solution {
    pub fn stone_game_iii(piles: Vec<i32>) -> String {
        let mut sum = 0;
        let mut memory = std::collections::VecDeque::with_capacity(4);
        memory.push_front(0);
        memory.push_front(0);
        memory.push_front(0);
        for stone in piles.into_iter().rev() {
            sum += stone;
            memory.push_front(memory.iter().map(|i| sum - i).max().unwrap());
            memory.pop_back();
        }
        match (2 * memory.pop_front().unwrap_or(0)).cmp(&sum) {
            std::cmp::Ordering::Less => "Bob",
            std::cmp::Ordering::Equal => "Tie",
            std::cmp::Ordering::Greater => "Alice",
        }
        .to_string()
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
