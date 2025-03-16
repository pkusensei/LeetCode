mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut res = 0;
    for row in grid.iter() {
        for c in 0..n {
            if row.iter().zip(0..n).all(|(&v1, r)| v1 == grid[r][c]) {
                res += 1;
            }
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
