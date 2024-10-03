mod helper;
mod trie;

use std::collections::{BTreeMap, HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn find_restaurant(list1: &[&str], list2: &[&str]) -> Vec<String> {
    let mut map: HashMap<&str, _> = HashSet::<&str>::intersection(
        &list1.iter().copied().collect(),
        &list2.iter().copied().collect(),
    )
    .map(|s| (*s, 0))
    .collect();
    for (i, s) in list1.iter().enumerate().chain(list2.iter().enumerate()) {
        if let Some(v) = map.get_mut(s) {
            *v += i;
        }
    }
    map.into_iter()
        .map(|(k, v)| (v, k))
        .fold(BTreeMap::<usize, Vec<_>>::new(), |mut acc, (c, s)| {
            acc.entry(c).or_default().push(s);
            acc
        })
        .pop_first()
        .map(|(_, v)| v.into_iter().map(|s| s.to_string()).collect())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            find_restaurant(
                &["Shogun", "Tapioca Express", "Burger King", "KFC"],
                &[
                    "Piatti",
                    "The Grill at Torrey Pines",
                    "Hungry Hunter Steakhouse",
                    "Shogun"
                ]
            ),
            ["Shogun"]
        );
        debug_assert_eq!(
            find_restaurant(
                &["Shogun", "Tapioca Express", "Burger King", "KFC"],
                &["KFC", "Shogun", "Burger King"]
            ),
            ["Shogun"]
        );
        sort_eq(
            find_restaurant(&["happy", "sad", "good"], &["sad", "happy", "good"]),
            ["sad", "happy"],
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
}
