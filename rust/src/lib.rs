mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_subarrays(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut pos = [0; 30];
    let mut res = vec![0; n];
    for (idx, &num) in nums.iter().enumerate().rev() {
        res[idx] = 1;
        for bit in 0..30 {
            if (num >> bit) & 1 == 1 {
                pos[bit] = idx;
            }
            // rightmost bit that is set
            res[idx] = res[idx].max((pos[bit] + 1 - idx) as i32);
        }
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
        assert_eq!(smallest_subarrays(&[1, 0, 2, 1, 3]), [3, 3, 2, 2, 1]);
        assert_eq!(smallest_subarrays(&[1, 2]), [2, 1]);
    }

    #[test]
    fn test() {}
}
