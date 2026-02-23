mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_subseq_widths(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    nums.sort_unstable();
    let [mut as_max, mut as_min] = [0, 0];
    for (i, &num) in nums.iter().enumerate() {
        as_max = (as_max + i64::from(num) * mod_pow(2, i as _)) % M;
        as_min = (as_min + i64::from(num) * mod_pow(2, (n - i - 1) as _)) % M;
    }
    (as_max - as_min).rem_euclid(M) as i32
}

const M: i64 = 1_000_000_007;

const fn mod_pow(b: i64, p: i64) -> i64 {
    if p == 0 {
        return 1;
    }
    if p & 1 == 0 {
        mod_pow(b * b % M, p >> 1) % M
    } else {
        mod_pow(b * b % M, p >> 1) * b % M
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
