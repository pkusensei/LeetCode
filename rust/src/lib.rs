mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_time_to_initial_state(word: &str, k: i32) -> i32 {
    let n = word.len();
    let k = k as usize;
    let lps = kmp(word.as_bytes());
    let mut len = lps[n - 1];
    while len > 0 && (n - len) % k > 0 {
        len = lps[len - 1];
    }
    (n - len).div_ceil(k) as i32
}

fn kmp(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut lps = vec![0; n];
    let mut len = 0;
    for idx in 1..n {
        while len > 0 && s[idx] != s[len] {
            len = lps[len - 1];
        }
        if s[idx] == s[len] {
            len += 1;
        }
        lps[idx] = len;
    }
    lps
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
