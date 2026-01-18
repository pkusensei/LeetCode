mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_capacity(costs: &[i32], capacity: &[i32], budget: i32) -> i32 {
    use itertools::izip;
    let mut res = 0;
    let mut arr = vec![];
    for (co, ca) in izip!(costs.iter(), capacity.iter())
        .filter_map(|(&co, &ca)| if co < budget { Some((co, ca)) } else { None })
    {
        arr.push((co, ca));
        res = res.max(ca)
    }
    arr.sort_unstable();
    let pref_max = arr.iter().fold(vec![], |mut acc, &(_, ca)| {
        acc.push(ca.max(*acc.last().unwrap_or(&0)));
        acc
    });
    for (right, &(cost, cap)) in arr.iter().enumerate().skip(1) {
        let left = arr[..right].partition_point(|v| v.0 + cost < budget);
        if left > 0 {
            res = res.max(cap + pref_max[left - 1]);
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
        assert_eq!(max_capacity(&[4, 8, 5, 3], &[1, 5, 2, 7], 8), 8);
        assert_eq!(max_capacity(&[3, 5, 7, 4], &[2, 4, 3, 6], 7), 6);
        assert_eq!(max_capacity(&[2, 2, 2], &[3, 5, 4], 5), 9);
    }

    #[test]
    fn test() {}
}
