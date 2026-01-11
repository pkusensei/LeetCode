mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_and(nums: &[i32], k: i32, m: i32) -> i32 {
    use itertools::Itertools;
    let mut res = 0;
    'out: for bit in (0..31).rev() {
        let candid = res | (1 << bit);
        let diff = nums
            .iter()
            .map(|&num| cost(num, candid))
            .k_smallest_relaxed(m as usize);
        let mut sum = 0;
        for d in diff {
            sum += d;
            if sum > k {
                continue 'out;
            }
        }
        res = candid;
    }
    res
}

fn cost(num: i32, candid: i32) -> i32 {
    if num & candid == candid {
        return 0;
    }
    let mut curr = num;
    for bit in (0..31).rev() {
        let candid_bit = candid & (1 << bit);
        let curr_bit = curr & (1 << bit);
        if candid_bit > 0 && curr_bit == 0 {
            let rem = curr % (1 << bit);
            let add = (1 << bit) - rem;
            curr += add;
        }
    }
    curr - num
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
        assert_eq!(maximum_and(&[3, 1, 2], 8, 2), 6);
        assert_eq!(maximum_and(&[1, 2, 8, 4], 7, 3), 4);
        assert_eq!(maximum_and(&[1, 1], 3, 2), 2);
    }

    #[test]
    fn test() {
        assert_eq!(maximum_and(&[8], 10, 1), 18);
        assert_eq!(maximum_and(&[5, 19], 1, 2), 4);
    }
}
