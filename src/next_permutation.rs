impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if let Some(i) = nums.as_slice().windows(2).rev().position(|x| x[1] > x[0]) {
            let i = nums.len() - 1 - i;
            let ith = nums[i - 1];
            nums[i..].sort_unstable();
            // TODO: may be Use binary search
            let p = nums[i..]
                .iter()
                .position(|x| x > &ith)
                .expect("there should be some element")
                + i;
            nums.swap(i - 1, p);
        } else {
            nums.sort_unstable();
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3],vec![1,3,2])]
    #[case(vec![3,2,1],vec![1,2,3])]
    #[case(vec![1,1,5],vec![1,5,1])]
    #[case(vec![1,5,1],vec![5,1,1])]
    #[case(vec![5,1,1],vec![1,1,5])]
    #[case(vec![1,2,2],vec![2,1,2])]
    #[case(vec![1,2,2],vec![2,1,2])]
    #[case(vec![1, 2, 3, 4],vec![1, 2, 4, 3])]
    #[case(vec![1, 2, 4, 3],vec![1, 3, 2, 4])]
    #[case(vec![1, 3, 2, 4],vec![1, 3, 4, 2])]
    #[case(vec![1, 3, 4, 2],vec![1, 4, 2, 3])]
    #[case(vec![1, 4, 2, 3],vec![1, 4, 3, 2])]
    #[case(vec![1, 4, 3, 2],vec![2, 1, 3, 4])]
    #[case(vec![2, 1, 3, 4],vec![2, 1, 4, 3])]
    #[case(vec![2, 1, 4, 3],vec![2, 3, 1, 4])]
    #[case(vec![2, 3, 1, 4],vec![2, 3, 4, 1])]
    #[case(vec![2, 3, 4, 1],vec![2, 4, 1, 3])]
    #[case(vec![2, 4, 1, 3],vec![2, 4, 3, 1])]
    #[case(vec![2, 4, 3, 1],vec![3, 1, 2, 4])]
    #[case(vec![3, 1, 2, 4],vec![3, 1, 4, 2])]
    #[case(vec![3, 1, 4, 2],vec![3, 2, 1, 4])]
    #[case(vec![3, 2, 1, 4],vec![3, 2, 4, 1])]
    #[case(vec![3, 2, 4, 1],vec![3, 4, 1, 2])]
    #[case(vec![3, 4, 1, 2],vec![3, 4, 2, 1])]
    #[case(vec![3, 4, 2, 1],vec![4, 1, 2, 3])]
    #[case(vec![4, 1, 2, 3],vec![4, 1, 3, 2])]
    #[case(vec![4, 1, 3, 2],vec![4, 2, 1, 3])]
    #[case(vec![4, 2, 1, 3],vec![4, 2, 3, 1])]
    #[case(vec![4, 2, 3, 1],vec![4, 3, 1, 2])]
    #[case(vec![4, 3, 1, 2],vec![4, 3, 2, 1])]
    #[case(vec![4, 3, 2, 1],vec![1, 2, 3, 4])]
    fn test(#[case] input: Vec<i32>, #[case] output: Vec<i32>) {
        let mut nums = input.clone();
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, output, "{input:?}");
    }
}
