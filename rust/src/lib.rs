mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_good_integers_on_path(l: i64, r: i64, directions: &str) -> i64 {
    let mut path = [false; 16];
    let mut i = 0;
    path[i] = true;
    for d in directions.bytes() {
        i += if d == b'D' { 4 } else { 1 };
        path[i] = true;
    }
    f(r, &path) - f(l - 1, &path)
}

fn f(num: i64, path: &[bool]) -> i64 {
    let s = format!("{:016}", num);
    let mut memo = [[[-1; 10]; 2]; 16];
    dfs(s.as_bytes(), path, 0, true, b'0', &mut memo)
}

fn dfs(
    s: &[u8],
    path: &[bool],
    idx: usize,
    tight: bool,
    prev: u8,
    memo: &mut [[[i64; 10]; 2]; 16],
) -> i64 {
    if idx >= 16 {
        return 1;
    }
    if memo[idx][usize::from(tight)][usize::from(prev - b'0')] > -1 {
        return memo[idx][usize::from(tight)][usize::from(prev - b'0')];
    }
    let upper = if tight { s[idx] } else { b'9' };
    let mut res = 0;
    for digit in b'0'..=upper {
        let ntight = tight && digit == upper;
        if path[idx] {
            if digit >= prev {
                res += dfs(s, path, 1 + idx, ntight, digit, memo)
            }
        } else {
            res += dfs(s, path, 1 + idx, ntight, prev, memo)
        }
    }
    memo[idx][usize::from(tight)][usize::from(prev - b'0')] = res;
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
        assert_eq!(count_good_integers_on_path(8, 10, "DDDRRR"), 2);
        assert_eq!(
            count_good_integers_on_path(123456789, 123456790, "DDRRDR"),
            1
        );
        assert_eq!(
            count_good_integers_on_path(1288561398769758, 1288561398769758, "RRRDDD"),
            0
        );
    }

    #[test]
    fn test() {
        assert_eq!(count_good_integers_on_path(9, 789, "DRDRDR"), 430);
    }
}
