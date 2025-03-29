mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
    if grid[0][0] != 0 {
        return false;
    }
    let mut nums = vec![];
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            nums.push((v, r, c));
        }
    }
    nums.sort_unstable_by_key(|v| v.0);
    nums.windows(2).all(|w| {
        let d1 = w[0].1.abs_diff(w[1].1);
        let d2 = w[0].2.abs_diff(w[1].2);
        w[0].0 + 1 == w[1].0 && d1 * d2 == 2
    })
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
