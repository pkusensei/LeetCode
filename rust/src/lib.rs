mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
    use std::collections::{HashMap, HashSet};
    let [s, sub] = [&s, &sub].map(|v| v.as_bytes());
    let map = mappings
        .iter()
        .fold(HashMap::<_, HashSet<_>>::new(), |mut acc, m| {
            let [a, b] = m[..] else { unreachable!() };
            acc.entry(a as u8).or_default().insert(b as u8);
            acc
        });
    let n = sub.len();
    for w in s.windows(n) {
        if sub
            .iter()
            .zip(w.iter())
            .filter(|&(a, b)| a != b)
            .all(|(a, b)| map.get(a).is_some_and(|set| set.contains(b)))
        {
            return true;
        }
    }
    false
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
