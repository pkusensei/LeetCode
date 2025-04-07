mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sort_vowels(s: String) -> String {
    use itertools::Itertools;
    const V: &[u8] = b"AEIOUaeiou";
    let mut s = s.into_bytes();
    let v = s
        .iter()
        .copied()
        .filter(|b| V.contains(b))
        .sorted_unstable()
        .collect_vec();
    let mut idx = 0;
    for b in s.iter_mut() {
        if V.contains(b) {
            *b = v[idx];
            idx += 1;
        }
    }
    String::from_utf8(s).unwrap()
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
