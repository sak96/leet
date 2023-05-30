impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        // assert_ne!(divisor, 0, "divisor != 0");
        // handle signs
        let neg = (dividend < 0) ^ (divisor < 0);
        let mut multiple = 1u32;
        let mut product = divisor.unsigned_abs();
        let mut remainder = dividend.unsigned_abs();

        let mut quotient = 0;

        // get biggest 2 multiple
        while product.saturating_add(product) <= remainder {
            product += product;
            multiple += multiple;
        }

        // start dividing by 2's multiples
        while multiple > 0 {
            // if dividing is possible do it
            if remainder >= product {
                remainder -= product;
                quotient += multiple;
            }
            multiple = multiple.checked_shr(1).unwrap();
            product = product.checked_shr(1).unwrap();
        }
        if quotient > (i32::MAX as u32) {
            if neg {
                i32::MIN
            } else {
                i32::MAX
            }
        } else if neg {
            -(quotient as i32)
        } else {
            quotient as i32
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(10, 3, 3)]
    #[case::leet2(7,-3,-2)]
    #[case::leet3(-2147483648, -1, 2147483647)]
    #[case::leet4(-2147483648, 1, -2147483648)]
    #[case::mine1(2147483647, 1, 2147483647)]
    #[case::mine1(2147483647, -1, -2147483647)]
    fn test(#[case] dividend: i32, #[case] divisor: i32, #[case] quotient: i32) {
        assert_eq!(Solution::divide(dividend, divisor), quotient);
    }
}
