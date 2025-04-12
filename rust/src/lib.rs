mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::izip;

pub fn max_number_of_alloys(
    _n: i32,
    _k: i32,
    budget: i32,
    composition: &[&[i32]],
    stock: &[i32],
    cost: &[i32],
) -> i32 {
    let mut res = 0;
    for machine in composition.iter() {
        let mut left = 0;
        let mut right = 10i32.pow(9);
        while left < right {
            let mid = left + (right + 1 - left) / 2;
            let total: i64 = izip!(machine.iter(), stock.iter(), cost.iter())
                .map(|(&comp, &st, &co)| {
                    (i64::from(comp) * i64::from(mid) - i64::from(st)).max(0) * i64::from(co)
                })
                .sum();
            if total <= i64::from(budget) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        res = res.max(left);
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
        assert_eq!(
            max_number_of_alloys(3, 2, 15, &[&[1, 1, 1], &[1, 1, 10]], &[0, 0, 0], &[1, 2, 3]),
            2
        );
        assert_eq!(
            max_number_of_alloys(
                3,
                2,
                15,
                &[&[1, 1, 1], &[1, 1, 10]],
                &[0, 0, 100],
                &[1, 2, 3]
            ),
            5
        );
        assert_eq!(
            max_number_of_alloys(2, 3, 10, &[&[2, 1], &[1, 2], &[1, 1]], &[1, 1], &[5, 5]),
            2
        );
    }

    #[test]
    fn test() {}
}
