mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
    let [mut min, mut max] = [i32::MAX, i32::MIN];
    let [mut mini, mut maxi] = [0, 0];
    for (i, &num) in nums.iter().enumerate() {
        if num < min {
            min = num;
            mini = i;
        }
        if max < num {
            max = num;
            maxi = i;
        }
    }
    let n = nums.len();
    let [a, b] = [mini.min(maxi), mini.max(maxi)];
    (1 + b).min(n - a).min(1 + a + n - b) as i32
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
