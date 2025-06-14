mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn beautiful_numbers(l: i32, r: i32) -> i32 {
    solve(r) - solve(l - 1)
}

fn solve(mut num: i32) -> i32 {
    let mut ds = vec![];
    while num > 0 {
        ds.push(num % 10);
        num /= 10;
    }
    ds.reverse();
    dfs(&ds, 0, true, 1, 0, &mut HashMap::new())
}

fn dfs(
    s: &[i32],
    idx: usize,
    tight: bool,
    prod: i32,
    sum: i32,
    memo: &mut HashMap<(usize, bool, i32, i32), i32>,
) -> i32 {
    if idx >= s.len() {
        return i32::from(sum > 0 && prod % sum == 0);
    }
    let k = (idx, tight, prod, sum);
    if let Some(&v) = memo.get(&k) {
        return v;
    }
    let upper = if tight { s[idx] } else { 9 };
    let mut res = 0;
    for d in 0..=upper {
        let ntight = tight && d == upper;
        let nsum = sum + d;
        let nprod = if nsum > 0 { prod * d } else { prod };
        res += dfs(s, 1 + idx, ntight, nprod, nsum, memo);
    }
    memo.insert(k, res);
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
        assert_eq!(beautiful_numbers(10, 20), 2);
        assert_eq!(beautiful_numbers(1, 15), 10);
    }

    #[test]
    fn test() {
        assert_eq!(beautiful_numbers(20, 100), 15);
    }
}
