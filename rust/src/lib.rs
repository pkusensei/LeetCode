mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
    let [mut min1, mut min2] = [money; 2];
    for &num in prices.iter() {
        if num < min1 {
            min2 = min1;
            min1 = num
        } else if num < min2 {
            min2 = num;
        }
    }
    let res = money - min1 - min2;
    if res < 0 { money } else { res }
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
