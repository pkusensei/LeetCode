mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn kth_smallest_product(nums1: &[i32], nums2: &[i32], k: i64) -> i64 {
    let [mut a1, mut a2, mut b1, mut b2] = [0; 4].map(|_| vec![]);
    for &num in nums1.iter() {
        if num < 0 {
            a1.push(-i64::from(num));
        } else {
            a2.push(i64::from(num));
        }
    }
    for &num in nums2.iter() {
        if num < 0 {
            b1.push(-i64::from(num));
        } else {
            b2.push(i64::from(num));
        }
    }
    a1.reverse();
    b1.reverse();
    let neg_count = (a1.len() * b2.len() + b1.len() * a2.len()) as i64;
    let [sign, k] = if k > neg_count {
        [1, k - neg_count]
    } else {
        // Want negative number
        // switch (b1<0, b2>0)
        std::mem::swap(&mut b1, &mut b2);
        [-1, neg_count + 1 - k] // might overflow
    };
    let [mut left, mut right] = [0, 10i64.pow(10)];
    while left < right {
        let mid = left + (right - left) / 2;
        if k <= count(&a1, &b1, mid) + count(&a2, &b2, mid) {
            right = mid;
        } else {
            left = 1 + mid;
        }
    }
    sign * left
}

fn count(nums1: &[i64], nums2: &[i64], val: i64) -> i64 {
    let mut res = 0;
    let mut i2 = nums2.len();
    for &num1 in nums1 {
        while i2 > 0 && num1 * nums2[i2 - 1] > val {
            i2 -= 1;
        }
        res += i2;
    }
    res as _
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
        assert_eq!(kth_smallest_product(&[2, 5], &[3, 4], 2), 8);
        assert_eq!(kth_smallest_product(&[-4, -2, 0, 3], &[2, 4], 6), 0);
        assert_eq!(
            kth_smallest_product(&[-2, -1, 0, 1, 2], &[-3, -1, 2, 4, 5], 3),
            -6
        );
    }

    #[test]
    fn test() {}
}
