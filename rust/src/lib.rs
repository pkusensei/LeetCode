mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_maximum_number(k: i64, x: i32) -> i64 {
    let mut left = 1;
    let mut right = 10i64.pow(15);
    while left < right {
        let mid = left + (right + 1 - left) / 2;
        if sum(x as u32, mid) > k {
            right = mid - 1;
        } else {
            left = mid;
        }
    }
    left
}

fn sum(x: u32, mid: i64) -> i64 {
    let mut total = 0;
    let mut bit = x - 1;
    while (1 << bit) <= mid {
        total += count(mid, bit);
        bit += x;
    }
    total
}

fn count(n: i64, bit: u32) -> i64 {
    let group_size = 1i64 << (bit + 1);
    let ones_in_full_group = 1i64 << bit;
    let full_groups = n / group_size;
    let remaining = n % group_size;
    let extra = (remaining - ones_in_full_group + 1)
        .max(0)
        .min(ones_in_full_group);
    full_groups * ones_in_full_group + extra
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
        assert_eq!(find_maximum_number(9, 1), 6);
        assert_eq!(find_maximum_number(7, 2), 9);
    }

    #[test]
    fn test() {}
}
