mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn count_different_subsequence_gc_ds(nums: &[i32]) -> i32 {
    const fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 {
            b
        } else {
            gcd(b % a, a)
        }
    }

    let set: HashSet<_> = nums.iter().copied().collect();
    let max = nums.iter().copied().max().unwrap();
    let mut res = 0;
    // candidate gcd x
    for x in 1..=max {
        let mut curr = 0;
        // scan thru seq of multiples of x
        // test whether the seq's intersection with nums has gcd==x
        for num in (x..=max).step_by(x as usize) {
            if set.contains(&num) {
                curr = gcd(curr, num);
                if x == curr {
                    break;
                }
            }
        }
        res += i32::from(curr == x);
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
        assert_eq!(count_different_subsequence_gc_ds(&[6, 10, 3]), 5);
        assert_eq!(count_different_subsequence_gc_ds(&[5, 15, 40, 5, 6]), 7);
    }

    #[test]
    fn test() {}
}
