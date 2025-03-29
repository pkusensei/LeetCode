mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn dist_money(money: i32, children: i32) -> i32 {
    if money < children {
        return -1;
    }
    let count = (money - children) / 7;
    let rem = (money - children) % 7;
    if count > children {
        children - 1
    } else if count == children {
        if rem > 0 { count - 1 } else { count }
    } else if rem == 3 && count + 1 == children {
        count - 1
    } else {
        count
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
