//! Solution for https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let mut index = 0;
        for row in grid.into_iter().rev() {
            index = index + row[index..].partition_point(|&x| x >= 0);
            if row.len() - index > 0 {
                count += row.len() - index;
            } else {
                break;
            }
        }
        count as _
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(vec![vec![4,3,2,-1],vec![3,2,1,-1],vec![1,1,-1,-2],vec![-1,-1,-2,-3]],8)]
    #[case::leet2(vec![vec![3,2],vec![1,0]],0)]
    fn test(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        assert_eq!(Solution::count_negatives(grid), expected);
    }
}
