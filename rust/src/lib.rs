mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
    if a == e {
        let min = b.min(f);
        let max = b.max(f);
        return if c == a && (min..max).contains(&d) {
            2
        } else {
            1
        };
    }
    if b == f {
        let min = a.min(e);
        let max = a.max(e);
        return if d == b && (min..max).contains(&c) {
            2
        } else {
            1
        };
    }
    if c.abs_diff(e) == d.abs_diff(f) {
        if a.abs_diff(e) == b.abs_diff(f) {
            let dx1 = c - e;
            let dy1 = d - f;
            let dx2 = a - e;
            let dy2 = b - f;
            return if dx1 / dy1 == dx2 / dy2 && dx1 * dx2 > 0 && dx1.abs() > dx2.abs() {
                2
            } else {
                1
            };
        }
        return 1;
    }
    2
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
    fn test() {
        assert_eq!(min_moves_to_capture_the_queen(4, 5, 6, 4, 7, 5), 1);
        assert_eq!(min_moves_to_capture_the_queen(8, 4, 8, 8, 7, 7), 1);
    }
}
