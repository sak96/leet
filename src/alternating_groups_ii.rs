//! Solution for https://leetcode.com/problems/alternating-groups-ii
//! 3208. Alternating Groups II

impl Solution {
    pub fn number_of_alternating_groups(mut colors: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut count = 0;
        colors.extend(colors[..(k - 1)].to_owned());
        let mut it = colors.windows(k);
        let mut value = None;
        while let Some(win) = it.next() {
            let skip = match value {
                v if v == win.last() => {
                    value = None;
                    k - 2
                }
                Some(_) => {
                    count += 1;
                    value = win.last();
                    0
                }
                None => {
                    let s = win.windows(2).rev().take_while(|z| z[0] != z[1]).count();
                    if s + 1 == k {
                        value = win.last();
                        count += 1;
                        0
                    } else {
                        k - s - 2
                    }
                }
            };
            for _ in 0..skip {
                it.next();
            }
        }
        count
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![0,1,0,1,0], 3, 3)]
    #[case(vec![0,1,0,0,1,0,1], 6, 2)]
    #[case(vec![1,1,0,1], 4, 0)]
    #[case(vec![1,1,0], 3, 1)]
    fn case(#[case] colors: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::number_of_alternating_groups(colors, k);
        assert_eq!(actual, expected);
    }
}
