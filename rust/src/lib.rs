mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn concat_hex36(n: i32) -> String {
    let mut a = convert(n.pow(2), 16);
    a.extend(convert(n.pow(3), 36));
    String::from_utf8(a).unwrap()
}

fn convert(mut num: i32, base: i32) -> Vec<u8> {
    let mut res = vec![];
    while num > 0 {
        let d = (num % base) as u8;
        num /= base;
        res.push(if d < 10 { b'0' + d } else { b'A' + d - 10 });
    }
    res.reverse();
    res
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
