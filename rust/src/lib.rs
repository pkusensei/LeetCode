mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
    let [a, b, c] = [length, width, height].map(i64::from);
    let bulky = [a, b, c].iter().any(|&v| v >= 10_000) || a * b * c >= 1_000_000_000;
    let heavy = mass >= 100;
    match [bulky, heavy] {
        [true, true] => "Both".into(),
        [true, false] => "Bulky".into(),
        [false, true] => "Heavy".into(),
        [false, false] => "Neither".into(),
    }
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
