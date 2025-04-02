mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_triplet_value(nums: &[i32]) -> i64 {
    let mut res = 0;
    for (i1, a) in nums.iter().enumerate() {
        for (i2, b) in nums.iter().enumerate().skip(1 + i1) {
            for c in nums.iter().skip(1 + i2) {
                res = res.max(i64::from(a - b) * i64::from(*c));
            }
        }
    }
    res
}

pub fn greedy(nums: &[i32]) -> i64 {
    let mut res = 0;
    let mut maxi = 0;
    let mut diff = 0;
    for &num in nums.iter() {
        let num = i64::from(num);
        res = res.max(diff * num); // diff * [k]
        diff = diff.max(maxi - num); // [i] - [j]
        maxi = maxi.max(num); // max of [i]
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
        assert_eq!(maximum_triplet_value(&[12, 6, 1, 2, 7]), 77);
        assert_eq!(maximum_triplet_value(&[1, 10, 3, 4, 19]), 133);
        assert_eq!(maximum_triplet_value(&[1, 2, 3]), 0);

        assert_eq!(greedy(&[12, 6, 1, 2, 7]), 77);
        assert_eq!(greedy(&[1, 10, 3, 4, 19]), 133);
        assert_eq!(greedy(&[1, 2, 3]), 0);
    }

    #[test]
    fn test() {}
}
