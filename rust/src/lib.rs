mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn furthest_distance_from_origin(moves: String) -> i32 {
    let mut ml = 0i32;
    let mut mr = 0;
    let mut un = 0;
    for b in moves.bytes() {
        match b {
            b'L' => ml += 1,
            b'R' => mr += 1,
            _ => un += 1,
        }
    }
    (ml - mr).abs() + un
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
