mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn cyclic_shift(
    n: i32,
    mut grid: Vec<Vec<i32>>,
    row_shift: &[i32],
    col_shift: &[i32],
) -> Vec<Vec<i32>> {
    let n = n as usize;
    for (r, &v) in row_shift.iter().enumerate() {
        let v = v as usize % n;
        grid[r].rotate_left(v);
    }
    for (c, &v) in col_shift.iter().enumerate() {
        let v = v as usize % n;
        let a: Vec<i32> = (0..n).map(|i| grid[i][c]).collect();
        for i in 0..n {
            grid[(i + n - v) % n][c] = a[i];
        }
    }
    grid
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
    fn basics() {
        assert_eq!(
            cyclic_shift(
                3,
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                &[1, 2, 0],
                &[2, 2, 1]
            ),
            [[7, 8, 5], [2, 3, 9], [6, 4, 1]]
        );
        assert_eq!(
            cyclic_shift(2, vec![vec![1, 2], vec![3, 4]], &[1, 0], &[0, 1]),
            [[2, 4], [3, 1]]
        )
    }

    #[test]
    fn test() {}
}
