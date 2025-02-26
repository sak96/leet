//! Solution for https://leetcode.com/problems/product-of-the-last-k-numbers
//! 1352. Product of the Last K Numbers

pub struct ProductOfNumbers(Vec<i32>);

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Default for ProductOfNumbers {
    fn default() -> Self {
        Self::new()
    }
}

impl ProductOfNumbers {
    pub fn new() -> Self {
        Self(vec![1])
    }

    pub fn add(&mut self, num: i32) {
        if num == 0 {
            self.0.truncate(1);
        } else {
            match self.0.last() {
                None => self.0.push(num),
                Some(&x) => self.0.push(x * num),
            }
        }
    }

    pub fn get_product(&self, k: i32) -> i32 {
        match self.0.len().checked_sub(k as usize + 1) {
            None => 0,
            Some(x) => self.0.last().unwrap() / self.0[x],
        }
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leet1() {
        let mut product_of_numbers = ProductOfNumbers::new();
        product_of_numbers.add(3);
        product_of_numbers.add(0);
        product_of_numbers.add(2);
        product_of_numbers.add(5);
        product_of_numbers.add(4);
        assert_eq!(product_of_numbers.get_product(2), 20);
        assert_eq!(product_of_numbers.get_product(3), 40);
        assert_eq!(product_of_numbers.get_product(4), 0);
        product_of_numbers.add(8);
        assert_eq!(product_of_numbers.get_product(2), 32);
    }
}
