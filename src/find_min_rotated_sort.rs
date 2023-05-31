impl Solution {
    pub fn find_min(mut nums: Vec<i32>) -> i32 {
        let mut min = nums.pop().unwrap();
        nums.binary_search_by(|x| {
            let output = x.cmp(&min).reverse();
            min = min.min(*x);
            output
        })
        .unwrap_err();
        min
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![3,4,5,1,2],1)]
    #[case(vec![4,5,6,7,0,1,2],0)]
    #[case(vec![11,13,15,17],11)]
    fn test(#[case] nums: Vec<i32>, #[case] output: i32) {
        assert_eq!(Solution::find_min(nums), output)
    }
}
