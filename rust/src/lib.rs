mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_time(time: &[i32], total_trips: i32) -> i64 {
    let trips = i64::from(total_trips);
    let mut left = 1;
    let mut right = i64::from(*time.iter().min().unwrap_or(&1)) * trips;
    while left < right {
        let mid = left + (right - left) / 2;
        if time.iter().map(|&v| mid / i64::from(v)).sum::<i64>() >= trips {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
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
        assert_eq!(minimum_time(&[1, 2, 3], 5), 3)
    }

    #[test]
    fn test() {}
}
