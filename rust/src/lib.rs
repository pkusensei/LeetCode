mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_increments(n: i32, cost: &[i32]) -> i32 {
    let n = n as usize;
    let mut vals = vec![0; n];
    let max = max_cost(n, cost, 0, 0, &mut vals);
    let mut res = 0;
    dfs(n, &vals, max, 0, &mut res);
    res
}

fn dfs(n: usize, vals: &[i32], max: i32, node: usize, res: &mut i32) -> i32 {
    if 2 * node + 1 >= n {
        return max - vals[node];
    }
    let a = dfs(n, vals, max, 2 * node + 1, res);
    let b = dfs(n, vals, max, 2 * node + 2, res);
    let big = a.max(b);
    let small = a.min(b);
    *res += big - small;
    small
}

fn max_cost(n: usize, cost: &[i32], node: usize, val: i32, vals: &mut [i32]) -> i32 {
    let curr = val + cost[node];
    if 2 * node + 1 >= n {
        vals[node] = curr; // leaf
        return curr;
    }
    let a = max_cost(n, cost, 2 * node + 1, curr, vals);
    let b = max_cost(n, cost, 2 * node + 2, curr, vals);
    a.max(b)
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
        assert_eq!(min_increments(7, &[1, 5, 2, 2, 3, 3, 1]), 6);
        assert_eq!(min_increments(3, &[5, 3, 3]), 0);
    }

    #[test]
    fn test() {}
}
