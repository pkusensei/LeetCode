mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn good_triplets(nums1: &[i32], nums2: &[i32]) -> i64 {
    // with_btreeset(nums1, nums2)
    let n = nums1.len();
    let mut data = vec![0; n];
    for (i, &num) in nums1.iter().enumerate() {
        data[num as usize] = i;
    }
    let [mut v1, mut v2] = [0, 1].map(|_| vec![0; n]);
    for (i, &num) in nums2.iter().enumerate() {
        v1[i] = data[num as usize] as i32;
        v2[i] = n as i32 - 1 - v1[i];
    }
    let [mut suf_large, mut pre_small] = [0, 1].map(|_| vec![0; n]);
    count_smaller(&v2, &mut suf_large);
    v1.reverse();
    count_smaller(&v1, &mut pre_small);
    pre_small.reverse();
    (1..n - 1)
        .map(|i| i64::from(suf_large[i]) * i64::from(pre_small[i]))
        .sum()
}

fn count_smaller(nums: &[i32], res: &mut [i32]) {
    let n = nums.len();
    let mut indices: Vec<_> = (0..n).collect();
    merge_sort(nums, &mut indices, res, 0, n - 1);
}

fn merge_sort(nums: &[i32], indices: &mut [usize], res: &mut [i32], left: usize, right: usize) {
    if left >= right {
        return;
    }
    let mid = left + (right - left) / 2;
    merge_sort(nums, indices, res, left, mid);
    merge_sort(nums, indices, res, 1 + mid, right);
    merge(nums, indices, res, left, right, mid);
}

fn merge(
    nums: &[i32],
    indices: &mut [usize],
    res: &mut [i32],
    left: usize,
    right: usize,
    mid: usize,
) {
    let [mut i1, mut i2] = [left, 1 + mid];
    let mut count = 0;
    let mut temp = vec![];
    while i1 <= mid && i2 <= right {
        if nums[indices[i2]] < nums[indices[i1]] {
            count += 1;
            temp.push(indices[i2]);
            i2 += 1
        } else {
            temp.push(indices[i1]);
            res[indices[i1]] += count;
            i1 += 1;
        }
    }
    while i1 <= mid {
        temp.push(indices[i1]);
        res[indices[i1]] += count;
        i1 += 1;
    }
    while i2 <= right {
        temp.push(indices[i2]);
        i2 += 1;
    }
    for (i, &v) in temp.iter().enumerate() {
        indices[left + i] = v;
    }
}

// TLE's
pub fn with_btreeset(nums1: &[i32], nums2: &[i32]) -> i64 {
    use std::collections::BTreeSet;

    let n = nums1.len();
    let mut pos = vec![0; n];
    // num-idx in nums2
    for (idx, &num) in nums2.iter().enumerate() {
        pos[num as usize] = idx;
    }
    let mut pos_in_2 = BTreeSet::from([pos[nums1[0] as usize]]);
    let mut pre_1 = vec![0];
    for &num in nums1[1..].iter() {
        pos_in_2.insert(pos[num as usize]);
        pre_1.push(pos_in_2.range(..pos[num as usize]).count());
    }
    pos_in_2 = BTreeSet::from([pos[nums1[n - 1] as usize]]);
    let mut suf_1 = vec![0];
    for &num in nums1[..n - 1].iter().rev() {
        let i = pos_in_2.range(..pos[num as usize]).count();
        suf_1.push(pos_in_2.len() - i);
        pos_in_2.insert(pos[num as usize]);
    }
    suf_1.reverse();
    pre_1
        .into_iter()
        .zip(suf_1)
        .map(|(a, b)| (a * b) as i64)
        .sum()
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
        assert_eq!(good_triplets(&[2, 0, 1, 3], &[0, 1, 2, 3]), 1);
        assert_eq!(good_triplets(&[4, 0, 1, 3, 2], &[4, 1, 0, 2, 3]), 4);

        assert_eq!(with_btreeset(&[2, 0, 1, 3], &[0, 1, 2, 3]), 1);
        assert_eq!(with_btreeset(&[4, 0, 1, 3, 2], &[4, 1, 0, 2, 3]), 4);
    }

    #[test]
    fn test() {}
}
