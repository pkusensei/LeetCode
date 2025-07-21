mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let k = k as usize;
    let mut memo = vec![vec![[-1; 2]; 1 + k]; n];
    dfs(&prices, 0, k, 1, &mut memo)
}

fn dfs(prices: &[i32], idx: usize, k: usize, buy: usize, memo: &mut [Vec<[i32; 2]>]) -> i32 {
    if k == 0 || idx >= prices.len() {
        return 0;
    }
    if memo[idx][k][buy] > -1 {
        return memo[idx][k][buy];
    }
    memo[idx][k][buy] = if buy == 1 {
        (-prices[idx] + dfs(prices, 1 + idx, k, 0, memo)).max(dfs(prices, 1 + idx, k, buy, memo))
    } else {
        (prices[idx] + dfs(prices, 1 + idx, k - 1, 1, memo)).max(dfs(prices, 1 + idx, k, buy, memo))
    };
    memo[idx][k][buy]
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
