mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_time_to_initial_state(word: &str, k: i32) -> i32 {
    let (s, n) = (word.as_bytes(), word.len());
    let k = k as usize;
    for idx in (k..n).step_by(k) {
        if s.starts_with(&s[idx..]) {
            return (idx / k) as i32;
        }
    }
    n.div_ceil(k) as i32
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
        assert_eq!(minimum_time_to_initial_state("abacaba", 3), 2);
        assert_eq!(minimum_time_to_initial_state("abacaba", 4), 1);
        assert_eq!(minimum_time_to_initial_state("abcbabcd", 2), 4);
    }

    #[test]
    fn test() {}
}
