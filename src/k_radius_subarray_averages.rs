//! Solution for https://leetcode.com/problems/k-radius-subarray-averages
impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let radius = 2 * k + 1;
        let k = k as usize;
        if radius as usize > nums.len() {
            return vec![-1; nums.len()];
        }
        let mut output = vec![-1; k];
        let mut sum = nums.iter().map(|&i| i as i64).take(2 * k).sum::<i64>();
        let mut prev = 0;
        for (&i, &j) in nums.iter().zip(nums.iter().skip(2 * k)) {
            sum -= prev;
            sum += j as i64;
            prev = i as i64;
            output.push((sum / radius as i64) as i32);
        }
        output.extend(vec![-1; k]);
        output
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::leet1([7,4,3,9,1,8,5,2,6],3,[-1,-1,-1,5,4,4,-1,-1,-1])]
    #[case::leet2([100000],0,[100000])]
    #[case::leet3([8],100000,[-1])]
    #[case::leet3(vec![100000;100000],4000,{
        let mut vec = vec![-1;4000];
        vec.extend(vec![100000;100000 - 2 * 4000]);
        vec.extend(vec![-1;4000]);
        vec
    })]
    fn test(#[case] nums: impl AsRef<[i32]>, #[case] k: i32, #[case] output: impl AsRef<[i32]>) {
        assert_eq!(
            Solution::get_averages(nums.as_ref().into(), k),
            output.as_ref()
        );
    }
}
