mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn uniform_array(nums1: Vec<i32>) -> bool {
    let [mut min_odd, mut min_even] = [i32::MAX; 2];
    for &num in nums1.iter() {
        if num & 1 == 1 && min_odd > num {
            min_odd = num;
        }
        if num & 1 == 0 && min_even > num {
            min_even = num;
        }
    }
    let min = min_odd.min(min_even);
    let par = min & 1;
    for &num in nums1.iter() {
        if num & 1 == par {
            continue;
        }
        if par == 1 {
            // num is even
            if num < min_odd {
                return false;
            }
        } else {
            // num is odd
            if num - min_odd < 2 {
                return false;
            }
        }
    }
    true
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
