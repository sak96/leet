//! Solution for https://leetcode.com/problems/shortest-path-to-get-all-keys
//! 864. Shortest Path to Get All Keys

impl Solution {
    const NEIGHBOUR: &'static [[usize; 2]] = &[[0, 1], [0, usize::MAX], [1, 0], [usize::MAX, 0]];
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let grid: Vec<_> = grid.into_iter().map(|s| s.into_bytes()).collect();
        let mut keys = 0u32;
        let mut start = None;
        for (i, r) in grid.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                if c.is_ascii_lowercase() {
                    keys |= 1u32 << (c - b'a') as u32
                }
                if c.eq(&b'@') {
                    start = Some((i, j));
                }
            }
        }
        let mut queued = std::collections::HashSet::new();
        let mut distance = 0;
        let mut queue = std::collections::VecDeque::new();
        if let Some((i, j)) = start {
            queue.push_front((i, j, keys));
            queued.insert((i, j, keys));
        }
        while !queue.is_empty() {
            distance += 1;
            for _ in 0..queue.len() {
                let (row, col, keys) = queue.pop_back().unwrap();
                for [r, c] in Self::NEIGHBOUR {
                    let r = row.wrapping_add(*r);
                    let c = col.wrapping_add(*c);
                    if let Some(value) = grid.get(r).and_then(|x| x.get(c)) {
                        let keys = match value {
                            b'#' => {
                                // ignore wall
                                continue;
                            }
                            c if c.is_ascii_lowercase() => {
                                // key
                                let keys = keys & !(1u32 << (c - b'a') as u32);
                                if keys == 0 {
                                    return distance;
                                }
                                keys
                            }
                            c if c.is_ascii_uppercase() => {
                                // handle lock
                                if keys & 1u32 << (c - b'A') as u32 != 0 {
                                    continue; // not yet unlocked
                                }
                                keys
                            }
                            _ => keys, // handle traversal
                        };
                        if !queued.contains(&(r, c, keys)) {
                            queue.push_front((r, c, keys));
                            queued.insert((r, c, keys));
                        }
                    }
                }
            }
        }
        -1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["@.a..".into(),"###.#".into(),"b.A.B".into()], 8)]
    #[case(vec!["@..aA".into(),"..B#.".into(),"....b".into()], 6)]
    #[case(vec!["@Aa".into()], -1)]
    #[case(vec!["@abcdeABCDEFf".into()], -1)]
    fn case(#[case] grid: Vec<String>, #[case] expected: i32) {
        let actual = Solution::shortest_path_all_keys(grid);
        assert_eq!(actual, expected);
    }
}
