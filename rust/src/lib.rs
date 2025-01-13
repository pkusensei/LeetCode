mod dsu;
mod helper;
mod trie;

use std::collections::{BTreeMap, BTreeSet, HashMap};

#[allow(unused_imports)]
use helper::*;

pub fn display_table(orders: &[[&str; 3]]) -> Vec<Vec<String>> {
    let mut table_food = BTreeMap::<u16, HashMap<_, _>>::new();
    let mut foods = BTreeSet::new();
    for ord in orders.iter() {
        *table_food
            .entry(ord[1].parse().unwrap_or(0))
            .or_default()
            .entry(ord[2])
            .or_insert(0) += 1;
        foods.insert(ord[2]);
    }
    let mut res = vec![];
    let mut row = vec!["Table".to_string()];
    row.extend(foods.iter().map(|s| s.to_string()));
    res.push(row);
    for (k, v) in table_food.into_iter() {
        row = vec![k.to_string()];
        for food in foods.iter() {
            let num = v.get(food).copied().unwrap_or(0).to_string();
            row.push(num);
        }
        res.push(row);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            display_table(&[
                ["David", "3", "Ceviche"],
                ["Corina", "10", "Beef Burrito"],
                ["David", "3", "Fried Chicken"],
                ["Carla", "5", "Water"],
                ["Carla", "5", "Ceviche"],
                ["Rous", "3", "Ceviche"]
            ]),
            [
                ["Table", "Beef Burrito", "Ceviche", "Fried Chicken", "Water"],
                ["3", "0", "2", "1", "0"],
                ["5", "0", "1", "0", "1"],
                ["10", "1", "0", "0", "0"]
            ]
        );
        assert_eq!(
            display_table(&[
                ["James", "12", "Fried Chicken"],
                ["Ratesh", "12", "Fried Chicken"],
                ["Amadeus", "12", "Fried Chicken"],
                ["Adam", "1", "Canadian Waffles"],
                ["Brianna", "1", "Canadian Waffles"]
            ]),
            [
                ["Table", "Canadian Waffles", "Fried Chicken"],
                ["1", "2", "0"],
                ["12", "0", "3"]
            ]
        );
        assert_eq!(
            display_table(&[
                ["Laura", "2", "Bean Burrito"],
                ["Jhon", "2", "Beef Burrito"],
                ["Melissa", "2", "Soda"]
            ]),
            [
                ["Table", "Bean Burrito", "Beef Burrito", "Soda"],
                ["2", "1", "1", "1"]
            ]
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
