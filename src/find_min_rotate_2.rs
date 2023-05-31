impl Solution {
    pub fn find_min(mut nums: Vec<i32>) -> i32 {
        let mut min = nums.pop().unwrap();
        nums.binary_search_by(|x| {
            let output = x.cmp(&min).reverse();
            min = min.min(*x);
            if output == std::cmp::Ordering::Equal {
                std::cmp::Ordering::Greater
            } else {
                output
            }
        })
        .unwrap_err();
        min
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = [1, 3, 5];
        let output = 1;
        assert_eq!(Solution::find_min(nums.into()), output)
    }

    #[test]
    fn case2() {
        let nums = [2, 2, 2, 0, 1];
        let output = 0;
        assert_eq!(Solution::find_min(nums.into()), output)
    }
}
