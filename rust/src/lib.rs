mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_make_subsequence(s: &str, t: &str) -> bool {
    let [n1, n2] = [&s, &t].map(|v| v.len());
    if n1 > n2 {
        return false;
    }
    if n1 == 1 {
        return true;
    }
    let [s, t] = [&s, &t].map(|v| v.as_bytes());
    let mut i2 = 0;
    let mut prefix = vec![None; n1];
    for (i1, &b1) in s.iter().enumerate() {
        while t.get(i2).is_some_and(|&b2| b2 != b1) {
            i2 += 1;
        }
        if i2 >= n2 {
            break;
        }
        prefix[i1] = Some(i2);
        i2 += 1;
    }
    let mut suffix = vec![None; n1];
    let mut i2 = n2 - 1;
    'out: for (i1, &b1) in s.iter().enumerate().rev() {
        while t[i2] != b1 {
            let Some(v) = i2.checked_sub(1) else {
                break 'out;
            };
            i2 = v;
        }
        suffix[i1] = Some(i2);
        let Some(v) = i2.checked_sub(1) else {
            break 'out;
        };
        i2 = v;
    }
    for i in 0..n1 {
        if i == 0 {
            if suffix[1 + i].is_some_and(|v| v > 0) {
                return true;
            }
        } else if i == n1 - 1 {
            if prefix[i - 1].is_some_and(|v| v < n2 - 1) {
                return true;
            }
        } else {
            if prefix[i - 1]
                .zip(suffix[1 + i])
                .is_some_and(|(a, b)| 1 + a < b)
            {
                return true;
            }
        }
    }
    false
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
        assert!(can_make_subsequence("cat", "chat"));
        assert!(!can_make_subsequence("plane", "apple"));
    }

    #[test]
    fn test() {
        assert!(!can_make_subsequence("aab", "aba"));
        assert!(can_make_subsequence("zaaz", "zyazbz"));
        assert!(!can_make_subsequence("bbb", "yab"));
    }
}
