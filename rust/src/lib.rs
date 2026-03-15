mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_arithmetic(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut streak = 1;
    let mut prev_d = None;
    let mut prefix = vec![(streak, prev_d)];
    let mut res = 0;
    for w in nums.windows(2) {
        let d = w[1] - w[0];
        if prev_d.is_none_or(|v| v == d) {
            streak += 1;
        } else {
            streak = 2;
        }
        res = res.max((1 + streak).min(n as i32));
        prev_d = Some(d);
        prefix.push((streak, prev_d));
    }
    streak = 1;
    let mut suf_d = None;
    let mut suffix = vec![(streak, suf_d)];
    for w in nums.windows(2).rev() {
        let d = w[1] - w[0];
        if suf_d.is_none_or(|v| v == d) {
            streak += 1;
        } else {
            streak = 2;
        }
        res = res.max((1 + streak).min(n as i32));
        suf_d = Some(d);
        suffix.push((streak, suf_d));
    }
    suffix.reverse();
    for i in 1..n - 1 {
        let curr_d = nums[1 + i] - nums[i - 1];
        match (prefix[i - 1].1, suffix[1 + i].1) {
            (Some(d1), Some(d2)) if d1 == d2 && d1 + d2 == curr_d => {
                res = res.max(1 + prefix[i - 1].0 + suffix[1 + i].0)
            }
            (Some(d), _) if d * 2 == curr_d => res = res.max(2 + prefix[i - 1].0),
            (_, Some(d)) if d * 2 == curr_d => res = res.max(2 + suffix[1 + i].0),
            _ => (),
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
    fn basics() {
        assert_eq!(longest_arithmetic(&[9, 7, 5, 10, 1]), 5);
        assert_eq!(longest_arithmetic(&[1, 2, 6, 7]), 3);
    }

    #[test]
    fn test() {
        assert_eq!(
            longest_arithmetic(&[79734, 13414, 52866, 11223, 46264, 42963]),
            4
        );
    }
}
