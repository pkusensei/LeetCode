mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let cols = grid[0].len();
        (0..cols)
            .map(|c| grid.iter().map(|r| width(r[c])).max().unwrap())
            .collect()
}

fn width(num: i32) -> i32 {
    match num.cmp(&0) {
        std::cmp::Ordering::Less => 2 + num.abs().ilog10() as i32,
        std::cmp::Ordering::Equal => 1,
        std::cmp::Ordering::Greater => 1 + num.abs().ilog10() as i32,
    }
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
