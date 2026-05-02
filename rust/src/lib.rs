mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn rotated_digits(n: i32) -> i32 {
    let s = n.to_string().into_bytes();
    let n = s.len();
    let mut memo = vec![[[-1; 2]; 2]; n];
    dfs(&s, 0, true, false, &mut memo)
}

fn dfs(s: &[u8], idx: usize, tight: bool, rotate: bool, memo: &mut [[[i32; 2]; 2]]) -> i32 {
    if idx >= s.len() {
        return i32::from(rotate);
    }
    if memo[idx][usize::from(tight)][usize::from(rotate)] > -1 {
        return memo[idx][usize::from(tight)][usize::from(rotate)];
    }
    let upper = if tight { s[idx] } else { b'9' };
    let mut res = 0;
    for b in b'0'..=upper {
        let ntight = tight && b == upper;
        if matches!(b, b'0' | b'1' | b'8') {
            res += dfs(s, 1 + idx, ntight, rotate, memo)
        } else if matches!(b, b'2' | b'5' | b'6' | b'9') {
            res += dfs(s, 1 + idx, ntight, true, memo)
        }
    }
    memo[idx][usize::from(tight)][usize::from(rotate)] = res;
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
