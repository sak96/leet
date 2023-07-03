impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut ugly_num = Vec::with_capacity(n as usize);
        ugly_num.push(1);
        // ith index to be qualified as next multiple
        let mut multiples = (0, 0, 0);
        for _ in 1..n {
            // reason: ugly[i] * multiple < ugly[i+1] * multiple
            let nums = [
                ugly_num[multiples.0] * 2,
                ugly_num[multiples.1] * 3,
                ugly_num[multiples.2] * 5,
            ];
            let min = nums.iter().min().copied().unwrap();
            ugly_num.push(min);
            if min == nums[0] {
                multiples.0 += 1;
            }
            if min == nums[1] {
                multiples.1 += 1;
            }
            if min == nums[2] {
                multiples.2 += 1;
            }
        }
        ugly_num.pop().unwrap()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(10, 12)]
    #[case(1, 1)]
    fn test(#[case] n: i32, #[case] sol: i32) {
        assert_eq!(Solution::nth_ugly_number(n), sol);
    }
}
