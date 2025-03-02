//! Solution for https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values
//! 2570. Merge Two 2D Arrays by Summing Values

impl Solution {
    pub fn merge_arrays(mut nums1: Vec<Vec<i32>>, mut nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        nums1.reverse();
        nums2.reverse();
        let (mut n1, mut n2) = (None, None);
        let mut num = vec![];
        loop {
            if n1.is_none() {
                n1 = nums1.pop();
            }
            if n2.is_none() {
                n2 = nums2.pop();
            }
            match (&n1, &n2) {
                (Some(v1), Some(v2)) => match v1[0].cmp(&v2[0]) {
                    std::cmp::Ordering::Less => {
                        num.push(vec![v1[0], v1[1]]);
                        n1 = None;
                    }
                    std::cmp::Ordering::Greater => {
                        num.push(vec![v2[0], v2[1]]);
                        n2 = None;
                    }
                    std::cmp::Ordering::Equal => {
                        num.push(vec![v1[0], v2[1] + v1[1]]);
                        n1 = None;
                        n2 = None;
                    }
                },
                (Some(ent), None) | (None, Some(ent)) => {
                    num.push(ent.to_vec());
                    n1 = None;
                    n2 = None;
                }
                (None, None) => {
                    break num;
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
    #[case(vec![vec![1,2],vec![2,3],vec![4,5]], vec![vec![1,4],vec![3,2],vec![4,1]], vec![vec![1,6],vec![2,3],vec![3,2],vec![4,6]])]
    //#[case(vec![vec![2,4],vec![3,6],vec![5,5]], vec![vec![1,3],vec![4,3]], vec![vec![1,3],vec![2,4],vec![3,6],vec![4,3],vec![5,5]])]
    fn case(
        #[case] nums1: Vec<Vec<i32>>,
        #[case] nums2: Vec<Vec<i32>>,
        #[case] expected: Vec<Vec<i32>>,
    ) {
        let actual = Solution::merge_arrays(nums1, nums2);
        assert_eq!(actual, expected);
    }
}
