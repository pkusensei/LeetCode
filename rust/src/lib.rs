mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut left = 0;
    let mut res = i64::MAX;
    for ch in nums.chunk_by(|a, b| a == b) {
        let len = ch.len();
        let right = left + len - 1;
        if (1..n - 1).contains(&left) || (1..n - 1).contains(&right) {
            let num = i64::from(ch[0]);
            if left == 0 {
                res = res.min(num * (n - right - 1) as i64)
            } else if right == n - 1 {
                res = res.min(num * left as i64);
            } else {
                res = res.min(num * (n - len) as i64);
            }
        }
        left += len;
    }
    if res == i64::MAX { 0 } else { res }
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
        let nums: Vec<i32> = vec![1, 2, 3];
        assert_eq!(min_cost(nums), 4);

        let nums: Vec<i32> = vec![3, 3, 3, 6, 3, 3];
        assert_eq!(min_cost(nums), 9);
    }

    #[test]
    fn test() {}
}
