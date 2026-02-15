mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn almost_palindromic(s: &str) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    if n < 2 {
        return n as i32;
    }
    let mut memo = vec![vec![[-1; 2]; n]; n];
    let res = (0..n).map(|i| f(s, i, i, 0, &mut memo)).max().unwrap_or(2);
    (0..n - 1)
        .filter_map(|i| {
            if s[i] == s[1 + i] {
                Some(f(s, i, 1 + i, 0, &mut memo))
            } else {
                None
            }
        })
        .max()
        .unwrap_or(2)
        .max(res)
}

fn f(
    s: &[u8],
    mut left: usize,
    mut right: usize,
    skipped: usize,
    memo: &mut [Vec<[i32; 2]>],
) -> i32 {
    let n = s.len();
    if memo[left][right][skipped] > -1 {
        return memo[left][right][skipped];
    }
    let mut res = (1 + right - left) as i32;
    while let Some(ll) = left.checked_sub(1)
        && 1 + right < n
        && s[ll] == s[1 + right]
    {
        res += 2;
        left -= 1;
        right += 1;
    }
    if skipped == 0 {
        if left > 0 {
            res = res.max(f(s, left - 1, right, 1, memo));
        }
        if right < n - 1 {
            res = res.max(f(s, left, 1 + right, 1, memo));
        }
    }
    memo[left][right][skipped] = res;
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
        assert_eq!(almost_palindromic("abca"), 4);
        assert_eq!(almost_palindromic("abba"), 4);
        assert_eq!(almost_palindromic("zzabba"), 5);
    }

    #[test]
    fn test() {
        assert_eq!(almost_palindromic("abc"), 2);
    }
}
