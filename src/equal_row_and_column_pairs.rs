//! Solution for https://leetcode.com/problems/equal-row-and-column-pairs

use std::collections::BTreeMap;

#[derive(Default)]
struct Trie(BTreeMap<i32, Trie>);
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut trie = Trie::default();
        for row in &grid {
            let mut set = &mut trie;
            for &c in row {
                let entry = set.0.entry(c).or_default();
                set = entry;
            }
            if let Some(&key) = set.0.keys().next() {
                let value = set.0.remove(&key).unwrap();
                set.0.insert(key + 1, value);
            } else {
                set.0.insert(1, Trie::default());
            }
        }

        let mut count = 0;
        let trie = trie;
        let n = grid.len();
        'outer: for col in 0..n {
            let mut set = &trie;
            for row in &grid {
                if let Some(value) = set.0.get(&row[col]) {
                    set = value
                } else {
                    continue 'outer;
                }
            }
            count += set.0.keys().next().unwrap();
        }

        count
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_builder;

    use rstest::rstest;

    #[rstest]
    #[case::leet1(graph_builder![[3,2,1],[1,7,6],[2,7,7]],1)]
    #[case::leet2(graph_builder![[3,1,2,2],[1,4,4,5],[2,4,2,2],[2,4,2,2]],3)]
    fn test(#[case] grid: Vec<Vec<i32>>, #[case] output: i32) {
        assert_eq!(Solution::equal_pairs(grid), output);
    }
}
