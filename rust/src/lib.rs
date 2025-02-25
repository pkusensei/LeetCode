mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_vowel_substrings(word: &str) -> i32 {
    const VOWELS: &[u8] = b"aeiou";
    let n = word.len();
    if n < 5 {
        return 0;
    }
    let mut res = 0;
    for (left, b) in word[..n - 4].bytes().enumerate() {
        if !VOWELS.contains(&b) {
            continue;
        }
        let mut seen = std::collections::HashSet::new();
        let mut right = left;
        while word
            .as_bytes()
            .get(right)
            .is_some_and(|b| VOWELS.contains(b))
        {
            seen.insert(word.as_bytes()[right]);
            right += 1;
            if seen.len() >= 5 {
                res += 1;
            }
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
        assert_eq!(count_vowel_substrings("aeiouu"), 2);
        assert_eq!(count_vowel_substrings("unicornarihan"), 0);
        assert_eq!(count_vowel_substrings("cuaieuouac"), 7);
    }

    #[test]
    fn test() {}
}
