mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn alternating_subarray(nums: &[i32]) -> i32 {
    let mut res = 0;
    for (left, &v) in nums.iter().enumerate() {
        let mut curr = v;
        let mut d = 1;
        let mut right = 1 + left;
        while nums.get(right).is_some_and(|&v| v == curr + d) {
            curr += d;
            d *= -1;
            right += 1;
        }
        res = res.max(right - left);
    }
    if res >= 2 { res as i32 } else { -1 }
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
        assert_eq!(alternating_subarray(&[2, 3, 4, 3, 4]), 4);
        assert_eq!(alternating_subarray(&[3, 4, 5]), 2);
    }

    #[test]
    fn test() {}
}
