mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_vowels(word: &str) -> i64 {
    const VOWELS: &[u8] = b"aeiou";
    let n = word.len();
    let mut res = 0;
    for (i, b) in word.bytes().enumerate() {
        if VOWELS.contains(&b) {
            // substrs ends on i
            let left = i as i64;
            // substrs starts on i
            let right = (n - i - 1) as i64;
            // left*right => all combos
            // and single length
            res += left + right + left * right + 1;
        }
    }
    res
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
        assert_eq!(count_vowels("aba"), 6);
        assert_eq!(count_vowels("abc"), 3);
        assert_eq!(count_vowels("ltcd"), 0);
    }

    #[test]
    fn test() {}
}
