mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn interval_intersection(first_list: &[[i32; 2]], second_list: &[[i32; 2]]) -> Vec<Vec<i32>> {
    let n1 = first_list.len();
    let n2 = second_list.len();
    let [mut i1, mut i2] = [0, 0];
    let mut res = vec![];
    while i1 < n1 && i2 < n2 {
        let a = first_list[i1][0].max(second_list[i2][0]);
        let b = first_list[i1][1].min(second_list[i2][1]);
        if a <= b {
            res.push(vec![a, b]);
        }
        if first_list[i1][1] <= second_list[i2][1] {
            i1 += 1
        } else {
            i2 += 1
        }
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
    fn basics() {
        assert_eq!(
            interval_intersection(
                &[[0, 2], [5, 10], [13, 23], [24, 25]],
                &[[1, 5], [8, 12], [15, 24], [25, 26]]
            ),
            [[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]]
        );
    }

    #[test]
    fn test() {}
}
