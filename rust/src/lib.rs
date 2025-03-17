mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
    let cols = matrix[0].len();
    let grid: Vec<_> = matrix
        .iter()
        .map(|row| row.iter().fold(0, |acc, &v| (acc << 1) | v))
        .collect();
    let mut res = 0;
    for mask in 0_i32..(1 << (1 + cols)) - 1 {
        if mask.count_ones() == num_select as u32 {
            let curr = grid.iter().filter(|&&num| num & mask == num).count() as i32;
            res = res.max(curr)
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
    fn basics() {}

    #[test]
    fn test() {}
}
