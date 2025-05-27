mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_k_reducible_numbers(s: &str, k: i32) -> i32 {
    let n = s.len();
    let mut k_vals = vec![0; 1 + n];
    for i in 1..=n {
        reduce(i, &mut k_vals);
    }
    let mut memo = vec![vec![vec![-1; 1 + n]; 2]; n];
    dfs(s.as_bytes(), k, &k_vals, 0, true, 0, &mut memo)
}

const M: i32 = 1_000_000_007;

fn dfs(
    s: &[u8],
    k: i32,
    k_vals: &[i32],
    idx: usize,
    tight: bool,
    set_bits: usize,
    memo: &mut [Vec<Vec<i32>>],
) -> i32 {
    if idx >= s.len() {
        return i32::from(!tight && set_bits > 0 && k_vals[set_bits] < k);
    }
    if memo[idx][usize::from(tight)][set_bits] > -1 {
        return memo[idx][usize::from(tight)][set_bits];
    }
    let upper = if tight { s[idx] } else { b'1' };
    let mut res = 0;
    for d in b'0'..=upper {
        let next_tight = tight && d == upper;
        res += dfs(
            s,
            k,
            k_vals,
            1 + idx,
            next_tight,
            set_bits + usize::from(d == b'1'),
            memo,
        );
        res %= M;
    }
    memo[idx][usize::from(tight)][set_bits] = res;
    res
}

fn reduce(n: usize, k_vals: &mut [i32]) -> i32 {
    if n <= 1 {
        return 0;
    }
    if k_vals[n] > 0 {
        return k_vals[n];
    }
    k_vals[n] = 1 + reduce(n.count_ones() as usize, k_vals);
    k_vals[n]
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
        assert_eq!(count_k_reducible_numbers("111", 1), 3);
        assert_eq!(count_k_reducible_numbers("1000", 2), 6);
        assert_eq!(count_k_reducible_numbers("1", 3), 0);
    }

    #[test]
    fn test() {}
}
