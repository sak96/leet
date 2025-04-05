//! Solution for https://leetcode.com/problems/counting-bits
//! 338. Counting Bits

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        match n {
            0 => vec![0],
            1 => vec![0, 1],
            _ => {
                let mut output = vec![0, 1];
                let mut last_2_power = 2;
                let mut cur_index = 0;
                let mut i = 2;
                while i < n + 1 {
                    if cur_index < last_2_power {
                        output.push(1 + output[cur_index]);
                        cur_index += 1;
                        i += 1;
                    } else {
                        last_2_power = output.len();
                        cur_index = 0;
                    }
                }
                output
            }
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, vec![0,1,1])]
    #[case(5, vec![0,1,1,2,1,2])]
    fn case(#[case] n: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::count_bits(n);
        assert_eq!(actual, expected);
    }
}
