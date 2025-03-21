//! Solution for https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies
//! 2115. Find All Possible Recipes from Given Supplies

use std::collections::HashSet;

impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let mut new_supplies: HashSet<_> = supplies.into_iter().collect();
        let mut supplies: HashSet<_> = HashSet::new();
        let mut recipes: Vec<_> = recipes
            .into_iter()
            .zip(ingredients)
            .map(|(r, i)| (r, HashSet::<String>::from_iter(i)))
            .collect();
        let mut output = vec![];
        while !new_supplies.is_empty() {
            std::mem::swap(&mut new_supplies, &mut supplies);
            recipes = recipes
                .drain(..)
                .filter_map(|(recipe, mut ingredients)| {
                    ingredients = ingredients.difference(&supplies).cloned().collect();
                    if ingredients.is_empty() {
                        new_supplies.insert(recipe);
                        None
                    } else {
                        Some((recipe, ingredients))
                    }
                })
                .collect();
            output.extend(new_supplies.iter().cloned());
            supplies.clear()
        }
        output
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["bread".into()], vec![vec!["yeast".into(),"flour".into()]], vec!["yeast".into(),"flour".into(),"corn".into()], vec!["bread"])]
    #[case(vec!["bread".into(),"sandwich".into()], vec![vec!["yeast".into(),"flour".into()],vec!["bread".into(),"meat".into()]], vec!["yeast".into(),"flour".into(),"meat".into()], vec!["bread","sandwich"])]
    #[case(vec!["bread".into(),"sandwich".into(),"burger".into()], vec![vec!["yeast".into(),"flour".into()],vec!["bread".into(),"meat".into()],vec!["sandwich".into(),"meat".into(),"bread".into()]], vec!["yeast".into(),"flour".into(),"meat".into()], vec!["bread","sandwich","burger"])]
    fn case(
        #[case] recipes: Vec<String>,
        #[case] ingredients: Vec<Vec<String>>,
        #[case] supplies: Vec<String>,
        #[case] expected: Vec<&str>,
    ) {
        let actual = Solution::find_all_recipes(recipes, ingredients, supplies);
        assert_eq!(actual, expected);
    }
}
