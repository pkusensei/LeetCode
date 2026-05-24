mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let pos = nums.iter().position(|&v| v == 0).unwrap_or(0);
    let inc_sorted = (0..n).all(|i| nums[(pos + i) % n] == i as i32);
    let mut res = usize::MAX >> 1;
    if inc_sorted {
        let a = pos;
        let b = 2 + n - pos;
        res = res.min(a).min(b);
    }
    let dec_sorted = (0..n).all(|i| {
        let curr = nums[i] as usize;
        let next = nums[(1 + i) % n] as usize;
        (next - curr + n) % n == n - 1
    });
    if dec_sorted {
        let k = n - 1 - pos;
        let a = 1 + k;
        let b = 1 + n - k;
        res = res.min(a).min(b);
    }
    if res == usize::MAX >> 1 {
        -1
    } else {
        res as i32
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
