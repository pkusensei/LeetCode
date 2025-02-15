mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn longest_common_subpath(n: i32, paths: &[&[i32]]) -> i32 {
    let n = i64::from(n);
    let upper = paths.iter().map(|p| p.len()).min().unwrap();
    let [mut left, mut right] = [0, upper];
    while left < right {
        let mid = left + (right - left) / 2;
        if rabin_karp(mid, n, paths) {
            left = 1 + mid;
        } else {
            right = mid;
        }
    }
    if left == upper && rabin_karp(left, n, paths) {
        left as i32
    } else {
        left as i32 - 1
    }
}

fn rabin_karp(len: usize, base: i64, paths: &[&[i32]]) -> bool {
    const MOD: i64 = 1_000_000_007;
    if len == 0 {
        return true;
    }
    let base_pow_len = (1..len).fold(1, |acc, _| (acc * base) % MOD);

    let mut map: HashMap<_, Vec<_>> = HashMap::new();
    for (pi, path) in paths.iter().enumerate() {
        let mut map2: HashMap<i64, Vec<_>> = HashMap::new();
        let mut hash = path[..len]
            .iter()
            .fold(0, |acc, &v| (acc * base + i64::from(v)) % MOD);
        for idx in 0..=(path.len() - len) {
            if let Some(prev_path) = pi.checked_sub(1) {
                if map.get(&hash).is_some_and(|prev_matches| {
                    prev_matches.iter().any(|&prev_i| {
                        &paths[prev_path][prev_i..(prev_i + len)] == &path[idx..(idx + len)]
                    })
                }) {
                    map2.entry(hash).or_default().push(idx);
                }
            } else {
                map2.entry(hash).or_default().push(idx);
            }
            if idx < path.len() - len {
                hash -= i64::from(path[idx]) * base_pow_len;
                hash *= base;
                hash += i64::from(path[idx + len]);
                hash = hash.rem_euclid(MOD);
            }
        }
        map = map2;
    }
    !map.is_empty()
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
        assert_eq!(
            longest_common_subpath(5, &[&[0, 1, 2, 3, 4], &[2, 3, 4], &[4, 0, 1, 2, 3]]),
            2
        );
        assert_eq!(longest_common_subpath(3, &[&[0], &[1], &[2]]), 0);
        assert_eq!(
            longest_common_subpath(5, &[&[0, 1, 2, 3, 4], &[4, 3, 2, 1, 0]]),
            1
        );
    }

    #[test]
    fn test() {}
}
