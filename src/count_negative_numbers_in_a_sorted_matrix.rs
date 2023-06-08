//! Solution for https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix
impl Solution {
    pub fn count_negatives(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        while let Some(mut row) = grid.pop() {
            while let Some(cell) = row.pop() {
                if cell < 0 {
                    count += 1;
                }
            }
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
    #[case::leet1(graph_builder![[4,3,2,-1],[3,2,1,-1],[1,1,-1,-2],[-1,-1,-2,-3]],8)]
    #[case::leet2(graph_builder![[3,2],[1,0]],0)]
    fn test(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        assert_eq!(Solution::count_negatives(grid), expected);
    }
}
