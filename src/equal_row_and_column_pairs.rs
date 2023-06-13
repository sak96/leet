//! Solution for https://leetcode.com/problems/equal-row-and-column-pairs

use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new();
        let mut vec = Vec::with_capacity(grid.len());
        for col in 0..grid.len() {
            for row in &grid {
                vec.push(row[col]);
            }
            *map.entry(vec.clone()).or_default() += 1;
            vec.clear();
        }
        let mut result = 0;
        for row in grid {
            for cell in row {
                vec.push(cell);
            }
            result += map.get(&vec).unwrap_or(&0);
            vec.clear();
        }
        result
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
