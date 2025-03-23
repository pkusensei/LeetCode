mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_value(n: i32) -> i32 {
    let mut n = n;
    while let Some(v) = sum_prime(n) {
        if n == v {
            break;
        }
        n = v;
    }
    n
}

fn sum_prime(n: i32) -> Option<i32> {
    if n < 4 {
        return None;
    }
    let mut res = 0;
    let mut num = n;
    for p in 2..n {
        while num % p == 0 {
            res += p;
            num /= p;
        }
    }
    if res > 0 { Some(res) } else { None }
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
        assert_eq!(smallest_value(15), 5);
    }

    #[test]
    fn test() {
        assert_eq!(smallest_value(4), 4);
    }
}
