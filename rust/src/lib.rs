mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_moves(balance: Vec<i32>) -> i64 {
    use itertools::Itertools;
    let n = balance.len();
    let mut sum = 0;
    let mut neg = None;
    for (i, &num) in balance.iter().enumerate() {
        sum += i64::from(num);
        if num < 0 {
            neg = Some(i)
        }
    }
    if sum < 0 {
        return -1;
    }
    let Some(neg) = neg else {
        return 0;
    };
    let mut val = balance[neg];
    let mut res = 0;
    for (dist, num) in balance
        .iter()
        .enumerate()
        .map(|(i, &num)| {
            let mut dist = i.abs_diff(neg);
            dist = dist.min(n - dist);
            (dist, num)
        })
        .sorted_unstable()
        .skip(1)
    {
        if val >= 0 {
            break;
        }
        let diff = val.abs().min(num);
        res += dist as i64 * i64::from(diff);
        val += diff;
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
    fn test() {}
}
