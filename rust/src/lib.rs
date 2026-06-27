mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_vals_from_labels(
    values: &[i32],
    labels: &[i32],
    mut num_wanted: i32,
    use_limit: i32,
) -> i32 {
    use itertools::{Itertools, izip};
    let mut arr = izip!(values, labels).sorted_unstable().collect_vec();
    let mut freq = vec![0; 20_001];
    let mut res = 0;
    while num_wanted > 0 {
        let Some((val, &lab)) = arr.pop() else {
            break;
        };
        if freq[lab as usize] < use_limit {
            freq[lab as usize] += 1;
            res += val;
            num_wanted -= 1;
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
            largest_vals_from_labels(&[5, 4, 3, 2, 1], &[1, 1, 2, 2, 3], 3, 1),
            9
        );
    }

    #[test]
    fn test() {}
}
