use std::collections::HashMap;
impl Solution {
    pub fn stone_game_ii_(
        piles: &[i32],
        m: usize,
        memory: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        let sum: i32 = piles.iter().clone().sum();
        let key = (piles.len(), m);
        let it = 1..=2 * m;
        let value = if piles.len() <= 2 * m {
            // take all if you can
            sum
        } else if memory.contains_key(&key) {
            return *memory.get(&key).expect("already checked");
        } else {
            // pick move that maximizes final score
            it.map(|i| {
                // final score = current piles sum - opponents best move
                sum - Self::stone_game_ii_(&piles[i..], i.max(m), memory)
            })
            .max()
            .unwrap()
        };
        memory.insert(key, value);
        value
    }

    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        Self::stone_game_ii_(&piles, 1, &mut HashMap::new())
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::simple_1(vec![4], 4)]
    #[case::simple_2(vec![1, 3], 4)]
    #[case::simple_3(vec![1, 3, 3], 4)]
    #[case::leet1(vec![2,7,9,4,4], 10)]
    #[case::leet2(vec! [1,2,3,4,5,100], 104)]
    fn test(#[case] piles: Vec<i32>, #[case] output: i32) {
        assert_eq!(Solution::stone_game_ii(piles), output)
    }
}
