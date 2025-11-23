mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut rem1 = vec![];
    let mut rem2 = vec![];
    for &num in nums.iter() {
        sum += num;
        if num % 3 == 1 {
            rem1.push(num);
        } else if num % 3 == 2 {
            rem2.push(num);
        }
    }
    let mut res = 0;
    rem1.sort_unstable();
    rem2.sort_unstable();
    match sum % 3 {
        1 => {
            if rem1.len() > 0 {
                res = res.max(sum - rem1[0]);
            }
            if rem2.len() > 1 {
                res = res.max(sum - rem2[0] - rem2[1]);
            }
            res
        }
        2 => {
            if rem2.len() > 0 {
                res = res.max(sum - rem2[0]);
            }
            if rem1.len() > 1 {
                res = res.max(sum - rem1[0] - rem1[1]);
            }
            res
        }
        _ => sum,
    }
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
