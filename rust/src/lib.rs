mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_missing_and_repeated_values(grid: &[&[i32]]) -> Vec<i32> {
    let n = grid.len() as i32;
    let xor = (1..=n.pow(2)).fold(0, |acc, v| acc ^ v);
    let mut grid_xor = 0;
    let mut set = std::collections::HashSet::new();
    let mut dup = 0;
    for r in grid.iter() {
        for &v in r.iter() {
            grid_xor ^= v;
            if !set.insert(v) {
                dup = v;
            }
        }
    }
    vec![dup, xor ^ grid_xor ^ dup]
}

pub fn with_math(grid: &[&[i32]]) -> Vec<i32> {
    let mut diff = 0;
    let mut sum_diff = 0;
    let mut sqr_diff = 0;
    for r in grid.iter() {
        for &v in r.iter() {
            diff += 1;
            sum_diff += v - diff;
            sqr_diff += v.pow(2) - diff.pow(2);
        }
    }
    let dup = (sqr_diff / sum_diff + sum_diff) / 2;
    let missing = (sqr_diff / sum_diff - sum_diff) / 2;
    vec![dup, missing]
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
            find_missing_and_repeated_values(&[&[1, 3], &[2, 2]]),
            [2, 4]
        );
        assert_eq!(
            find_missing_and_repeated_values(&[&[9, 1, 7], &[8, 9, 2], &[3, 4, 6]]),
            [9, 5]
        );

        assert_eq!(with_math(&[&[1, 3], &[2, 2]]), [2, 4]);
        assert_eq!(with_math(&[&[9, 1, 7], &[8, 9, 2], &[3, 4, 6]]), [9, 5]);
    }

    #[test]
    fn test() {}
}
