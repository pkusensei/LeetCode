mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
    let mut prev = grid[0].clone();
    let n = prev.len();
    for (r, row) in grid.iter().enumerate().skip(1) {
        let mut curr = vec![i32::MAX; n];
        for (c, p) in prev.iter().enumerate() {
            for i in 0..n {
                let mc = move_cost[grid[r - 1][c] as usize][i];
                curr[i] = curr[i].min(p + mc + row[i]);
            }
        }
        prev = curr
    }
    prev.into_iter().min().unwrap_or(0)
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
        assert_eq!(
            min_path_cost(
                vec![vec![5, 3], vec![4, 0], vec![2, 1]],
                vec![
                    vec![9, 8],
                    vec![1, 5],
                    vec![10, 12],
                    vec![18, 6],
                    vec![2, 4],
                    vec![14, 3]
                ]
            ),
            17
        );
    }

    #[test]
    fn test() {}
}
