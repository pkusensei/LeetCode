mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_subsequence(nums: &[i32]) -> i32 {
    let mut res = 0;
    for bit in 0..31 {
        let mut lis = vec![];
        for num in nums
            .iter()
            .filter_map(|v| if v & (1 << bit) > 0 { Some(*v) } else { None })
        {
            let i = lis.partition_point(|&v| v < num);
            if i >= lis.len() {
                lis.push(num);
            } else {
                lis[i] = num;
            }
        }
        res = res.max(lis.len())
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
    fn test() {
        assert_eq!(longest_subsequence(&[1, 1]), 1);
    }
}
