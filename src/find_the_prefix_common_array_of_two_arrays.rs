//! Solution for https://leetcode.com/problems/find-the-prefix-common-array-of-two-arrays
//! 2657. Find the Prefix Common Array of Two Arrays

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut output = Vec::with_capacity(a.len());
        let mut seen = vec![false; a.len() + 1];
        let mut count = 0;
        for (a, b) in a.into_iter().zip(b) {
            if seen[a as usize] {
                count += 1
            } else {
                seen[a as usize] = true;
            }
            if seen[b as usize] {
                count += 1
            } else {
                seen[b as usize] = true;
            }
            output.push(count)
        }
        output
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,2,4], vec![3,1,2,4], vec![0,2,3,4])]
    #[case(vec![2,3,1], vec![3,1,2], vec![0,1,3])]
    fn case(#[case] a: Vec<i32>, #[case] b: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::find_the_prefix_common_array(a, b);
        assert_eq!(actual, expected);
    }
}
