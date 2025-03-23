mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
    let cols = grid[0].len();
    for r in grid.iter_mut() {
        r.sort_unstable();
    }
    let mut res = 0;
    for c in 0..cols {
        res += grid.iter().map(|r| r[c]).max().unwrap_or(0);
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
