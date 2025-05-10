mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_cost(
    _m: i32,
    _n: i32,
    mut horizontal_cut: Vec<i32>,
    mut vertical_cut: Vec<i32>,
) -> i64 {
    horizontal_cut.sort_unstable();
    vertical_cut.sort_unstable();
    let mut h_count = 1;
    let mut v_count = 1;
    let mut res = 0;
    while let Some((&hor, &ver)) = horizontal_cut.last().zip(vertical_cut.last()) {
        if hor > ver {
            horizontal_cut.pop();
            res += i64::from(hor * v_count);
            h_count += 1;
        } else {
            vertical_cut.pop();
            res += i64::from(ver * h_count);
            v_count += 1;
        }
    }
    while let Some(h) = horizontal_cut.pop() {
        res += i64::from(h * v_count);
    }
    while let Some(v) = vertical_cut.pop() {
        res += i64::from(v * h_count);
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
    fn basics() {
        assert_eq!(minimum_cost(3, 2, vec![1, 3], vec![5]), 13);
        assert_eq!(minimum_cost(2, 2, vec![7], vec![4]), 15);
    }

    #[test]
    fn test() {}
}
