mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn nth_smallest(mut n: i64, k: i32) -> i64 {
    let mut k = k as usize;
    let mut memo = vec![vec![-1; 1 + k]; 1 + N];
    let mut res = 0;
    // 1..=N to make dfs/memo easier
    for bit in (1..=N).rev() {
        if k == 0 {
            break;
        }
        let f = dfs(bit - 1, k, &mut memo);
        if f < n {
            n -= f;
            res |= 1 << bit;
            k -= 1;
        }
    }
    res >> 1 // offset 1..=N
}

const N: usize = 50;

fn dfs(idx: usize, k: usize, memo: &mut [Vec<i64>]) -> i64 {
    if k == 0 {
        return 1;
    }
    if idx == 0 {
        return 0;
    }
    if memo[idx][k] > -1 {
        return memo[idx][k];
    }
    memo[idx][k] = dfs(idx - 1, k, memo) + dfs(idx - 1, k - 1, memo);
    memo[idx][k]
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
        assert_eq!(nth_smallest(4, 2), 9);
        assert_eq!(nth_smallest(3, 1), 4);
    }

    #[test]
    fn test() {}
}
