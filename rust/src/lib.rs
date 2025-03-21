mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn odd_string(words: Vec<String>) -> String {
    use itertools::Itertools;
    use std::collections::HashMap;
    words
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let v = s
                .as_bytes()
                .windows(2)
                .map(|w| i32::from(w[1]) - i32::from(w[0]))
                .collect_vec();
            (v, i)
        })
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (v, i)| {
            acc.entry(v).or_default().push(i);
            acc
        })
        .into_values()
        .find_map(|v| {
            if v.len() == 1 {
                Some(words[v[0]].to_string())
            } else {
                None
            }
        })
        .unwrap_or_default()
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
