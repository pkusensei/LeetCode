mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
    let s = n.to_string().into_bytes();
    let len = s.len();
    let mut memo = vec![[[[-1; 1 << 10]; 2]; 2]; len];
    let v = dfs(&s, 0, true, true, 0, &mut memo);
    n - v
}

fn dfs(
    s: &[u8],
    idx: usize,
    tight: bool,
    leading: bool,
    mask: usize,
    memo: &mut [[[[i32; 1 << 10]; 2]; 2]],
) -> i32 {
    if idx >= s.len() {
        return i32::from(!leading);
    }
    if memo[idx][usize::from(tight)][usize::from(leading)][mask] > -1 {
        return memo[idx][usize::from(tight)][usize::from(leading)][mask];
    }
    let upper = if tight { s[idx] } else { b'9' };
    let mut res = 0;
    for d in b'0'..=upper {
        if (mask >> (d - b'0')) & 1 == 1 {
            continue;
        }
        let ntight = tight && d == upper;
        let nleading = leading && d == b'0';
        let nmask = if nleading { 0 } else { mask | 1 << (d - b'0') };
        res += dfs(s, 1 + idx, ntight, nleading, nmask, memo);
    }
    memo[idx][usize::from(tight)][usize::from(leading)][mask] = res;
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
    fn basics() {}

    #[test]
    fn test() {}
}
