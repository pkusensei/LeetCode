mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_absent(nums: &[i32]) -> i32 {
    let n = nums.len() as i32;
    let sum = nums.iter().sum::<i32>();
    let set: std::collections::HashSet<i32> = nums.iter().copied().collect();
    let mut res = ((sum + n) / n).max(1);
    while set.contains(&res) {
        res += 1;
    }
    return res;
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
        assert_eq!(smallest_absent(&[3, 5]), 6);
        assert_eq!(smallest_absent(&[-1, 1, 2]), 3);
        assert_eq!(smallest_absent(&[-1, 4]), 2);
    }

    #[test]
    fn test() {}
}
