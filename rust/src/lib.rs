mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
    let n = grid.len();
    if n == 1 {
        return 0;
    }
    let mut prev_pick = vec![0; 1 + n]; // paint prev col
    let mut prev_skip = vec![0; 1 + n]; // keep it white
    for col in 1..n {
        let mut curr_pick = vec![0; 1 + n];
        let mut curr_skip = vec![0; 1 + n];
        for row in 0..=n {
            let mut prev_white = 0;
            let mut curr_black = 0;
            for painted in 0..row {
                curr_black += i64::from(grid[painted][col]);
            }
            for black in 0..=n {
                if (1..=row).contains(&black) {
                    curr_black -= i64::from(grid[black - 1][col]);
                }
                if black > row {
                    prev_white += i64::from(grid[black - 1][col - 1]);
                }
                curr_pick[black] = curr_pick[black]
                    .max(curr_black + prev_pick[row])
                    .max(curr_black + prev_white + prev_skip[row]);
                curr_skip[black] = curr_skip[black]
                    .max(prev_white + prev_skip[row])
                    .max(prev_pick[row]);
            }
        }
        prev_pick = curr_pick;
        prev_skip = curr_skip;
    }
    prev_pick.into_iter().max().unwrap_or(0)
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
            maximum_score(vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 3, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![5, 0, 0, 3, 0],
                vec![0, 0, 0, 0, 2]
            ]),
            11
        );
        assert_eq!(
            maximum_score(vec![
                vec![10, 9, 0, 0, 15],
                vec![7, 1, 0, 8, 0],
                vec![5, 20, 0, 11, 0],
                vec![0, 0, 0, 1, 2],
                vec![8, 12, 1, 10, 3]
            ]),
            94
        );
    }

    #[test]
    fn test() {}
}
