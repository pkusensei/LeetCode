mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
    let [x, y, k] = [x, y, k].map(|v| v as usize);
    for c in y..y + k {
        let mut up = x;
        let mut down = x + k - 1;
        while up < down {
            (grid[up][c], grid[down][c]) = (grid[down][c], grid[up][c]);
            up += 1;
            down -= 1;
        }
    }
    grid
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
            reverse_submatrix(
                vec![
                    vec![1, 2, 3, 4],
                    vec![5, 6, 7, 8],
                    vec![9, 10, 11, 12],
                    vec![13, 14, 15, 16]
                ],
                1,
                0,
                3
            ),
            [
                [1, 2, 3, 4],
                [13, 14, 15, 8],
                [9, 10, 11, 12],
                [5, 6, 7, 16]
            ]
        );
        assert_eq!(
            reverse_submatrix(vec![vec![3, 4, 2, 3], vec![2, 3, 4, 2]], 0, 2, 2),
            [[3, 4, 4, 2], [2, 3, 2, 3]]
        );
    }

    #[test]
    fn test() {}
}
