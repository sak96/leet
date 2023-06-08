//! Solution for https://leetcode.com/problems/substring-with-concatenation-of-all-words
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut zero = 0;
        let word_len = words[0].len();
        let word_count = words.len();
        let mut word_map = std::collections::HashMap::<_, usize>::new();
        for word in &words {
            *word_map.entry(word.as_str()).or_default() += 1;
        }
        let mut indices = vec![];
        let mut seen_words = std::collections::VecDeque::with_capacity(word_count);
        for offset in 0..word_len {
            if let Some(w) = seen_words.pop_front() {
                *word_map.get_mut(w).unwrap() += 1;
            }
            let mut windows =
                (offset..(s.len().saturating_sub(word_count * word_len - 1))).step_by(word_len);
            while let Some(index) = windows.next() {
                let sub_string = &s[index..];
                let mut start = seen_words.len() * word_len;
                while seen_words.len() < word_count {
                    let word = &sub_string[start..start + word_len];
                    let count = if let Some(count) = word_map.get_mut(word) {
                        count
                    } else {
                        &mut zero
                    };
                    if *count > 0 {
                        seen_words.push_back(word);
                        *count -= 1;
                        start += word_len;
                    } else {
                        // when you see more than you should!!
                        while let Some(w) = seen_words.pop_front() {
                            if w.eq(word) {
                                seen_words.push_front(w);
                                break;
                            }
                            *word_map.get_mut(w).unwrap() += 1;
                            windows.next();
                        }
                        break;
                    }
                }
                if seen_words.len() == word_count {
                    indices.push(index as i32);
                }
                if let Some(w) = seen_words.pop_front() {
                    *word_map.get_mut(w).unwrap() += 1;
                }
            }
        }
        indices
    }
}
pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("barfoothefoobarman", vec!["foo","bar"],vec![0,9])]
    #[case("wordgoodgoodgoodbestword", vec!["word","good","best","word"],vec![])]
    #[case("barfoofoobarthefoobarman", vec!["bar","foo","the"], vec![6,9,12])]
    #[case("wordgoodgoodwordgoodbestwordword", vec!["word","good","best","word"],vec![12,16])]
    #[case("a",vec!["a","a"], vec![])]
    #[case("aaaaaaaaaaaaaa",vec!["aa","aa"], vec![0,1,2,3,4,5,6,7,8,9,10])]
    fn case(#[case] s: String, #[case] words: Vec<&str>, #[case] expected: Vec<i32>) {
        dbg!(&s, &words);
        let words = words.into_iter().map(|x| x.to_string()).collect();
        let mut actual = Solution::find_substring(s, words);
        actual.sort_unstable();
        assert_eq!(actual, expected);
    }
}
