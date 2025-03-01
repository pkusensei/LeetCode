mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
    asteroids.sort_unstable();
    let mut total = i64::from(mass);
    for a in asteroids.into_iter().map(i64::from) {
        if total >= a {
            total += a;
        } else {
            return false;
        }
    }
    true
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
    fn basics() {}

    #[test]
    fn test() {}
}
