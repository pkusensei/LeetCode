mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_beautiful_string(s: &str, k: i32) -> String {
    let (s, n) = (s.as_bytes(), s.len());
    let k = k as u8 + b'a' - 1;
    let mut res = s.to_vec();
    for idx in (0..n).rev() {
        res[idx] += 1;
        while !check(&res, idx) {
            res[idx] += 1;
        }
        if res[idx] <= k {
            for i in 1 + idx..n {
                res[i] = b'a';
                while !check(&res, i) {
                    res[i] += 1;
                }
            }
            return String::from_utf8(res).unwrap();
        }
    }
    String::new()
}

fn check(v: &[u8], idx: usize) -> bool {
    idx.checked_sub(1).is_none_or(|i| v[i] != v[idx])
        && idx.checked_sub(2).is_none_or(|i| v[i] != v[idx])
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
        assert_eq!(smallest_beautiful_string("abcz", 26), "abda");
        assert_eq!(smallest_beautiful_string("dc", 4), "");
    }

    #[test]
    fn test() {}
}
