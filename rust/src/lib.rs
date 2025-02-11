mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_xor_sum(nums1: &[i32], nums2: &[i32]) -> i32 {
    let n = nums1.len();
    dfs(nums1, nums2, 0, 0, &mut vec![vec![-1; 1 << n]; n])
}

fn dfs(nums1: &[i32], nums2: &[i32], i1: usize, mask: usize, memo: &mut [Vec<i32>]) -> i32 {
    let n = nums1.len();
    if i1 >= n {
        if mask.count_ones() as usize == n {
            return 0;
        }
        return i32::MAX;
    }
    if memo[i1][mask] > -1 {
        return memo[i1][mask];
    }
    let mut res = i32::MAX;
    for i2 in 0..n {
        if (mask >> i2) & 1 == 1 {
            continue;
        }
        let v = dfs(nums1, nums2, 1 + i1, mask | (1 << i2), memo);
        if v < i32::MAX {
            res = res.min(v + (nums1[i1] ^ nums2[i2]))
        }
    }
    memo[i1][mask] = res;
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
        assert_eq!(minimum_xor_sum(&[1, 2], &[2, 3]), 2);
        assert_eq!(minimum_xor_sum(&[1, 0, 3], &[5, 4, 3]), 8);
    }

    #[test]
    fn test() {
        assert_eq!(
            minimum_xor_sum(&[72, 97, 8, 32, 15], &[63, 97, 57, 60, 83]),
            152
        );
    }
}
