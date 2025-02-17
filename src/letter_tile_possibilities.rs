//! Solution for https://leetcode.com/problems/letter-tile-possibilities
//! 1079. Letter Tile Possibilities

impl Solution {
    const LEN: usize = (b'Z' - b'A' + 1) as usize;
    fn possibilities(mem: &mut [usize]) -> i32 {
        let mut total = 0;
        for i in 0..mem.len() {
            if mem[i] == 0 {
                continue;
            }
            total += 1;
            mem[i] -= 1;
            total += Self::possibilities(mem);
            mem[i] += 1;
        }
        total
    }

    pub fn num_tile_possibilities(mut tiles: String) -> i32 {
        let mut mem = vec![0; Self::LEN];
        for &mut ch in unsafe { tiles.as_bytes_mut() } {
            mem[(ch - b'A') as usize] += 1;
        }
        Self::possibilities(&mut mem)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("AAB", 8)]
    #[case("AAABBC", 188)]
    #[case("V", 1)]
    fn case(#[case] tiles: String, #[case] expected: i32) {
        let actual = Solution::num_tile_possibilities(tiles);
        assert_eq!(actual, expected);
    }
}
