mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_cost(mut nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let (_, &mut med, _) = nums.select_nth_unstable(n / 2);
    let val1 = build(med, -1);
    let val2 = build(med, 1);
    cost(&nums, val1).min(cost(&nums, val2))
}

fn cost(nums: &[i32], med: i32) -> i64 {
    nums.iter()
        .map(|&v| (i64::from(v.abs_diff(med))).abs())
        .sum()
}

fn build(mut num: i32, delta: i32) -> i32 {
    while !is_palindrome(num.to_string().into_bytes()) {
        num += delta;
    }
    num
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
        assert_eq!(minimum_cost(vec![10, 12, 13, 14, 15]), 11);
        assert_eq!(minimum_cost(vec![1, 2, 3, 4, 5]), 6);
        assert_eq!(minimum_cost(vec![22, 33, 22, 33, 22]), 22);
    }

    #[test]
    fn test() {}
}
