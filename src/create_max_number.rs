use std::collections::HashMap;
impl Solution {
    fn merge(mut nums1: &[i32], mut nums2: &[i32], merged: &mut Vec<i32>) {
        while !nums1.is_empty() || !nums2.is_empty() {
            // pop from bigger vector
            if nums1 > nums2 {
                merged.push(nums1[0]);
                nums1 = &nums1[1..];
            } else {
                merged.push(nums2[0]);
                nums2 = &nums2[1..];
            }
        }
    }

    fn get_max_index(nums: &[i32]) -> Option<usize> {
        nums.iter()
            .enumerate()
            .min_by_key(|(idx, &value)| (-value, *idx))
            .map(|(idx, _)| idx)
    }

    pub fn max_number_dp<'a>(
        mut nums1: &[i32],
        mut nums2: &[i32],
        mut k: usize,
        memory: &'a mut HashMap<(usize, usize, usize), Vec<i32>>,
    ) -> &'a std::vec::Vec<i32> {
        let key = (nums1.len(), nums2.len(), k);
        if memory.contains_key(&key) {
            // Dynamic programming 101
            // NOTE: entry API has some issue with lifetime
            //       hence using contains_key and get
            return memory.get(&key).unwrap();
        } else {
            let mut ans = Vec::with_capacity(k);

            // until all k elements are found.
            while k > 0 {
                // number of drops
                let drops = nums1.len() + nums2.len() - k;
                if drops == 0 {
                    Self::merge(nums1, nums2, &mut ans);
                    break;
                }

                // Get max elements for slice in drops + 1 element
                // which ever is max elements can be added to answer
                let nums1_idx = Self::get_max_index(&nums1[..(drops + 1).min(nums1.len())]);
                let nums2_idx = Self::get_max_index(&nums2[..(drops + 1).min(nums2.len())]);
                match nums1_idx
                    .and_then(|x| nums1.get(x))
                    .cmp(&nums2_idx.and_then(|x| nums2.get(x)))
                {
                    std::cmp::Ordering::Less => {
                        let idx = nums2_idx.unwrap();
                        ans.push(nums2[idx]);
                        nums2 = &nums2[idx + 1..];
                        k -= 1;
                    }
                    std::cmp::Ordering::Greater => {
                        let idx = nums1_idx.unwrap();
                        ans.push(nums1[idx]);
                        nums1 = &nums1[idx + 1..];
                        k -= 1;
                    }
                    std::cmp::Ordering::Equal => {
                        // if both are each check both solution
                        ans.push(nums1[nums1_idx.unwrap()]);
                        let len = ans.len();

                        let max1 = &nums1[nums1_idx.unwrap() + 1..];
                        ans.extend(Self::max_number_dp(max1, nums2, k - 1, memory));

                        let max2 = &nums2[nums2_idx.unwrap() + 1..];
                        let nums = Self::max_number_dp(nums1, max2, k - 1, memory);

                        if &ans[len..] < nums {
                            // truncate keeps the capacity
                            ans.truncate(len);
                            ans.extend(nums);
                        }
                        break;
                    }
                }
            }
            // store output in memory
            memory.entry(key).or_insert(ans)
        }
    }

    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let mut memory = HashMap::new();
        Self::max_number_dp(&nums1, &nums2, k as usize, &mut memory);
        memory
            .remove(&(nums1.len(), nums2.len(), k as usize))
            .unwrap()
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let num1 = [3, 4, 6, 5];
        let num2 = [9, 1, 2, 5, 8, 3];
        let k = 5;
        let answer = [9, 8, 6, 5, 3];
        assert_eq!(Solution::max_number(num1.into(), num2.into(), k), answer);
    }

    #[test]
    fn case2() {
        let num1 = [6, 7];
        let num2 = [6, 0, 4];
        let k = 5;
        let answer = [6, 7, 6, 0, 4];
        assert_eq!(Solution::max_number(num1.into(), num2.into(), k), answer);
    }

    #[test]
    fn case3() {
        let num1 = [3, 9];
        let num2 = [8, 9];
        let k = 3;
        let answer = [9, 8, 9];
        assert_eq!(Solution::max_number(num1.into(), num2.into(), k), answer);
    }

    #[test]
    fn case4() {
        let num1 = [8, 9];
        let num2 = [3, 9];
        // 9 8 9
        let k = 3;
        let answer = [9, 8, 9];
        assert_eq!(Solution::max_number(num1.into(), num2.into(), k), answer);
    }

    #[test]
    fn case5() {
        let num1 = [8, 9];
        let num2 = [3, 9];
        let k = 2;
        let answer = [9, 9];
        assert_eq!(Solution::max_number(num1.into(), num2.into(), k), answer);
    }

    #[test]
    fn case6() {
        let num1 = [9, 8, 3, 3];
        let num2 = [7, 8, 3, 3];
        let k = 4;
        let answer = [9, 8, 8, 3];
        assert_eq!(Solution::max_number(num1.into(), num2.into(), k), answer);
    }

    #[test]
    fn case7() {
        let num1 = [5, 2, 2];
        let num2 = [6, 4, 1];
        let k = 3;
        let answer = [6, 5, 4];
        assert_eq!(Solution::max_number(num1.into(), num2.into(), k), answer);
    }

    #[test]
    fn case8() {
        let num1 = [5, 1, 0];
        let num2 = [5, 2, 1];
        let k = 3;
        let answer = [5, 5, 2];
        assert_eq!(Solution::max_number(num1.into(), num2.into(), k), answer);
    }

    #[test]
    fn case9() {
        let num1 = [3, 8, 5, 3, 4];
        let num2 = [8, 7, 3, 6, 8];
        let k = 5;
        let answer = [8, 8, 8, 5, 4];
        assert_eq!(Solution::max_number(num1.into(), num2.into(), k), answer);
    }

    #[test]
    fn case10() {
        let num1 = [
            4, 6, 9, 1, 0, 6, 3, 1, 5, 2, 8, 3, 8, 8, 4, 7, 2, 0, 7, 1, 9, 9, 0, 1, 5, 9, 3, 9, 3,
            9, 7, 3, 0, 8, 1, 0, 9, 1, 6, 8, 8, 4, 4, 5, 7, 5, 2, 8, 2, 7, 7, 7, 4, 8, 5, 0, 9, 6,
            9, 2,
        ];
        let num2 = [
            9, 9, 4, 5, 1, 2, 0, 9, 3, 4, 6, 3, 0, 9, 2, 8, 8, 2, 4, 8, 6, 5, 4, 4, 2, 9, 5, 0, 7,
            3, 7, 5, 9, 6, 6, 8, 8, 0, 2, 4, 2, 2, 1, 6, 6, 5, 3, 6, 2, 9, 6, 4, 5, 9, 7, 8, 0, 7,
            2, 3,
        ];
        let k = 60;
        let answer = [
            9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 8, 8, 6, 8, 8, 4, 4, 5, 7, 5, 2, 8, 2, 7, 7, 7,
            4, 8, 5, 0, 9, 6, 9, 2, 0, 2, 4, 2, 2, 1, 6, 6, 5, 3, 6, 2, 9, 6, 4, 5, 9, 7, 8, 0, 7,
            2, 3,
        ];
        assert_eq!(Solution::max_number(num1.into(), num2.into(), k), answer);
    }

    #[test]
    fn case11() {
        let num1 = [1, 9, 7, 1, 8, 9, 1];
        let num2 = [9, 1, 8, 9, 8, 9];
        let k = 5;
        let answer = [9, 9, 9, 9, 9];
        assert_eq!(Solution::max_number(num1.into(), num2.into(), k), answer);
    }

    #[test]
    fn case12() {
        let num1 = [9, 1, 8, 1, 9];
        let num2 = [9, 3, 1, 1, 9];
        let k = 6;
        let answer = [9, 9, 9, 8, 1, 9];
        assert_eq!(
            Solution::max_number(num1.into(), num2.into(), k),
            answer,
            "{:?}, {:?}",
            num1,
            num2
        );
    }
}
