//! Solution for https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies
//! 2115. Find All Possible Recipes from Given Supplies

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn dfs_recipes(
        recipe: &str,
        recipes: &mut HashMap<&str, Option<Vec<String>>>,
        supplies: &HashSet<String>,
    ) {
        if supplies.contains(recipe) || recipes.get(recipe).is_some_and(|x| x.is_none()) {
            return;
        }

        if let Some((recipe, ingredients)) = recipes.remove_entry(recipe) {
            let ingredients = ingredients.unwrap();
            for i in &ingredients {
                if !supplies.contains(i.as_str()) {
                    Self::dfs_recipes(i, recipes, supplies);
                    if !recipes.contains_key(i.as_str()) {
                        return;
                    }
                }
            }
            recipes.insert(recipe, None);
        }
    }

    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let supplies: HashSet<_> = supplies.into_iter().collect();
        let mut recipe_book: HashMap<_, _> = recipes
            .iter()
            .zip(ingredients)
            .map(|(r, i)| (r.as_str(), Some(i)))
            .collect();

        for recipe in &recipes {
            Self::dfs_recipes(recipe, &mut recipe_book, &supplies);
        }

        recipe_book.keys().map(|i| i.to_string()).collect()
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
    #[case(vec!["bread".into()], vec![vec!["yeast".into(),"flour".into()]], vec!["yeast".into()], vec![])]
    #[case(vec!["bread".into(),"sandwich".into()], vec![vec!["yeast".into(),"flour".into()],vec!["bread".into(),"meat".into()]], vec!["yeast".into(),"flour".into(),"meat".into()], vec!["bread","sandwich"])]
    #[case(vec!["bread".into(),"sandwich".into(),"burger".into()], vec![vec!["yeast".into(),"flour".into()],vec!["bread".into(),"meat".into()],vec!["sandwich".into(),"meat".into(),"bread".into()]], vec!["yeast".into(),"flour".into(),"meat".into()], vec!["bread","sandwich","burger"])]
    #[case(vec!["bread".into(),"burger".into(),"sandwich".into()], vec![vec!["yeast".into(),"flour".into()],vec!["sandwich".into(),"meat".into(),"bread".into()],vec!["bread".into(),"meat".into()]], vec!["yeast".into(),"flour".into(),"meat".into()], vec!["bread","sandwich","burger"])]
    fn case(
        #[case] recipes: Vec<String>,
        #[case] ingredients: Vec<Vec<String>>,
        #[case] supplies: Vec<String>,
        #[case] mut expected: Vec<&str>,
    ) {
        let mut actual = Solution::find_all_recipes(recipes, ingredients, supplies);
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
