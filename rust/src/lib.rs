mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::{BTreeMap, BTreeSet, HashMap};
#[derive(Default)]
struct FoodRatings {
    f_cr: HashMap<String, (String, i32)>,
    c_r_f: HashMap<String, BTreeMap<i32, BTreeSet<String>>>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut f_cr = HashMap::new();
        let mut c_r_f: HashMap<String, BTreeMap<i32, BTreeSet<String>>> = HashMap::new();
        for (f, (c, r)) in foods.into_iter().zip(cuisines.into_iter().zip(ratings)) {
            f_cr.insert(f.clone(), (c.clone(), r));
            c_r_f.entry(c).or_default().entry(r).or_default().insert(f);
        }
        Self { f_cr, c_r_f }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let (c, old_r) = self.f_cr.remove(&food).unwrap();
        let r_f = self.c_r_f.get_mut(&c).unwrap();
        let set = r_f.get_mut(&old_r).unwrap();
        set.remove(&food);
        if set.is_empty() {
            r_f.remove(&old_r);
        }
        self.f_cr.insert(food.clone(), (c.clone(), new_rating));
        self.c_r_f
            .entry(c)
            .or_default()
            .entry(new_rating)
            .or_default()
            .insert(food);
    }

    fn highest_rated(&self, cuisine: String) -> String {
        self.c_r_f
            .get(&cuisine)
            .and_then(|r_f| r_f.values().last())
            .and_then(|set| set.first())
            .map(|s| s.to_string())
            .unwrap_or_default()
    }
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
