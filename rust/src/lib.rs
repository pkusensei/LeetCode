mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_removals(s: &str, p: &str, removable: &[i32]) -> i32 {
    let mut left = 0;
    let mut right = removable.len();
    while left < right {
        let mid = left + (right + 1 - left) / 2;
        let del: HashSet<_> = removable[..mid].iter().copied().collect();
        if is_subseq(s.as_bytes(), p.as_bytes(), &del) {
            left = mid
        } else {
            right = mid - 1;
        }
    }
    left as i32
}

fn is_subseq(hay: &[u8], needle: &[u8], del: &HashSet<i32>) -> bool {
    let (n1, n2) = (hay.len(), needle.len());
    let [mut i1, mut i2] = [0, 0];
    while i1 < n1 && i2 < n2 {
        if del.contains(&(i1 as i32)) {
            i1 += 1;
            continue;
        }
        if i1 < n1 && hay[i1] == needle[i2] {
            i2 += 1;
        }
        i1 += 1;
    }
    i2 == n2
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
        assert_eq!(maximum_removals("abcacb", "ab", &[3, 1, 0]), 2);
        assert_eq!(
            maximum_removals("abcbddddd", "abcd", &[3, 2, 1, 4, 5, 6]),
            1
        );
        assert_eq!(maximum_removals("abcab", "abc", &[0, 1, 2, 3, 4]), 0);
    }

    #[test]
    fn test() {}
}
