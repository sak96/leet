//! Solution for https://leetcode.com/problems/jump-game-iv
//! 1345. Jump Game IV

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let len = arr.len();
        let mut seen = vec![false; len];
        let mut map = std::collections::HashMap::new();
        for (i, &val) in arr.iter().enumerate() {
            map.entry(val as usize).or_insert_with(Vec::new).push(i);
        }
        let mut queue = vec![0];
        let mut next = vec![];
        let mut depth = 0;
        'outer: loop {
            while let Some(idx) = queue.pop() {
                if seen[idx] {
                    continue;
                }
                seen[idx] = true;
                if idx == len - 1 {
                    break 'outer;
                }
                let val = arr[idx];
                if let Some(v) = map.remove(&(val as usize)) {
                    next.extend(v)
                }
                if idx < len && !seen[idx + 1] {
                    next.push(idx + 1);
                }
                if idx != 0 && !seen[idx - 1] {
                    next.push(idx - 1);
                }
            }
            depth += 1;
            std::mem::swap(&mut queue, &mut next);
        }
        depth
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![100,-23,-23,404,100,23,23,23,3,404],3)]
    #[case(vec![7],0)]
    #[case(vec![7,6,9,6,9,6,9,7], 1)]
    fn case(#[case] arr: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_jumps(arr);
        assert_eq!(actual, expected);
    }
}
