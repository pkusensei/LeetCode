mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
    for r in 0..=1 {
        for c in 0..=1 {
            let curr = i32::from(grid[r][c] == 'B')
                + i32::from(grid[1 + r][c] == 'B')
                + i32::from(grid[r][1 + c] == 'B')
                + i32::from(grid[1 + r][1 + c] == 'B');
            if curr != 2 {
                return true;
            }
        }
    }
    false
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
