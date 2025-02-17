mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_special_subsequences(nums: &[i32]) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let [mut zero, mut one, mut two] = [0; 3];
    for &num in nums.iter() {
        match num {
            0 => zero += zero + 1,  // seq ends on 0
            1 => one += zero + one, // seq ends on 1
            _ => two += one + two,  // seq ends on 2
        }
        zero %= MOD;
        one %= MOD;
        two %= MOD;
    }
    two as _
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
        assert_eq!(count_special_subsequences(&[0, 1, 2, 2]), 3);
        assert_eq!(count_special_subsequences(&[2, 2, 0, 0]), 0);
        assert_eq!(count_special_subsequences(&[0, 1, 2, 0, 1, 2]), 7);
    }

    #[test]
    fn test() {}
}
