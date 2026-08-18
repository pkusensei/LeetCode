mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let freq = nums.iter().fold([0; 51], |mut acc, &v| {
        acc[v as usize] += 1;
        acc
    });
    if k == 1 {
        freq.iter()
            .rposition(|&v| v == 1)
            .map(|i| i as i32)
            .unwrap_or(-1)
    } else if k == n as i32 {
        freq.iter()
            .rposition(|&v| v > 0)
            .map(|i| i as i32)
            .unwrap_or(-1)
    } else {
        let a = nums[0];
        let b = nums[n - 1];
        match [freq[a as usize], freq[b as usize]] {
            [1, 1] => a.max(b),
            [1, _] => a,
            [_, 1] => b,
            _ => -1,
        }
    }
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
