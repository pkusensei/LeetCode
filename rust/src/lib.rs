mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_good(nums: &[i32]) -> bool {
    let n = nums.len() - 1;
    if n == 0 {
        return false;
    }
    let mut freq = vec![0; 1 + n];
    for &num in nums.iter() {
        if num as usize > n {
            return false;
        }
        freq[num as usize] += 1;
    }
    freq[1..n].iter().all(|&v| v == 1) && freq[n] == 2
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
        assert!(is_good(&[1, 3, 3, 2]));
    }

    #[test]
    fn test() {}
}
