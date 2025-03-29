mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn even_odd_bit(mut n: i32) -> Vec<i32> {
    let [mut even, mut odd] = [0, 0];
    let mut flag = false;
    while n > 0 {
        if flag {
            odd += n & 1
        } else {
            even += n & 1
        }
        n >>= 1;
        flag = !flag;
    }
    vec![even, odd]
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
