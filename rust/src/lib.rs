mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn square_free_subsets(nums: &[i32]) -> i32 {
    use std::collections::HashMap;
    const MOD: i32 = 1_000_000_007;
    let mut dp = HashMap::from([(0, 0)]);
    for &num in nums.iter() {
        if let Some(mask) = to_mask(num) {
            let mut curr = dp.clone();
            let v = curr.entry(mask).or_insert(0);
            *v = (*v + 1) % MOD;
            for (&m, &c) in dp.iter() {
                if m & mask == 0 {
                    let v = curr.entry(m | mask).or_insert(0);
                    *v = (*v + c) % MOD;
                }
            }
            dp = curr;
        }
    }
    dp.values().fold(0, |acc, v| (acc + v) % MOD)
}

fn to_mask(num: i32) -> Option<i32> {
    const P: [i32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let mut mask = 0;
    let mut val = num;
    for p in P {
        while val > 0 && val % p == 0 {
            val /= p;
            if (mask >> p) & 1 == 1 {
                return None;
            }
            mask |= 1 << p
        }
    }
    Some(mask)
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
        assert_eq!(square_free_subsets(&[3, 4, 4, 5]), 3);
        assert_eq!(square_free_subsets(&[1]), 1);
    }

    #[test]
    fn test() {}
}
