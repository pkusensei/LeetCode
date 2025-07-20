mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn popcount_depth(n: i64, k: i32) -> i64 {
    let bit_width = 1 + n.ilog2() as usize;
    let mut memo = vec![[[-1; 2]; 64]; bit_width];
    dfs(n, k, bit_width - 1, 0, 1, &mut memo) - i64::from(k == 1)
}

fn dfs(n: i64, k: i32, idx: usize, ones: usize, tight: usize, memo: &mut [[[i64; 2]; 64]]) -> i64 {
    if memo[idx][ones][tight] > -1 {
        return memo[idx][ones][tight];
    }
    let max_bit = if tight == 1 { (n >> idx) & 1 } else { 1 };
    let mut res = 0;
    if idx == 0 {
        for bit in 0..=max_bit {
            let curr_ones = ones + usize::from(bit == 1);
            res += i64::from(DEPTH[curr_ones] == k);
        }
    } else {
        for bit in 0..=max_bit {
            let curr_ones = ones + usize::from(bit == 1);
            let ntight = usize::from(tight > 0 && bit == max_bit);
            res += dfs(n, k, idx - 1, curr_ones, ntight, memo)
        }
    }
    memo[idx][ones][tight] = res;
    res
}

const DEPTH: [i32; 64] = {
    let mut depth = [-1; 64];
    let mut x = 0;
    while x < 64 {
        precompute(x, &mut depth);
        x += 1;
    }
    depth
};

const fn precompute(x: usize, depth: &mut [i32; 64]) -> i32 {
    if x < 2 {
        depth[x] = x as i32;
    }
    if depth[x] > -1 {
        return depth[x];
    }
    depth[x] = 1 + precompute(x.count_ones() as _, depth);
    depth[x]
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
        assert_eq!(popcount_depth(4, 1), 2);
        assert_eq!(popcount_depth(7, 2), 3);
    }

    #[test]
    fn test() {}
}
