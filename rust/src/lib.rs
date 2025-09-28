mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn split_array(nums: &[i32]) -> i64 {
    let n = nums.len();
    let [mut pref, mut suf] = [vec![false; n], vec![false; n]];
    pref[0] = true;
    for (i, w) in nums.windows(2).enumerate() {
        pref[1 + i] = pref[i] && (w[0] < w[1]);
        if !pref[1 + i] {
            break;
        }
    }
    suf[n - 1] = true;
    for (i, w) in nums.windows(2).enumerate().rev() {
        suf[i] = suf[1 + i] && (w[0] > w[1]);
        if !suf[i] {
            break;
        }
    }
    let sum = nums.iter().map(|&v| i64::from(v)).sum::<i64>();
    let mut left = 0;
    let mut res = i64::MAX;
    for (i, &num) in nums[..n - 1].iter().enumerate() {
        left += i64::from(num);
        if pref[i] && suf[1 + i] {
            res = res.min((sum - 2 * left).abs());
        }
    }
    if res == i64::MAX { -1 } else { res }
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
    fn basics() {
        assert_eq!(split_array(&[1, 3, 2]), 2);
        assert_eq!(split_array(&[1, 2, 4, 3]), 4);
        assert_eq!(split_array(&[3, 1, 2]), -1);
    }

    #[test]
    fn test() {}
}
