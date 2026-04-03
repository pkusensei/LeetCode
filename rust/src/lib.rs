mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, mut walls: Vec<i32>) -> i32 {
    use itertools::{Itertools, izip};
    let n = robots.len();
    // (pos, dist)
    let robs = izip!(robots.iter().copied(), distance.iter().copied())
        .sorted_unstable()
        .collect_vec();
    walls.sort_unstable();
    // dp[i][0] turn left
    // dp[i][1] turn right
    let mut prev_left = find(&walls, robs[0].0 - robs[0].1, robs[0].0);
    let mut prev_right = find(
        &walls,
        robs[0].0,
        (robs[0].0 + robs[0].1).min(robs.get(1).map(|r| r.0 - 1).unwrap_or(i32::MAX >> 1)),
    );
    for i in 1..n {
        let (pos, dist) = robs[i];
        let curr_left = (prev_left + find(&walls, (pos - dist).max(1 + robs[i - 1].0), pos)).max(
            prev_right
                + find(
                    &walls,
                    (pos - dist).max(robs[i - 1].0 + robs[i - 1].1 + 1),
                    pos,
                ),
        );
        let curr_right = (prev_left
            + find(
                &walls,
                pos,
                (pos + dist).min(robs.get(1 + i).map(|r| r.0 - 1).unwrap_or(i32::MAX >> 1)),
            ))
        .max(
            prev_right
                + find(
                    &walls,
                    pos,
                    (pos + dist).min(robs.get(1 + i).map(|r| r.0 - 1).unwrap_or(i32::MAX >> 1)),
                ),
        );
        (prev_left, prev_right) = (curr_left, curr_right);
    }
    prev_left.max(prev_right)
}

fn find(walls: &[i32], left: i32, right: i32) -> i32 {
    let a = walls.partition_point(|&v| v < left);
    let b = walls.partition_point(|&v| v <= right);
    (b - a) as i32
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
