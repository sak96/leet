//! Solution for https://leetcode.com/problems/longest-common-suffix-queries
//! 3093. Longest Common Suffix Queries

pub struct TreeNode {
    words: [Option<Box<TreeNode>>; (b'z' + 1 - b'a') as usize],
    len: usize,
    idx: i32,
}

impl TreeNode {
    pub fn new(idx: i32, len: usize) -> Self {
        Self {
            len,
            idx,
            words: [const { None }; (b'z' + 1 - b'a') as usize],
        }
    }

    pub fn insert(self: &mut TreeNode, word: &[u8], len: usize, idx: i32) {
        if self.len > len {
            self.len = len;
            self.idx = idx;
        }
        if let Some((ch, word)) = word.split_first() {
            let i = (ch - b'a') as usize;
            match self.words.get_mut(i) {
                Some(Some(node)) => {
                    node.insert(word, len, idx);
                }
                Some(None) => {
                    let mut node = TreeNode::new(idx, len);
                    node.insert(word, len, idx);
                    self.words[i] = Some(Box::new(node));
                }
                _ => {
                    unreachable!()
                }
            }
        }
    }
    pub fn find(self: &TreeNode, word: &[u8]) -> Option<i32> {
        if let Some((ch, word)) = word.split_first() {
            let i = (ch - b'a') as usize;
            if let Some(Some(node)) = self.words.get(i) {
                Some(node.find(word).unwrap_or(self.idx))
            } else {
                Some(self.idx)
            }
        } else {
            Some(self.idx)
        }
    }
}

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let mut tree = TreeNode::new(0, usize::MAX);
        for (i, word) in words_container.into_iter().enumerate() {
            let mut word = word.into_bytes();
            word.reverse();
            tree.insert(&word, word.len(), i as _);
        }
        words_query
            .into_iter()
            .map(|word| {
                let mut word = word.into_bytes();
                word.reverse();
                tree.find(&word).unwrap_or(0)
            })
            .collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["abcd".into(),"bcd".into(),"xbcd".into()], vec!["cd".into(),"bcd".into(),"xyz".into()], vec![1,1,1])]
    #[case(vec!["abcdefgh".into(),"poiuygh".into(),"ghghgh".into()], vec!["gh".into(),"acbfgh".into(),"acbfegh".into()], vec![2,0,2])]
    fn case(
        #[case] words_container: Vec<String>,
        #[case] words_query: Vec<String>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::string_indices(words_container, words_query);
        assert_eq!(actual, expected);
    }
}
