mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub const fn distance_traveled(mut main_tank: i32, mut additional_tank: i32) -> i32 {
    let mut res = 0;
    while main_tank >= 5 {
        main_tank -= 5;
        res += 5;
        if additional_tank > 0 {
            main_tank += 1;
            additional_tank -= 1;
        }
    }
    res += main_tank;
    res * 10
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
