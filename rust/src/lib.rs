mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_cost(nums: &[i32], k: i32) -> i32 {
    const M: i128 = 1_000_000_007;
    let k = i128::from(k);
    let mut curr = k;
    let mut cost = 1;
    let mut res = 0;
    for &num in nums.iter() {
        // curr + d * k >= num
        // d*k >= num - curr
        // d = (num-curr+k-1)/k
        let num = i128::from(num);
        if curr >= num {
            curr -= num;
        } else {
            let d = (num - curr + k - 1) / k;
            let last = cost + d - 1;
            res = (res + (cost + last) * d / 2 % M) % M;
            cost = 1 + last;
            curr += d * k - num;
        }
    }
    res as i32
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
        assert_eq!(minimum_cost(&[1, 2, 3, 4], 4), 3);
    }

    #[test]
    fn test() {}
}
