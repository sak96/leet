//! Solution for https://leetcode.com/problems/partition-array-according-to-given-pivot
//! 2161. Partition Array According to Given Pivot

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut pre = Vec::with_capacity(nums.len());
        let mut post = vec![];
        let mut same = 0;
        for i in nums {
            match i.cmp(&pivot) {
                std::cmp::Ordering::Less => pre.push(i),
                std::cmp::Ordering::Equal => same += 1,
                std::cmp::Ordering::Greater => post.push(i),
            }
        }
        pre.extend([pivot].repeat(same));
        pre.extend(post);
        pre
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![9,12,5,10,14,3,10], 10,vec![9,5,3,10,10,12,14])]
    #[case(vec![-3,4,3,2], 2,  vec![-3,2,4,3])]
    fn case(#[case] nums: Vec<i32>, #[case] pivot: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::pivot_array(nums, pivot);
        assert_eq!(actual, expected);
    }
}
