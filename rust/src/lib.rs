mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn lexicographically_smallest_string(s: &str) -> String {
    let n = s.len();
    let mut memo1 = vec![None; n];
    let mut memo2 = vec![vec![None; n]; n];
    let res = dfs(s.as_bytes(), 0, &mut memo1, &mut memo2);
    String::from_utf8(res).unwrap_or_default()
}

fn dfs(
    s: &[u8],
    idx: usize,
    memo1: &mut [Option<Vec<u8>>],
    memo2: &mut [Vec<Option<bool>>],
) -> Vec<u8> {
    let n = s.len();
    if idx >= n {
        return vec![];
    }
    if let Some(ref v) = memo1[idx] {
        return v.clone();
    }
    let mut res = vec![s[idx]];
    res.extend(dfs(s, 1 + idx, memo1, memo2));
    for right in (1 + idx..n).step_by(2) {
        if can_remove(s, idx, right, memo2) {
            res = res.min(dfs(s, 1 + right, memo1, memo2))
        }
    }
    memo1[idx] = Some(res.clone());
    res
}

fn can_remove(s: &[u8], left: usize, right: usize, memo: &mut [Vec<Option<bool>>]) -> bool {
    if left > right {
        return true;
    }
    if let Some(v) = memo[left][right] {
        return v;
    }
    if [1, 25].contains(&s[left].abs_diff(s[right])) && can_remove(s, 1 + left, right - 1, memo) {
        memo[left][right] = Some(true);
        return true;
    }
    for mid in (1 + left..right).step_by(2) {
        if can_remove(s, left, mid, memo) && can_remove(s, 1 + mid, right, memo) {
            memo[left][right] = Some(true);
            return true;
        }
    }
    memo[left][right] = Some(false);
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
        assert_eq!(lexicographically_smallest_string("abc"), "a");
        assert_eq!(lexicographically_smallest_string("bcda"), "");
        assert_eq!(lexicographically_smallest_string("zdce"), "zdce");
    }

    #[test]
    fn test() {}
}
