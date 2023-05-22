impl Solution {
    pub fn top_k_frequent(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        nums.sort_unstable();
        let mut map = std::collections::HashMap::new();
        for n in nums {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut output: Vec<_> = map.into_iter().collect();
        output.sort_unstable_by_key(|(_, c)| *c);
        output
            .iter()
            .rev()
            .map(|(n, _)| *n)
            .take(k as usize)
            .collect()
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,1,1,2,2,3],2,vec![1,2])]
    #[case(vec![1],1,vec![1])]
    #[case(vec![1,2],2,vec![1,2])]
    #[case(vec![4,1,-1,2,-1,2,3],2,vec![-1,2])]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] mut expected: Vec<i32>) {
        let mut output = Solution::top_k_frequent(nums, k);
        output.sort_unstable();
        expected.sort_unstable();
        assert_eq!(output, expected);
    }
}
