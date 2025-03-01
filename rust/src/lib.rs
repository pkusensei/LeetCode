mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn find_all_recipes(
    recipes: Vec<String>,
    ingredients: Vec<Vec<String>>,
    supplies: Vec<String>,
) -> Vec<String> {
    let mut adj = HashMap::<&str, Vec<&str>>::new();
    let mut indegs = HashMap::<&str, _>::new();
    for (rec, ings) in recipes.iter().zip(ingredients.iter()) {
        for ing in ings.iter() {
            adj.entry(ing).or_default().push(rec);
            *indegs.entry(rec).or_insert(0) += 1;
        }
    }
    let mut queue = VecDeque::<&str>::new();
    for sup in supplies.iter() {
        queue.push_back(sup);
        *indegs.entry(sup).or_insert(0) = 0;
    }
    let mut res = vec![];
    while let Some(node) = queue.pop_front() {
        if recipes.iter().any(|s| s == node) {
            res.push(node.to_string());
        }
        let Some(v) = adj.get(node) else {
            continue;
        };
        for next in v.iter() {
            *indegs.entry(next).or_insert(0) -= 1;
            if indegs[next] == 0 {
                queue.push_back(next);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
