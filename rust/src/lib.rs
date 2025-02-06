mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_maximum_consecutive(coins: &mut [i32]) -> i32 {
    coins.sort_unstable();
    let mut curr = 0;
    for &num in coins.iter() {
        if num <= curr + 1 {
            curr = num.max(num + curr)
        } else {
            break;
        }
    }
    1 + curr
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
        assert_eq!(get_maximum_consecutive(&mut [1, 3]), 2);
        assert_eq!(get_maximum_consecutive(&mut [1, 1, 1, 4]), 8);
        assert_eq!(get_maximum_consecutive(&mut [1, 4, 10, 3, 1]), 20);
    }

    #[test]
    fn test() {}
}
