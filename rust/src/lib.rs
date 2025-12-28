mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let cols = grid[0].len();
    let mut curr_i = cols;
    let mut res = 0;
    for row in &grid {
        while curr_i > 0 && row[curr_i - 1] < 0 {
            curr_i -= 1;
        }
        res += cols - curr_i;
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
    fn test() {
        assert_eq!(
            most_booked(4, vec![[18, 19], [3, 12], [17, 19], [2, 13], [7, 10]]),
            0
        );
    }
}
