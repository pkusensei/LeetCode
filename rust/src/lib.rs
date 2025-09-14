mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_xor_subsequences(nums: &[i32]) -> i32 {
    const WIDTH: usize = 32;
    if nums.is_empty() {
        return 0;
    }
    // `basis` marks the highest bit reachable thru xor-ing `nums`.
    // `basis[bit]` records unique representative for this `bit`,
    // along with lesser bits, the complete "recipe" to reach `bit`.
    let mut basis = [0; WIDTH];
    for &(mut num) in nums.iter() {
        for bit in (0..WIDTH).rev() {
            if (num >> bit) & 1 == 1 {
                if basis[bit] == 0 {
                    basis[bit] = num;
                    break;
                }
                // Unset this `bit` so that from now on
                // `num` only represents lesser bits
                num ^= basis[bit];
            }
        }
    }
    let mut res = 0;
    // Once a high bit is set, it's never unset
    for &b in basis.iter().rev() {
        if b > 0 {
            res = res.max(res ^ b)
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
        assert_eq!(max_xor_subsequences(&[1, 2, 3]), 3);
        assert_eq!(max_xor_subsequences(&[5, 2]), 7);
    }

    #[test]
    fn test() {}
}
