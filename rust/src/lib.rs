mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_reachable(target_x: i32, target_y: i32) -> bool {
    // gcd(target_x as _, target_y as _).count_ones() == 1
    let [mut x, mut y] = [target_x, target_y];
    while x > 1 && y > 1 {
        if x < y {
            if y & 1 == 0 { y /= 2 } else { y -= x }
        } else {
            if x & 1 == 0 { x /= 2 } else { x -= y }
        }
    }
    x == 1 && y == 1
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
