mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_difference(s: &str, k: i32) -> i32 {
    use itertools::Itertools;
    use std::collections::HashMap;
    const INF: i32 = i32::MAX >> 1;
    let k = k as usize;
    let mut res = i32::MIN;
    for perm in (b'0'..=b'4').permutations(2) {
        let [a, b] = perm[..] else { unreachable!() };
        let mut seen = HashMap::new();
        let [mut pref_a, mut pref_b] = [vec![0], vec![0]];
        let mut left = 0;
        for (right, byte) in s.bytes().enumerate() {
            pref_a.push(*pref_a.last().unwrap_or(&0));
            pref_b.push(*pref_b.last().unwrap_or(&0));
            if byte == a {
                pref_a[1 + right] += 1;
            }
            if byte == b {
                pref_b[1 + right] += 1;
            }
            while left + k - 1 <= right
                && pref_a[left] < pref_a[1 + right]
                && pref_b[left] < pref_b[1 + right]
            {
                let key = [pref_a[left] & 1, pref_b[left] & 1];
                let diff = pref_a[left] - pref_b[left];
                let v = seen.entry(key).or_insert(INF);
                *v = (*v).min(diff); // keep min_diff
                left += 1;
            }
            let key = [1 - (pref_a[1 + right] & 1), pref_b[1 + right] & 1];
            let diff = pref_a[1 + right] - pref_b[1 + right];
            res = res.max(diff - seen.get(&key).unwrap_or(&INF));
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
        assert_eq!(max_difference("12233", 4), -1);
        assert_eq!(max_difference("1122211", 3), 1);
        assert_eq!(max_difference("110", 3), -1);
    }

    #[test]
    fn test() {}
}
