mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn good_integers(l: i64, r: i64, k: i32) -> i64 {
    f(r, k as u8) - f(l - 1, k as u8)
}

fn f(num: i64, k: u8) -> i64 {
    let s = num.to_string().into_bytes();
    let n = s.len();
    let mut memo = vec![[[[-1; 10]; 2]; 2]; n];
    dfs(&s, k, 0, true, true, b'0', &mut memo)
}

fn dfs(
    s: &[u8],
    k: u8,
    idx: usize,
    tight: bool,
    leading: bool,
    prev: u8,
    memo: &mut [[[[i64; 10]; 2]; 2]],
) -> i64 {
    if idx >= s.len() {
        return (!leading).into();
    }
    if memo[idx][usize::from(tight)][usize::from(leading)][usize::from(prev - b'0')] > -1 {
        return memo[idx][usize::from(tight)][usize::from(leading)][usize::from(prev - b'0')];
    }
    let upper = if tight { s[idx] } else { b'9' };
    let mut res = 0;
    for d in b'0'..=upper {
        let ntight = tight && d == upper;
        // This slot is still zero
        let nleading = leading && d == b'0';
        // leading represents `prev > 0`
        // !leading means prev already started number
        if leading || d.abs_diff(prev) <= k {
            res += dfs(s, k, 1 + idx, ntight, nleading, d, memo);
        }
    }
    memo[idx][usize::from(tight)][usize::from(leading)][usize::from(prev - b'0')] = res;
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
        assert_eq!(good_integers(10, 15, 1), 3);
        assert_eq!(good_integers(201, 204, 2), 2);
    }

    #[test]
    fn test() {
        assert_eq!(good_integers(15, 147, 7), 126);
    }
}
