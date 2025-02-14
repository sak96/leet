//! Solution for https://leetcode.com/problems/product-of-the-last-k-numbers
//! 1352. Product of the Last K Numbers

pub struct ProductOfNumbers {
    product: i32,
    stream: Vec<i32>,
    max_zero_index: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    pub fn new() -> Self {
        Self {
            product: 1,
            stream: vec![1],
            max_zero_index: 0,
        }
    }

    pub fn add(&mut self, num: i32) {
        if num == 0 {
            self.max_zero_index = 0;
            self.stream.push(1);
            self.product = 1;
        } else {
            self.product *= num;
            self.max_zero_index += 1;
            self.stream.push(self.product);
        }
    }

    pub fn get_product(&self, k: i32) -> i32 {
        if k > self.max_zero_index {
            0
        } else {
            self.product / self.stream[self.stream.len() - k as usize - 1]
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
