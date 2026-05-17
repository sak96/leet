//! Solution for https://leetcode.com/problems/jump-game-iii
//! 1306. Jump Game III

impl Solution {
    pub fn can_reach(mut arr: Vec<i32>, start: i32) -> bool {
        let len = arr.len();
        let mut queue = vec![start as usize];
        let mut next = vec![];
        while !queue.is_empty() {
            for i in queue.drain(..) {
                let val = arr[i] as usize;
                arr[i] = len as i32;
                if val == len {
                    continue;
                }
                if val == 0 {
                    return true;
                }
                if i >= val {
                    next.push(i - val);
                }
                if i + val < len {
                    next.push(i + val);
                }
            }
            std::mem::swap(&mut next, &mut queue);
            next.clear();
        }
        false
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![4,2,3,0,3,1,2], 5, true)]
    #[case(vec![4,2,3,0,3,1,2], 0, true)]
    #[case(vec![3,0,2,1,2], 2, false)]
    fn case(#[case] arr: Vec<i32>, #[case] start: i32, #[case] expected: bool) {
        let actual = Solution::can_reach(arr, start);
        assert_eq!(actual, expected);
    }
}
