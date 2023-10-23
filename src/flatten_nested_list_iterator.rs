//! Solution for https://leetcode.com/problems/flatten-nested-list-iterator
//! 341. Flatten Nested List Iterator

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub struct NestedIterator(Vec<i32>);

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    #[allow(non_snake_case)]
    pub fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut output = vec![];
        Self::flatten(nestedList, &mut output);
        output.reverse();
        Self(output)
    }

    fn flatten(item: Vec<NestedInteger>, output: &mut Vec<i32>) {
        for i in item {
            match i {
                NestedInteger::Int(x) => output.push(x),
                NestedInteger::List(x) => Self::flatten(x, output),
            }
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> i32 {
        self.0.pop().unwrap()
    }

    pub fn has_next(&self) -> bool {
        !self.0.is_empty()
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    use super::NestedInteger as ni;

    #[rstest]
    // #[case(vec![ni::List(vec![ni::Int(1),ni::Int(1)]),ni::Int(2),ni::List(vec![ni::Int(1),ni::Int(1)])], vec![1,1,2,1,1])]
    // #[case(vec![ni::Int(1),ni::List(vec![ni::Int(4),ni::List(vec![ni::Int(6)])])], vec![1,4,6])]
    #[case(vec![ni::List(vec![])], vec![])]
    // #[case(vec![ni::Int(1),ni::List(vec![ni::Int(4),ni::List(vec![ni::Int(6)])]),ni::List(vec![])], vec![1,4,6])]
    fn case(#[case] nested_list: Vec<NestedInteger>, #[case] expected: Vec<i32>) {
        let mut obj = NestedIterator::new(nested_list);
        let mut actual = vec![];
        while obj.has_next() {
            actual.push(obj.next());
        }
        assert_eq!(actual, expected);
    }
}
