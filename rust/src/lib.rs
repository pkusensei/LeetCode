mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_buckets(hamsters: String) -> i32 {
    let mut s = hamsters.into_bytes();
    let n = s.len();
    let mut res = 0;
    let mut idx = 0;
    while idx < n {
        if s[idx] == b'H' {
            if idx > 0 && s[idx - 1] == b'B' {
                idx += 1; // look left first
                continue;
            }
            if s.get(1 + idx).is_some_and(|&v| v == b'.') {
                s[1 + idx] = b'B'; // put bucket
                idx += 2;
                res += 1;
                continue;
            }
            // All above failed, attemp left
            if idx > 0 && s[idx - 1] == b'.' {
                res += 1;
                s[idx - 1] = b'B';
                idx += 1;
                continue;
            }
            return -1;
        }
        idx += 1;
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
        assert_eq!(minimum_buckets("H..H".into()), 2);
        assert_eq!(minimum_buckets(".H.H.".into()), 1);
        assert_eq!(minimum_buckets(".HHH.".into()), -1);
    }

    #[test]
    fn test() {}
}
