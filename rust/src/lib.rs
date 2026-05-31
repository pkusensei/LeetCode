mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_sale_items(items: Vec<[i32; 2]>, budget: i32) -> i32 {
    let n = items.len();
    let mut gains = vec![0; n];
    let mut min_price = i32::MAX;
    for i1 in 0..n {
        min_price = min_price.min(items[i1][1]);
        for i2 in 0..n {
            if items[i2][0] % items[i1][0] == 0 {
                gains[i1] += 1;
            }
        }
    }
    let mut memo = vec![[-1; 1501]; n];
    dfs(&items, &gains, min_price, 0, budget, &mut memo)
}

// [factor, price]
fn dfs(
    items: &[[i32; 2]],
    gains: &[i32],
    min_price: i32,
    idx: usize,
    budget: i32,
    memo: &mut [[i32; 1501]],
) -> i32 {
    if idx >= items.len() {
        return budget / min_price;
    }
    if memo[idx][budget as usize] != -1 {
        return memo[idx][budget as usize];
    }
    let skip = dfs(items, gains, min_price, 1 + idx, budget, memo);
    let mut take = 0;
    if budget >= items[idx][1] {
        take = gains[idx]
            + dfs(
                items,
                gains,
                min_price,
                1 + idx,
                budget - items[idx][1],
                memo,
            );
    }
    let res = skip.max(take);
    memo[idx][budget as usize] = res;
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
        assert_eq!(maximum_sale_items(vec![[6, 2], [2, 6], [3, 4]], 9), 4);
        assert_eq!(
            maximum_sale_items(vec![[2, 4], [3, 2], [4, 1], [6, 4], [12, 4]], 8),
            10
        )
    }

    #[test]
    fn test() {}
}
