//! Solution for https://leetcode.com/problems/summary-ranges
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut iter = nums.into_iter();
        let mut output = vec![];
        if let Some(value) = iter.next() {
            let (mut a, mut b) = (value, value);
            for item in iter {
                if item != b + 1 {
                    if a == b {
                        output.push(format!("{a}"));
                    } else {
                        output.push(format!("{a}->{b}"));
                    }
                    a = item;
                }
                b = item;
            }
            if a == b {
                output.push(format!("{a}"));
            } else {
                output.push(format!("{a}->{b}"));
            }
        }
        output
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::leet1(vec![0,1,2,4,5,7], vec!["0->2","4->5","7"])]
    #[case::leet2(vec![0,2,3,4,6,8,9], vec!["0","2->4","6","8->9"])]
    fn test(#[case] nums: Vec<i32>, #[case] output: Vec<&str>) {
        assert_eq!(Solution::summary_ranges(nums), output);
    }
}
