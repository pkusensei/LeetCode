mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sub_str_hash(s: &str, power: i32, modulo: i32, k: i32, hash_value: i32) -> String {
    let (bytes, n) = (s.as_bytes(), s.len());
    let [p, m, hv] = [power, modulo, hash_value].map(i64::from);
    let k = k as usize;
    let mut start = 0;
    let mut curr = 0;
    let mut p_k = 1;
    for (idx, &b) in bytes.iter().enumerate().rev() {
        curr = (p * curr + i64::from(b - b'a' + 1)) % m;
        if idx + k >= n {
            // mod_pow(p, k, m) but here m is not prime
            p_k = p_k * p % m;
        } else {
            curr = (curr - i64::from(bytes[idx + k] - b'a' + 1) * p_k % m).rem_euclid(m);
        }
        if curr == hv {
            start = idx;
        }
    }
    s[start..start + k].to_string()
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
        assert_eq!(sub_str_hash("leetcode", 7, 20, 2, 0), "ee");
        assert_eq!(sub_str_hash("fbxzaad", 31, 100, 3, 32), "fbx");
    }

    #[test]
    fn test() {}
}
