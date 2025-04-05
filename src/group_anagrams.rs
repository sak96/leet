//! Solution for https://leetcode.com/problems/group-anagrams
//! 49. Group Anagrams

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash = std::collections::HashMap::new();
        for s in strs {
            let mut b = s.as_bytes().to_vec();
            b.sort_unstable();
            hash.entry(b).or_insert(vec![]).push(s);
        }
        hash.into_values().collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    fn sort_(input: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut input: Vec<Vec<String>> = input
            .into_iter()
            .map(|mut v| {
                v.sort_unstable();
                v
            })
            .collect();
        input.sort_unstable();
        input
    }

    #[rstest]
    #[case(
        vec!["eat".into(),"tea".into(),"tan".into(),"ate".into(),"nat".into(),"bat".into()],
         vec![vec!["bat".into()],vec!["nat".into(),"tan".into()],vec!["ate".into(),"eat".into(),"tea".into()]]
    )]
    #[case(vec!["".into()], vec![vec!["".into()]])]
    #[case(vec!["a".into()], vec![vec!["a".into()]])]
    fn case(#[case] strs: Vec<String>, #[case] expected: Vec<Vec<String>>) {
        let actual = Solution::group_anagrams(strs);
        assert_eq!(sort_(actual), sort_(expected));
    }
}
