mod helper;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn uncommon_from_sentences(s1: &str, s2: &str) -> Vec<String> {
        s1.split_whitespace()
            .chain(s2.split_whitespace())
            .fold(HashMap::new(), |mut acc, s| {
                *acc.entry(s).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .filter_map(|(k, v)| if v == 1 { Some(k.to_string()) } else { None })
            .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(
            uncommon_from_sentences("this apple is sweet", "this apple is sour"),
            ["sweet", "sour"],
        );
        sort_eq(uncommon_from_sentences("apple apple", "banana"), ["banana"]);
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
