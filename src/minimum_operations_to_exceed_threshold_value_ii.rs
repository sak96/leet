//! Solution for https://leetcode.com/problems/minimum-operations-to-exceed-threshold-value-ii
//! 3066. Minimum Operations to Exceed Threshold Value II

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = nums
            .into_iter()
            .filter_map(|x| (x < k).then(|| std::cmp::Reverse(x)))
            .collect::<std::collections::BinaryHeap<_>>();
        let mut ops = 0;
        loop {
            match heap.len() {
                x if x < 2 => break ops + x as i32,
                _ => {
                    let x = heap.pop().unwrap().0;
                    let y = heap.pop().unwrap().0;
                    let z = x.saturating_add(x).saturating_add(y);
                    ops += 1;
                    if z < k {
                        heap.push(std::cmp::Reverse(z));
                    }
                }
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
    #[case(vec![2,11,10,1,3], 10, 2)]
    #[case(vec![1,1,2,4,9], 20, 4)]
    #[case(vec![999999999,999999999,999999999], 1000000000, 2)]
    #[case(vec![15,90,76,23,66,28,37,16,45], 91, 6)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::min_operations(nums, k);
        assert_eq!(actual, expected);
    }
}
