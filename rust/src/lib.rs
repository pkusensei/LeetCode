mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
    let [a, b, c] = [num1, num2, num3].map(f);
    let mut res = 0;
    for pow in 0..a.len().min(b.len()).min(c.len()) {
        let ad = *a.get(pow).unwrap_or(&9);
        let bd = *b.get(pow).unwrap_or(&9);
        let cd = *c.get(pow).unwrap_or(&9);
        res += ad.min(bd).min(cd) * 10i32.pow(pow as _);
    }
    res
}

fn f(mut num: i32) -> Vec<i32> {
    let mut ds = Vec::with_capacity(4);
    while num > 0 {
        ds.push(num % 10);
        num /= 10;
    }
    ds
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
