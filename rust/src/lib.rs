mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
    let n = items.len();
    let mut copies = vec![0; n];
    let mut min = i32::MAX >> 1;
    for buy in 0..n {
        min = min.min(items[buy][1]);
        for other in 0..n {
            if items[other][0] % items[buy][0] == 0 {
                copies[buy] += 1;
            }
        }
    }
    let mut memo = vec![[-1; 1501]; n];
    dfs(&items, &copies, min, 0, budget, &mut memo)
}

fn dfs(
    items: &[Vec<i32>],
    copies: &[i32],
    min: i32,
    idx: usize,
    budget: i32,
    memo: &mut [[i32; 1501]],
) -> i32 {
    if idx >= items.len() {
        return budget / min;
    }
    if memo[idx][budget as usize] > -1 {
        return memo[idx][budget as usize];
    }
    let mut res = dfs(items, copies, min, 1 + idx, budget, memo);
    if budget >= items[idx][1] {
        let curr = copies[idx] + dfs(items, copies, min, 1 + idx, budget - items[idx][1], memo);
        res = res.max(curr);
    }
    memo[idx][budget as usize] = res;
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
