mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_score(grid: &[&[i32]]) -> i64 {
    let n = grid.len();
    let mut prefix = vec![vec![0; n]; 1 + n];
    for c in 0..n {
        for r in 1..=n {
            prefix[r][c] = prefix[r - 1][c] + i64::from(grid[r - 1][c]);
        }
    }
    let mut memo = vec![vec![[-1; 2]; 1 + n]; n];
    dfs(&prefix, 0, 0, false, &mut memo)
    // let mut memo = vec![vec![vec![-1; 1 + n]; 1 + n]; n];
    // dfs1(&prefix, 0, 0, 0, &mut memo)
}

fn dfs(
    prefix: &[Vec<i64>],
    idx: usize,
    last_h: usize,
    taken: bool,
    memo: &mut [Vec<[i64; 2]>],
) -> i64 {
    let n = prefix.len() - 1;
    if idx >= n {
        return 0;
    }
    if memo[idx][last_h][usize::from(taken)] > -1 {
        return memo[idx][last_h][usize::from(taken)];
    }
    let mut res = 0;
    for h in 0..=n {
        if h <= last_h {
            let curr = prefix[last_h][idx] - prefix[h][idx];
            // Take [h..=last_h] points on current col
            res = res.max(curr + dfs(prefix, 1 + idx, h, true, memo));
            // Skip these points
            res = res.max(dfs(prefix, 1 + idx, h, false, memo));
        } else {
            // h > last_h
            // No point on current col is available
            if !taken {
                // These points are on prev col
                // They are only available if not taken previously
                let mut curr = 0;
                if idx > 0 {
                    curr = prefix[h][idx - 1] - prefix[last_h][idx - 1];
                }
                res = res.max(curr + dfs(prefix, 1 + idx, h, false, memo))
            } else {
                res = res.max(dfs(prefix, 1 + idx, h, false, memo))
            }
        }
    }
    memo[idx][last_h][usize::from(taken)] = res;
    res
}

fn dfs1(
    prefix: &[Vec<i64>],
    idx: usize,
    last1: usize,
    last2: usize,
    memo: &mut [Vec<Vec<i64>>],
) -> i64 {
    let n = prefix.len() - 1;
    if idx >= n {
        return 0;
    }
    if memo[idx][last1][last2] > -1 {
        return memo[idx][last1][last2];
    }
    let mut res = 0;
    for h in 0..=n {
        let mut curr = 0;
        if h < last1 {
            curr = prefix[last1][idx] - prefix[h][idx];
        } else if h > last1 && idx > 0 {
            let max_last = last1.max(last2);
            curr = (prefix[h][idx - 1] - prefix[max_last][idx - 1]).max(0);
        }
        res = res.max(curr + dfs1(prefix, 1 + idx, h, last1, memo))
    }
    memo[idx][last1][last2] = res;
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
        assert_eq!(
            maximum_score(&[
                &[0, 0, 0, 0, 0],
                &[0, 0, 3, 0, 0],
                &[0, 1, 0, 0, 0],
                &[5, 0, 0, 3, 0],
                &[0, 0, 0, 0, 2]
            ]),
            11
        );
        assert_eq!(
            maximum_score(&[
                &[10, 9, 0, 0, 15],
                &[7, 1, 0, 8, 0],
                &[5, 20, 0, 11, 0],
                &[0, 0, 0, 1, 2],
                &[8, 12, 1, 10, 3]
            ]),
            94
        );
    }

    #[test]
    fn test() {}
}
