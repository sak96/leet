impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let len = questions.len();
        let mut memory = vec![0i64; len + 1];
        for i in (0..len).rev() {
            memory[i] = memory[i + 1]
                .max(questions[i][0] as i64 + memory[len.min(i + questions[i][1] as usize + 1)]);
        }
        memory[0]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let questions = [[3, 2], [4, 3], [4, 4], [2, 5]];
        let questions: Vec<Vec<_>> = questions
            .iter()
            .map(|v| v.iter().cloned().collect())
            .collect();
        let output = Solution::most_points(questions);
        assert_eq!(output, 5);
    }

    #[test]
    fn case2() {
        let questions = [[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]];
        let questions: Vec<Vec<_>> = questions
            .iter()
            .map(|v| v.iter().cloned().collect())
            .collect();
        let output = Solution::most_points(questions);
        assert_eq!(output, 7);
    }
}
