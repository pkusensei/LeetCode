mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(mut nums: Vec<i32>, x: i32) -> i64 {
    let n = nums.len();
    let x = i64::from(x);
    let mut res = i64::MAX;
    for k in 0..n {
        let curr: i64 = nums.iter().map(|&v| i64::from(v)).sum();
        res = res.min(curr + k as i64 * x);
        let mut temp = Vec::with_capacity(n);
        for i in 0..n {
            temp.push(nums[i].min(nums[(1 + i) % n]));
        }
        nums = temp;
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
        assert_eq!(min_cost(vec![20, 1, 15], 5), 13);
        assert_eq!(min_cost(vec![1, 2, 3], 4), 6);
    }

    #[test]
    fn test() {}
}
