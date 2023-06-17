//! Solution for https://leetcode.com/problems/make-array-strictly-increasing
use std::collections::HashMap;
impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        arr2.sort_unstable();
        arr2.dedup();
        let mut arr1 = arr1.as_slice();
        let mut memory = Vec::new();
        let mut new_memory = HashMap::new();
        memory.push((i32::MIN, 0)); // base case
        while !arr1.is_empty() && !memory.is_empty() {
            let cur = arr1[0];
            for (prev, count) in memory {
                // no swap
                if prev < cur {
                    new_memory
                        .entry(cur)
                        .and_modify(|count_| *count_ = count.min(*count_))
                        .or_insert(count);
                }
                // with swap
                if let Some(&new_prev) = arr2.get(arr2.partition_point(|&x| x <= prev)) {
                    let count = count + 1; // swap done
                    new_memory
                        .entry(new_prev)
                        .and_modify(|count_| *count_ = count.min(*count_))
                        .or_insert(count);
                }
            }
            arr1 = &arr1[1..];
            memory = new_memory.drain().collect();
        }
        memory.into_iter().map(|(_, v)| v).min().unwrap_or(-1)
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::leet1([1,5,3,6,7],[1,3,2,4],1)]
    #[case::leet2([1,5,3,6,7],[4,3,1],2)]
    #[case::leet3([1,5,3,6,7],[1,6,3,3],-1)]
    #[case::leet4([5,16,19,2,1,12,7,14,5,16],[6,17,4,3,6,13,4,3,18,17,16,7,14,1,16],8)]
    fn test(#[case] arr1: impl AsRef<[i32]>, #[case] arr2: impl AsRef<[i32]>, #[case] output: i32) {
        assert_eq!(
            Solution::make_array_increasing(arr1.as_ref().to_vec(), arr2.as_ref().to_vec()),
            output
        );
    }
}
