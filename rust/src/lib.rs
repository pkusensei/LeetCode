mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_digit_range(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut range = 0;
    for num in nums.iter() {
        let mut x = *num;
        let mut mind = 9;
        let mut maxd = 0;
        while x > 0 {
            let d = x % 10;
            x /= 10;
            mind = mind.min(d);
            maxd = maxd.max(d);
        }
        let curr = maxd - mind;
        if curr > range {
            range = curr;
            res = *num;
        } else if curr == range {
            res += num;
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
