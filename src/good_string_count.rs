impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let modulo = 1000000007;
        let mut result = 0;
        let mut memory = vec![0; (high + 1) as usize];
        memory[0] = 1;

        for i in 1..=high {
            let zeros = if i >= zero {
                memory[(i - zero) as usize]
            } else {
                0
            };
            let ones = if i >= one {
                memory[(i - one) as usize]
            } else {
                0
            };
            memory[i as usize] = (zeros + ones) % modulo;
            if i >= low {
                result = (result + memory[i as usize]) % modulo;
            }
        }
        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let output = Solution::count_good_strings(3, 3, 1, 1);
        assert_eq!(output, 8);
    }

    #[test]
    fn case2() {
        let output = Solution::count_good_strings(2, 3, 1, 2);
        assert_eq!(output, 5);
    }
}
