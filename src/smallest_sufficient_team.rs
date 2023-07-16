//! Solution for https://leetcode.com/problems/smallest-sufficient-team
//! 1125. Smallest Sufficient Team

impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let (skill_count, people_count) = (req_skills.len(), people.len());
        let mut skill_id = std::collections::HashMap::new();
        for (i, skill) in req_skills.into_iter().enumerate() {
            skill_id.insert(skill, i);
        }
        //  (people required | previous_user | current_user)
        let mut memory = vec![(people_count, 0, 0); 1 << skill_count];
        memory[0].0 = 0;
        for (i, skills) in people
            .into_iter()
            .enumerate()
            .filter(|(_, skill)| !skill.is_empty())
        {
            let skill_mask = skills
                .iter()
                .fold(0, |skill_set, skill| skill_set | 1 << skill_id[skill]);
            for team_skill in 0..memory.len() {
                let new_team_skill = team_skill | skill_mask;
                if memory[team_skill].0 + 1 < memory[new_team_skill].0 {
                    memory[new_team_skill] = (memory[team_skill].0 + 1, team_skill, i as i32);
                }
            }
        }

        let mut last_user_id = (1 << skill_count) - 1;
        let mut ans = vec![];
        while last_user_id != 0 {
            ans.push(memory[last_user_id].2);
            last_user_id = memory[last_user_id].1
        }

        ans
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["java".into(),"nodejs".into(),"reactjs".into()], vec![vec!["java".into()],vec!["nodejs".into()],vec!["nodejs".into(),"reactjs".into()]], vec![0,2])]
    #[case(vec!["algorithms".into(),"math".into(),"java".into(),"reactjs".into(),"csharp".into(),"aws".into()], vec![vec!["algorithms".into(),"math".into(),"java".into()],vec!["algorithms".into(),"math".into(),"reactjs".into()],vec!["java".into(),"csharp".into(),"aws".into()],vec!["reactjs".into(),"csharp".into()],vec!["csharp".into(),"math".into()],vec!["aws".into(),"java".into()]], vec![1,2])]
    fn case(
        #[case] req_skills: Vec<String>,
        #[case] people: Vec<Vec<String>>,
        #[case] mut expected: Vec<i32>,
    ) {
        let mut actual = Solution::smallest_sufficient_team(req_skills, people);
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
