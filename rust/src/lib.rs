mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn eliminate_maximum(dist: &[i32], speed: &[i32]) -> i32 {
    let mut times: Vec<_> = dist
        .iter()
        .zip(speed.iter())
        .map(|(&d, &s)| (f64::from(d) / f64::from(s)).ceil() as i32)
        .collect();
    times.sort_unstable();
    times
        .iter()
        .enumerate()
        .take_while(|&(i, &time)| time as usize > i)
        .count() as _
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
        assert_eq!(eliminate_maximum(&[1, 3, 4], &[1, 1, 1]), 3);
        assert_eq!(eliminate_maximum(&[1, 1, 2, 3], &[1, 1, 1, 1]), 1);
        assert_eq!(eliminate_maximum(&[3, 2, 4], &[5, 3, 2]), 1);
    }

    #[test]
    fn test() {}
}
