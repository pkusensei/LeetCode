mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_changes(s: String, k: i32) -> i32 {
    let n = s.len();
    let k = k as usize;
    let mut facts = vec![vec![1]; 1 + n];
    for d in 2..n {
        for val in ((d + d)..=n).step_by(d) {
            facts[val].push(d);
        }
    }
    let mut memo = vec![vec![-1; 1 + n]; 1 + k];
    let mut min_costs = vec![vec![vec![-1; n]; 1 + n]; 1 + n];
    dfs(s.as_bytes(), k, &facts, n, &mut memo, &mut min_costs)
}

fn count(s: &[u8], left: usize, right: usize, d: usize, costs: &mut [Vec<Vec<i32>>]) -> i32 {
    if left >= right {
        return 0;
    }
    if costs[left][right][d] > -1 {
        return costs[left][right][d];
    }
    let mut res = count(s, left + d, right - d, d, costs);
    for i in 0..d {
        res += i32::from(s[left + i] != s[right + i - d]);
    }
    res
}

fn make_semi(
    s: &[u8],
    facts: &[Vec<usize>],
    left: usize,
    right: usize,
    min_costs: &mut [Vec<Vec<i32>>],
) -> i32 {
    let mut res = i32::MAX >> 1;
    for &d in &facts[right - left] {
        res = res.min(count(s, left, right, d, min_costs));
    }
    res
}

fn dfs(
    s: &[u8],
    k: usize,
    facts: &[Vec<usize>],
    end: usize,
    memo: &mut [Vec<i32>],
    min_costs: &mut [Vec<Vec<i32>>],
) -> i32 {
    if k == 1 {
        return make_semi(s, facts, 0, end, min_costs);
    }
    if memo[k][end] > -1 {
        return memo[k][end];
    }
    let mut res = i32::MAX >> 1;
    for left in 2 * (k - 1)..end - 1 {
        let mut curr = make_semi(s, facts, left, end, min_costs);
        curr += dfs(s, k - 1, facts, left, memo, min_costs);
        res = res.min(curr);
    }
    memo[k][end] = res;
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
