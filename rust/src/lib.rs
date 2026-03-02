mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut zeros = grid
        .iter()
        .map(|row| row.iter().rev().take_while(|&&v| v == 0).count())
        .collect_vec();
    let mut res = 0;
    for left in 0..n {
        let target = n - left - 1;
        if zeros[left] >= target {
            continue;
        }
        let Some(right) = zeros[1 + left..].iter().position(|&v| v >= target) else {
            return -1;
        };
        res += 1 + right;
        zeros[left..=(1 + left + right)].rotate_right(1);
    }
    res as i32
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
