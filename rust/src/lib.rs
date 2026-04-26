mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_valid_elements(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut pref_max = nums.to_vec();
    for i in 1..n {
        pref_max[i] = pref_max[i].max(pref_max[i - 1])
    }
    let mut suf_max = nums.to_vec();
    for i in (0..n - 1).rev() {
        suf_max[i] = suf_max[i].max(suf_max[1 + i]);
    }
    let mut res = vec![];
    for (i, &num) in nums.iter().enumerate() {
        if i == 0 || i == n - 1 {
            res.push(num);
            continue;
        }
        if num > pref_max[i - 1] || num > suf_max[1 + i] {
            res.push(num);
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
        assert_eq!(find_valid_elements(&[2, 2, 1, 1, 2]), [2, 2]);
    }

    #[test]
    fn test() {}
}
