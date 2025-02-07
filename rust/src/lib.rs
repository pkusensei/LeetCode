mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
    let [n, index, max_sum] = [n, index, max_sum].map(i64::from);
    let mut left = 1;
    let mut right = max_sum;
    while left < right {
        // The +1 here is to avoid infinite loops with left=mid;
        let mid = i64::from(1 + left + right) / 2;
        if min_sum(n, index, mid) > max_sum {
            right = mid - 1;
        } else {
            left = mid;
        }
    }
    left as _
}

const fn min_sum(n: i64, index: i64, num: i64) -> i64 {
    let left = if index < num {
        // 0 1 2
        // 2 3 4
        (num - index + num) * (1 + index) / 2
    } else {
        // 0 1 2 3
        // 1 1 1 2
        num * (num + 1) / 2 + (index - num + 1)
    };
    let len = n - index;
    let right = if len <= num {
        // 2 3 4 => 5-2
        // 3 2 1
        (num + num - len + 1) * len / 2
    } else {
        // 2 3 4
        // 2 1 1
        (num + 1) * (num) / 2 + len - num
    };
    left + right - num
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
        assert_eq!(max_value(4, 2, 6), 2);
        assert_eq!(max_value(6, 1, 10), 3);
    }

    #[test]
    fn test() {
        // 7 6 5 4 3
        assert_eq!(max_value(5, 0, 28), 7);
    }
}
