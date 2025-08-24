mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_walls(robots: &[i32], distance: &[i32], mut walls: Vec<i32>) -> i32 {
    use itertools::{Itertools, izip};
    walls.sort_unstable();
    // [rob, dist]
    let robs = izip!(robots.iter(), distance.iter())
        .map(|(&r, &d)| [r, d])
        .sorted_unstable()
        .collect_vec();
    let n = robs.len();
    dfs(&robs, &walls, 0, 0, 0, &mut vec![[-1; 2]; n])
}

fn dfs(
    robs: &[[i32; 2]],
    walls: &[i32],
    idx: usize,
    prev_dir: usize,
    prev_end: i32,
    memo: &mut [[i32; 2]],
) -> i32 {
    let Some(&[pos, dist]) = robs.get(idx) else {
        return 0;
    };
    if memo[idx][prev_dir] > -1 {
        return memo[idx][prev_dir];
    }
    let left = (1 + prev_end).max(pos - dist);
    let i1 = walls.partition_point(|&v| v < left);
    let i2 = walls.partition_point(|&v| v <= pos);
    let v1 = (i2 - i1) as i32 + dfs(robs, walls, 1 + idx, 0, pos, memo);
    let right = robs
        .get(1 + idx)
        .map(|&v| (v[0] - 1).min(pos + dist))
        .unwrap_or(pos + dist);
    let i3 = walls.partition_point(|&v| v < pos);
    let i4 = walls.partition_point(|&v| v <= right);
    let v2 = (i4 - i3) as i32 + dfs(robs, walls, 1 + idx, 1, right, memo);
    let res = v1.max(v2);
    memo[idx][prev_dir] = res;
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
        assert_eq!(max_walls(&[4], &[3], vec![1, 10]), 1);
        assert_eq!(max_walls(&[10, 2], &[5, 1], vec![5, 2, 7]), 3);
        assert_eq!(max_walls(&[1, 2], &[100, 1], vec![10]), 0);
    }

    #[test]
    fn test() {}
}
