mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
    let mut left = 0.0;
    let mut right = f64::from(i32::MAX);
    while (right - left) > EP {
        let mid = (left + right) / 2.0;
        if diff(&squares, mid) > 0.0 {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}

const EP: f64 = 1e-5;

fn diff(squares: &[Vec<i32>], mid: f64) -> f64 {
    let [mut top, mut bot] = [0.0; 2];
    for sq in squares {
        let [y, side] = [1, 2].map(|i| f64::from(sq[i]));
        if mid <= y {
            top += side.powi(2);
        } else if mid >= y + side {
            bot += side.powi(2);
        } else {
            top += side * (y + side - mid);
            bot += side * (mid - y);
        }
    }
    top - bot
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
    fn basics() {
        float_eq!(separate_squares(vec![vec![0, 0, 1], vec![2, 2, 1]]), 1.0);
        float_eq!(
            separate_squares(vec![vec![0, 0, 2], vec![1, 1, 1]]),
            1.16667
        );
    }

    #[test]
    fn test() {}
}
