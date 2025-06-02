mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn candy(ratings: &[i32]) -> i32 {
    let n = ratings.len();
    let mut arr = vec![1; n];
    arr[0] = 1;
    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            arr[i] = 1 + arr[i - 1]
        }
    }
    let mut res = arr[n - 1];
    for i in (0..n - 1).rev() {
        if ratings[i] > ratings[1 + i] {
            arr[i] = arr[i].max(1 + arr[1 + i]);
        } else {
            arr[i] = arr[i].max(1);
        }
        res += arr[i];
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
        assert_eq!(candy(&[1, 2, 2]), 4);
        assert_eq!(candy(&[1, 0, 2]), 5);
    }

    #[test]
    fn test() {
        assert_eq!(candy(&[1, 3, 4, 5, 2]), 11);
        assert_eq!(candy(&[1, 2, 87, 87, 87, 2, 1]), 13);
    }
}
