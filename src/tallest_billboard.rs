//! Solution for https://leetcode.com/problems/tallest-billboard
//! 956. Tallest Billboard

impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let sum = rods.iter().sum::<i32>() as usize;
        let mut result = vec![-1i32; sum + 1];
        result[0] = 0;
        let mut temp = result.clone();
        for r in rods {
            let r = r as usize;
            temp.copy_from_slice(&result);
            for (diff, &tall) in temp.iter().enumerate().filter(|&(_, v)| v >= &0) {
                // add to taller
                result[diff + r] = result[diff + r].max(tall + r as i32);
                // add to smaller
                if diff >= r {
                    result[diff - r] = result[diff - r].max(tall);
                } else {
                    result[r - diff] = result[r - diff].max((r - diff) as i32 + tall);
                }
            }
        }
        result[0]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,6], 6)]
    // #[case(vec![1,2,3,4,5,6], 10)]
    // #[case(vec![1,2],0)]
    // #[case(vec![3,4,3,3,2],6)]
    fn case(#[case] rods: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::tallest_billboard(rods);
        assert_eq!(actual, expected);
    }
}
