impl Solution {
    pub fn find_min(mut nums: Vec<i32>) -> i32 {
        let mut min = nums.pop().unwrap();
        while let Some(other) = nums.pop() {
            if other <= min {
                min = other
            } else {
                break;
            }
        }
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

    #[test]
    fn case3() {
        let nums = [3, 1, 3, 3];
        let output = 1;
        assert_eq!(Solution::find_min(nums.into()), output)
    }
}
