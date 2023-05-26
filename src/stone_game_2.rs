use std::collections::HashMap;
impl Solution {
    pub fn stone_game_ii_(
        piles: &[i32],
        m: usize,
        is_alice: bool,
        memory: &mut HashMap<(usize, usize, bool), i32>,
    ) -> i32 {
        let key = (piles.len(), m, is_alice);
        let it = 1..=((2 * m).min(piles.len()));
        let value = if piles.is_empty() {
            0
        } else if memory.contains_key(&key) {
            return *memory.get(&key).expect("already checked");
        } else if is_alice {
            it.map(|i| {
                piles.iter().take(i).copied().sum::<i32>()
                    + Self::stone_game_ii_(&piles[i..], i.max(m), !is_alice, memory)
            })
            .max()
            .unwrap()
        } else {
            it.map(|i| Self::stone_game_ii_(&piles[i..], i.max(m), !is_alice, memory))
                .min()
                .unwrap()
        };
        memory.insert(key, value);
        value
    }

    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        Self::stone_game_ii_(&piles, 1, true, &mut HashMap::new())
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::simple_1(vec![4], 4)]
    #[case::simple_1(vec![1, 3], 4)]
    #[case::simple_1(vec![1, 3, 3], 4)]
    #[case::leet1(vec![2,7,9,4,4], 10)]
    #[case::leet2(vec! [1,2,3,4,5,100], 104)]
    fn test(#[case] piles: Vec<i32>, #[case] output: i32) {
        assert_eq!(Solution::stone_game_ii(piles), output)
    }
}
