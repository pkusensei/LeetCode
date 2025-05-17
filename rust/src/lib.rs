mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_damage(power: i32, damage: Vec<i32>, health: Vec<i32>) -> i64 {
    use itertools::Itertools;
    let n = damage.len();
    let mut res = 0;
    let mut days = 0;
    for i in (0..n).sorted_unstable_by(|&a, &b| {
        let [aa, bb] =
            [a, b].map(|i| f64::from(damage[i]) / f64::from((health[i] + power - 1) / power));
        bb.total_cmp(&aa)
    }) {
        days += i64::from((health[i] + power - 1) / power);
        res += days * i64::from(damage[i]);
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
    fn basics() {}

    #[test]
    fn test() {}
}
