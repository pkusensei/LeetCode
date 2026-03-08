mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_balanced_index(nums: Vec<i32>) -> i32 {
    let sum = nums.iter().map(|&v| i64::from(v)).sum();
    let mut pref_sum: i64 = sum;
    let mut res = -1;
    let mut suf_prod = 1;
    for (i, &num) in nums.iter().enumerate().rev() {
        let num = i64::from(num);
        pref_sum -= num;
        if pref_sum == suf_prod {
            res = i as i32;
            break;
        }
        suf_prod *= num;
        if suf_prod > pref_sum {
            break;
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
    fn basics() {}

    #[test]
    fn test() {}
}
