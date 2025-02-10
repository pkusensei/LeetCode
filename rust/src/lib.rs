mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_speed_on_time(dist: &[i32], hour: f64) -> i32 {
    let mut left = 1;
    let mut right = 10_000_000;
    if count(dist, right) > hour {
        return -1;
    }
    while left < right {
        let mid = left + (right - left) / 2;
        if count(dist, mid) <= hour {
            right = mid;
        } else {
            left = 1 + mid;
        }
    }
    left
}

fn count(dist: &[i32], mid: i32) -> f64 {
    let n = dist.len();
    let mut res = 0.0;
    let v = f64::from(mid);
    for d in dist.iter().take(n - 1) {
        res += (f64::from(*d) / v).ceil();
    }
    res + f64::from(dist[n - 1]) / v
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
        assert_eq!(min_speed_on_time(&[1, 3, 2], 6.0), 1);
        assert_eq!(min_speed_on_time(&[1, 3, 2], 2.7), 3);
        assert_eq!(min_speed_on_time(&[1, 3, 2], 1.9), -1);
    }

    #[test]
    fn test() {}
}
