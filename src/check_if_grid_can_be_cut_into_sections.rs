//! Solution for https://leetcode.com/problems/check-if-grid-can-be-cut-into-sections
//! 3394. Check if Grid can be Cut into Sections

impl Solution {
    pub fn check_valid_cuts(_n: i32, mut rectangles: Vec<Vec<i32>>) -> bool {
        for (start, end) in [(0, 2), (1, 3)] {
            {
                rectangles.sort_unstable_by_key(|n| n[start]);
                let mut last_rectangle_end = -1;
                let mut gaps = -1;
                let mut rectangles = rectangles.as_slice();
                while let Some((rectangle, rest)) = rectangles.split_first() {
                    rectangles = rest;
                    if last_rectangle_end <= rectangle[start] {
                        gaps += 1;
                        if gaps == 2 {
                            return true;
                        }
                    }
                    last_rectangle_end = last_rectangle_end.max(rectangle[end])
                }
            }
        }
        false
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, vec![vec![1,0,5,2],vec![0,2,2,4],vec![3,2,5,3],vec![0,4,4,5]], true)]
    #[case(4, vec![vec![0,0,1,1],vec![2,0,3,4],vec![0,2,2,3],vec![3,0,4,3]], true)]
    #[case(4, vec![vec![0,2,2,4],vec![1,0,3,2],vec![2,2,3,4],vec![3,0,4,2],vec![3,2,4,4]],false)]
    fn case(#[case] n: i32, #[case] rectangles: Vec<Vec<i32>>, #[case] expected: bool) {
        let actual = Solution::check_valid_cuts(n, rectangles);
        assert_eq!(actual, expected);
    }
}
