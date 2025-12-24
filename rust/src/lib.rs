mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
    let mut sum: i32 = apple.iter().sum();
    capacity.sort_unstable();
    let mut res = 0;
    while sum > 0 {
        sum -= capacity.pop().unwrap_or_default();
        res += 1;
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
            max_two_events(vec![
                [28, 81, 48],
                [27, 90, 94],
                [97, 99, 79],
                [5, 35, 81],
                [65, 94, 84],
                [65, 83, 58],
                [94, 94, 31],
                [39, 52, 73]
            ]),
            173
        );
        assert_eq!(
            max_two_events(vec![
                [22, 44, 9],
                [93, 96, 48],
                [56, 90, 3],
                [80, 92, 45],
                [63, 73, 69],
                [73, 96, 33],
                [11, 23, 84],
                [59, 72, 29],
                [89, 100, 46]
            ]),
            153
        );
    }
}
