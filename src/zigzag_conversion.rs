//! Solution for https://leetcode.com/problems/zigzag-conversion
//! 6. Zigzag Conversion

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let num_rows = num_rows as usize;
        let mut output = vec![String::new(); num_rows];
        let mut i = (0..num_rows).chain((1..num_rows - 1).rev()).cycle();
        for a in s.chars() {
            output[i.next().unwrap()].push(a);
        }
        output.join("")
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR")]
    #[case("PAYPALISHIRING", 4, "PINALSIGYAHRPI")]
    #[case("A", 1, "A")]
    #[case("ABC", 1, "ABC")]
    fn case(#[case] s: String, #[case] num_rows: i32, #[case] expected: String) {
        let actual = Solution::convert(s, num_rows);
        assert_eq!(actual, expected);
    }
}
