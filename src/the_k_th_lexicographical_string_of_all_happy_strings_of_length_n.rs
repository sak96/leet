//! Solution for https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n
//! 1415. The k-th Lexicographical String of All Happy Strings of Length n

impl Solution {
    const CHARS: [char; 3] = ['a', 'b', 'c'];
    pub fn get_happy_string(n: i32, mut k: i32) -> String {
        let mut output = String::new();
        let mut index = 2_i32.pow((n - 1) as u32);

        k -= 1;
        let mut prev_char = 'z';
        for _ in 0..n {
            let mut char_index = k / index;
            if char_index >= 3 {
                break;
            }
            let mut cur_char = Self::CHARS[char_index as usize];
            if cur_char >= prev_char {
                char_index = (char_index + 1) % 3;
                cur_char = Self::CHARS[char_index as usize];
            }
            output.push(cur_char);
            prev_char = cur_char;
            k %= index;
            index /= 2;
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
    #[case(1, 3, "c")]
    #[case(1, 4, "")]
    #[case(3, 1, "aba")]
    #[case(3, 2, "abc")]
    #[case(3, 3, "aca")]
    #[case(3, 4, "acb")]
    #[case(3, 5, "bab")]
    #[case(3, 6, "bac")]
    #[case(3, 7, "bca")]
    #[case(3, 8, "bcb")]
    #[case(3, 9, "cab")]
    #[case(3, 10, "cac")]
    #[case(3, 11, "cba")]
    #[case(3, 12, "cbc")]
    #[case(10, 100, "abacbabacb")]
    fn case(#[case] n: i32, #[case] k: i32, #[case] expected: String) {
        let actual = Solution::get_happy_string(n, k);
        assert_eq!(actual, expected);
    }
}
