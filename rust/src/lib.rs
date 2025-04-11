mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_operations(num: String) -> i32 {
    let n = num.len();
    let mut zeros = 0;
    let mut res = n;
    for (i1, a) in num.bytes().enumerate() {
        if b"0257".contains(&a) {
            zeros += i32::from(a == b'0');
            for b in num.bytes().skip(1 + i1) {
                match a {
                    b'0' | b'5' if b == b'0' => res = res.min(n - i1 - 2),
                    b'2' | b'7' if b == b'5' => res = res.min(n - i1 - 2),
                    _ => (),
                }
            }
        }
    }
    if zeros == 1 {
        res = res.min(n-1);
    }
    res as i32
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
