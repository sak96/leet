//! Solution for https://leetcode.com/problems/rotated-digits
//! 788. Rotated Digits

impl Solution {
    const ROTATABLE_COUNT: [i32; 10] = [1, 2, 3, 3, 3, 4, 5, 5, 6, 7];
    const ROTATABLE_IDENTICAL_COUNT: [i32; 10] = [1, 2, 2, 2, 2, 2, 2, 2, 3, 3];
    const ROTATABLE_IDENTICAL_DIGITS: [i32; 4] = [2, 5, 6, 9];
    const ROTATABLE_DIGITS: [i32; 7] = [0, 1, 2, 5, 6, 8, 9];
    pub fn rotated_digits(mut num: i32) -> i32 {
        let mut digits = Vec::with_capacity(4);
        let mut answer = 0;
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        let mut seen_non_identical = false;
        for (pos, digit) in digits.into_iter().enumerate().rev() {
            let mut sum = 0;
            // sums = a(b-1)0* -> a(b-1)9*
            if digit != 0 || pos == 0 {
                let idx = if pos != 0 { digit - 1 } else { digit } as usize;
                // all digits which are rotatable
                sum += Self::ROTATABLE_COUNT[idx] * 7i32.pow(pos as u32);
                if !seen_non_identical {
                    // all digits from only identical rotation digits
                    // if we have seen non identical then we don't need this
                    sum -= Self::ROTATABLE_IDENTICAL_COUNT[idx] * 3i32.pow(pos as u32)
                }
            }
            answer += sum;
            if Self::ROTATABLE_IDENTICAL_DIGITS.contains(&digit) {
                // mark seeing non identical
                seen_non_identical = true;
            }
            if !Self::ROTATABLE_DIGITS.contains(&digit) {
                // no need to calculate all the new number will have non rotatable digit
                break;
            }
        }
        answer
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, 1)]
    #[case(20, 9)]
    #[case(30, 15)]
    #[case(100, 40)]
    #[case(102, 41)]
    #[case(105, 42)]
    #[case(106, 43)]
    #[case(109, 44)]
    #[case(110, 44)]
    #[case(111, 44)]
    #[case(112, 45)]
    #[case(113, 45)]
    #[case(114, 45)]
    #[case(115, 46)]
    #[case(116, 47)]
    #[case(118, 47)]
    #[case(119, 48)]
    #[case(120, 49)]
    #[case(200, 81)]
    #[case(10, 4)]
    #[case(1, 0)]
    #[case(2, 1)]
    #[case(123, 51)]
    #[case(122, 51)]
    #[case(1234, 417)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::rotated_digits(n);
        assert_eq!(actual, expected, "{}", n);
    }
}
