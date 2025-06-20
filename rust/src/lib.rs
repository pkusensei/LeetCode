mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn special_grid(n: i32) -> Vec<Vec<i32>> {
    dfs(n)
}

fn dfs(n: i32) -> Vec<Vec<i32>> {
    if n == 0 {
        return vec![vec![0]];
    }
    let top_right = dfs(n - 1);
    let delta = 2_i32.pow(n as u32).pow(2) / 4;
    let bot_right = increment(&top_right, delta);
    let mut bot_left = increment(&bot_right, delta);
    // top_left
    let mut res = increment(&bot_left, delta);
    for (row, right) in res.iter_mut().zip(top_right) {
        row.extend(right);
    }
    for (row, right) in bot_left.iter_mut().zip(bot_right) {
        row.extend(right);
    }
    res.extend(bot_left);
    res
}

fn increment(grid: &[Vec<i32>], delta: i32) -> Vec<Vec<i32>> {
    let mut res = grid.to_vec();
    for v in res.iter_mut().flat_map(|row| row.iter_mut()) {
        *v += delta;
    }
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
            special_grid(2),
            [[15, 12, 3, 0], [14, 13, 2, 1], [11, 8, 7, 4], [10, 9, 6, 5]]
        );
    }

    #[test]
    fn test() {}
}
