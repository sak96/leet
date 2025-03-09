//! Solution for https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks
//! 2379. Minimum Recolors to Get K Consecutive Black Blocks

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let mut it = blocks.as_bytes().windows(k as usize);
        let win = it.next().unwrap();
        let mut min = win.iter().filter(|a| b'W'.eq(a)).count() as i32;
        let mut cur = min;
        let mut first_char = win[0];
        for win in it {
            if first_char == b'W' {
                cur -= 1;
            }

            if win.last() == Some(&b'W') {
                cur += 1;
            }
            first_char = win[0];
            min = min.min(cur);
        }
        min
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("WBBWWBBWBW", 7, 3)]
    #[case("WBWBBBW", 2, 0)]
    fn case(#[case] blocks: String, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::minimum_recolors(blocks, k);
        assert_eq!(actual, expected);
    }
}
