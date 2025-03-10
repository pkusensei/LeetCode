mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_of_substrings(word: &str, k: i32) -> i64 {
    fn at_least_k(s: &[u8], k: i32) -> i64 {
        let n = s.len();
        let mut res = 0;
        let mut left = 0;
        let mut vows = [0; 5];
        let mut cons = 0;
        for (right, &b) in s.iter().enumerate() {
            match b {
                b'a' => vows[0] += 1,
                b'e' => vows[1] += 1,
                b'i' => vows[2] += 1,
                b'o' => vows[3] += 1,
                b'u' => vows[4] += 1,
                _ => cons += 1,
            }
            while vows.iter().all(|&v| v > 0) && cons >= k {
                res += n - right;
                match s[left] {
                    b'a' => vows[0] -= 1,
                    b'e' => vows[1] -= 1,
                    b'i' => vows[2] -= 1,
                    b'o' => vows[3] -= 1,
                    b'u' => vows[4] -= 1,
                    _ => cons -= 1,
                }
                left += 1;
            }
        }
        res as _
    }

    at_least_k(word.as_bytes(), k) - at_least_k(word.as_bytes(), 1 + k)
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
        assert_eq!(count_of_substrings("aeioqq", 1), 0);
        assert_eq!(count_of_substrings("aeiou", 0), 1);
        assert_eq!(count_of_substrings("ieaouqqieaouqq", 1), 3);
    }

    #[test]
    fn test() {
        assert_eq!(count_of_substrings("iqeaouqi", 2), 3);
        assert_eq!(count_of_substrings("aadieuoh", 1), 2);
    }
}
