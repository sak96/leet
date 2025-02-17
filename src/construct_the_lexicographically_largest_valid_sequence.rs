//! Solution for https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence
//! 1718. Construct the Lexicographically Largest Valid Sequence

impl Solution {
    pub fn generate_sequence(output: &mut [i32], seen: &mut [bool], mut index: usize) -> bool {
        let len = output.len();
        while output[index] != 0 {
            index += 1;
            if index == len {
                dbg!(index);
                return true;
            }
        }

        for i in (1..seen.len()).rev() {
            if seen[i] {
                continue;
            }
            seen[i] = true;
            output[index] = i as i32;
            if i == 1 {
                if Self::generate_sequence(output, seen, index) {
                    return true;
                }
            } else if index + i < len && output[index + i] == 0 {
                output[index + i] = i as i32;
                if Self::generate_sequence(output, seen, index) {
                    return true;
                }
                output[index + i] = 0;
            }
            output[index] = 0;
            seen[i] = false;
        }
        false
    }

    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let mut output = vec![0; 2 * n as usize - 1];
        let mut seen = vec![false; n as usize + 1];
        Self::generate_sequence(&mut output, &mut seen, 0);
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
    #[case(3, vec![3,1,2,3,2])]
    #[case(5, vec![5,3,1,4,3,5,2,4,2])]
    fn case(#[case] n: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::construct_distanced_sequence(n);
        assert_eq!(actual, expected);
    }
}
