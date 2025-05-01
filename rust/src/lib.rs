mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost_to_equalize_array(nums: &[i32], cost1: i32, cost2: i32) -> i32 {
    const M: i64 = 1_000_000_007;
    let n = nums.len();
    let [cost1, cost2] = [cost1, cost2].map(i64::from);
    let min = nums.iter().map(|&v| i64::from(v)).min().unwrap_or(1);
    let max = nums.iter().map(|&v| i64::from(v)).max().unwrap_or(1);
    let sum = nums.iter().map(|&v| i64::from(v)).sum::<i64>();
    if cost2 >= 2 * cost1 || n < 3 {
        return (cost1 * (n as i64 * max - sum) % M) as i32;
    }
    let mut res = i64::MAX;
    for target in max..2 * max {
        let diff = target - min;
        let total_diff = target * n as i64 - sum;
        let pairs = (total_diff / 2).min(total_diff - diff);
        res = res.min(cost1 * (total_diff - 2 * pairs) + cost2 * pairs);
    }
    (res % M) as i32
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
        assert_eq!(min_cost_to_equalize_array(&[4, 1], 5, 2), 15);
        assert_eq!(min_cost_to_equalize_array(&[2, 3, 3, 3, 5], 2, 1), 6);
        assert_eq!(min_cost_to_equalize_array(&[3, 5, 3], 1, 3), 4);
    }

    #[test]
    fn test() {}
}
