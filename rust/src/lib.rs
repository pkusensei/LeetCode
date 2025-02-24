mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_valid_words(sentence: String) -> i32 {
    let mut res = 0;
    for s in sentence.split_ascii_whitespace() {
        if s.is_empty() {
            continue;
        }
        if s.bytes().any(|b| b.is_ascii_digit()) {
            continue;
        }
        if s.bytes().filter(|&b| b == b'-').count() > 1 {
            continue;
        }
        if let Some(i) = s.bytes().position(|b| b == b'-') {
            if i == 0 || i == s.len() - 1 {
                continue;
            }
            if !s.as_bytes()[i - 1].is_ascii_alphabetic()
                || !s.as_bytes()[i + 1].is_ascii_alphabetic()
            {
                continue;
            }
        }
        if s.bytes().filter(|b| b"!.,".contains(b)).count() > 1 {
            continue;
        }
        if s.contains(['!', '.', ',']) && !s.bytes().last().is_some_and(|b| b"!.,".contains(&b)) {
            continue;
        }
        res += 1;
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
    fn basics() {}

    #[test]
    fn test() {}
}
