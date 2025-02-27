mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
    let [map1, map2] = [&words1, &words2].map(|v| {
        v.iter().fold(HashMap::new(), |mut acc, s| {
            *acc.entry(s.as_str()).or_insert(0) += 1;
            acc
        })
    });
    map1.into_iter()
        .filter(|(s, c1)| *c1 == 1 && map2.get(s).is_some_and(|&c2| c2 == 1))
        .count() as i32
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
