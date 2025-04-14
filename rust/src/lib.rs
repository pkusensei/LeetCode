mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum(nums: &[i32], k: i32) -> i32 {
    let mut freq = [0; 32];
    for &num in nums {
        for bit in 0..32 {
            if num & (1 << bit) > 0 {
                freq[bit] += 1;
            }
        }
    }
    let mut res = 0;
    for _ in 0..k {
        let mut curr = 0_i64;
        for (bit, v) in freq.iter_mut().enumerate() {
            if *v > 0 {
                *v -= 1;
                curr |= 1 << bit
            }
        }
        res += curr.pow(2);
        res %= 1_000_000_007;
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
        assert_eq!(max_sum(&[2, 6, 5, 8], 2), 261);
        assert_eq!(max_sum(&[4, 5, 4, 7], 3), 90);
    }

    #[test]
    fn test() {}
}
