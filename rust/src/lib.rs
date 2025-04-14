mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn construct_product_matrix(grid: &[&[i32]]) -> Vec<Vec<i32>> {
    const MOD: i64 = 12345;
    let [rows, cols] = get_dimensions(grid);
    let flat: Vec<_> = grid.iter().flat_map(|r| r.iter().copied()).collect();
    let f = |mut acc: Vec<i64>, &val| {
        acc.push(i64::from(val) * acc.last().unwrap_or(&1) % MOD);
        acc
    };
    let prefix = flat[..rows * cols - 1].iter().fold(vec![1_i64], f);
    let mut suffix = flat[1..].iter().rev().fold(vec![1_i64], f);
    suffix.reverse();
    let mut res = vec![vec![0; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            res[r][c] = (prefix[r * cols + c] * suffix[r * cols + c] % MOD) as i32;
        }
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
            construct_product_matrix(&[&[1, 2], &[3, 4]]),
            [[24, 12], [8, 6]]
        );
        assert_eq!(
            construct_product_matrix(&[&[12345], &[2], &[1]]),
            [[2], [0], [0]]
        );
    }

    #[test]
    fn test() {}
}
