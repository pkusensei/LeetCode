mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn equal_frequency(word: &str) -> bool {
    let mut it = word
        .bytes()
        .fold([0; 26], |mut acc, b| {
            acc[usize::from(b - b'a')] += 1;
            acc
        })
        .into_iter()
        .filter(|&v| v > 0)
        .counts()
        .into_iter();
    match [it.next(), it.next(), it.next()] {
        [Some((k1, v1)), Some((k2, v2)), None] => {
            let mut vals = [[k1, v1], [k2, v2]];
            vals.sort_unstable();
            let [[k1, v1], [k2, v2]] = vals;
            (k1 == 1 && v1 == 1) || (k1 + 1 == k2 && v2 == 1)
        }
        [Some((k, v)), None, None] => k == 1 || v == 1,
        _ => false,
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
    fn basics() {
        assert!(equal_frequency("abcc"));
    }

    #[test]
    fn test() {
        assert!(equal_frequency("abbcc"));
        assert!(equal_frequency("zz"));
    }
}
