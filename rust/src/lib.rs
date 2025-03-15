mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
    amount.sort_unstable();
    if amount[0] + amount[1] <= amount[2] {
        amount[2]
    } else {
        let sum: i32 = amount.iter().sum();
        sum / 2 + (sum & 1)
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
