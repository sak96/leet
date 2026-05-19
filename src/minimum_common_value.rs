//! Solution for https://leetcode.com/problems/minimum-common-value
//! 2540. Minimum Common Value

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut it1 = nums1.into_iter();
        let mut it2 = nums2.into_iter();
        let mut a = it1.next().expect("nums1.len > 1");
        let mut b = it2.next().expect("nums2.len > 1");
        loop {
            match a.cmp(&b) {
                std::cmp::Ordering::Less => {
                    if let Some(a_) = it1.next() {
                        a = a_
                    } else {
                        break -1;
                    }
                }
                std::cmp::Ordering::Equal => break a,
                std::cmp::Ordering::Greater => {
                    if let Some(b_) = it2.next() {
                        b = b_
                    } else {
                        break -1;
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
    #[case(vec![1,2,3], vec![2,4], 2)]
    #[case(vec![1,2,3,6], vec![2,3,4,5], 2)]
    #[case(vec![1,2,3], vec![4], -1)]
    fn case(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::get_common(nums1, nums2);
        assert_eq!(actual, expected);
    }
}
