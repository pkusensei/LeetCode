mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_if_pangram(sentence: String) -> bool {
    sentence
        .bytes()
        .fold([false; 26], |mut acc, b| {
            acc[usize::from(b - b'a')] = true;
            acc
        })
        .into_iter()
        .all(|b| b)
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
