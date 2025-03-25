//! Solution for https://leetcode.com/problems/trapping-rain-water
//! 42. Trapping Rain Water

impl Solution {
    pub fn trap(heights: Vec<i32>) -> i32 {
        let mut blocks = vec![];
        let mut water_count = 0;
        let mut prev_min_height = heights[0];
        for (d1, h1) in heights.into_iter().enumerate() {
            while let Some((d2, h2)) = blocks.pop() {
                if h1 >= h2 {
                    water_count += (h2 - prev_min_height) * ((d1 - d2) as i32);
                    prev_min_height = h2;
                } else if h1 >= prev_min_height {
                    water_count += (h1 - prev_min_height) * ((d1 - d2) as i32);
                    blocks.push((d2, h2));
                    prev_min_height = h1;
                    break;
                } else {
                    blocks.push((d2, h2));
                    break;
                }
            }
            blocks.push((d1 + 1, h1));
        }
        water_count
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![0,1,0,2], 1)]
    #[case(vec![0,1,0,2,1,0,1], 2)]
    #[case(vec![0,1,0,2,1,0,1,3,2,1,2,1], 6)]
    #[case(vec![4,2,0,3,2,5], 9)]
    #[case(vec![4,2,3], 1)]
    fn case(#[case] height: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::trap(height);
        assert_eq!(actual, expected);
    }
}
