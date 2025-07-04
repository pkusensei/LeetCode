mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_median_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> f64 {
    fn dfs(
        nums1: &[i32],
        nums2: &[i32],
        k: i32,
        [start1, end1]: [i32; 2],
        [start2, end2]: [i32; 2],
    ) -> f64 {
        if end1 < start1 {
            return f64::from(nums2[(k - start1) as usize]);
        }
        if end2 < start2 {
            return f64::from(nums1[(k - start2) as usize]);
        }
        let i1 = (start1 + end1) / 2;
        let i2 = (start2 + end2) / 2;
        if i1 + i2 < k {
            if nums1[i1 as usize] > nums2[i2 as usize] {
                dfs(nums1, nums2, k, [start1, end1], [1 + i2, end2])
            } else {
                dfs(nums1, nums2, k, [1 + i1, end1], [start2, end2])
            }
        } else {
            if nums1[i1 as usize] > nums2[i2 as usize] {
                dfs(nums1, nums2, k, [start1, i1 - 1], [start2, end2])
            } else {
                dfs(nums1, nums2, k, [start1, end1], [start2, i2 - 1])
            }
        }
    }

    let (n1, n2) = (nums1.len() as i32, nums2.len() as i32);
    let n = n1 + n2;
    if n & 1 == 1 {
        dfs(nums1, nums2, n / 2, [0, n1 - 1], [0, n2 - 1])
    } else {
        (dfs(nums1, nums2, n / 2, [0, n1 - 1], [0, n2 - 1])
            + dfs(nums1, nums2, n / 2 - 1, [0, n1 - 1], [0, n2 - 1]))
            / 2.0
    }
}

pub fn with_binary_search(nums1: &[i32], nums2: &[i32]) -> f64 {
    if nums1.len() > nums2.len() {
        return with_binary_search(nums2, nums1);
    }
    let (n1, n2) = (nums1.len(), nums2.len());
    let mut left = 0;
    let mut right = n1;
    while left <= right {
        let partition_a = left.midpoint(right);
        let partition_b = (1 + n1 + n2) / 2 - partition_a;
        let max_left_a = partition_a
            .checked_sub(1)
            .map(|i| nums1[i])
            .unwrap_or(i32::MIN);
        let min_right_a = *nums1.get(partition_a).unwrap_or(&i32::MAX);
        let max_left_b = partition_b
            .checked_sub(1)
            .map(|i| nums2[i])
            .unwrap_or(i32::MIN);
        let min_right_b = *nums2.get(partition_b).unwrap_or(&i32::MAX);
        if max_left_a <= min_right_b && max_left_b <= min_right_a {
            if (n1 + n2) & 1 == 0 {
                return f64::from(max_left_a.max(max_left_b) + min_right_a.min(min_right_b)) / 2.0;
            } else {
                return max_left_a.max(max_left_b).into();
            }
        } else if max_left_a > max_left_b {
            right = partition_a - 1;
        } else {
            left = 1 + partition_a;
        }
    }
    0.0
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
        assert_eq!(find_median_sorted_arrays(&[1, 3], &[2]), 2.0);
        assert_eq!(find_median_sorted_arrays(&[1, 2], &[3, 4]), 2.5);

        assert_eq!(with_binary_search(&[1, 3], &[2]), 2.0);
        assert_eq!(with_binary_search(&[1, 2], &[3, 4]), 2.5);
    }

    #[test]
    fn test() {}
}
