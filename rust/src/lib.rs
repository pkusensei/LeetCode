mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    use std::collections::HashMap;
    if numerator == 0 {
        return "0".into();
    }
    let [mut num, den] = [numerator, denominator].map(|v| i64::from(v).abs());
    let mut res = String::new();
    if (numerator < 0) ^ (denominator < 0) {
        res.push('-');
    }
    res.push_str(&format!("{}", num / den));
    num %= den;
    if num == 0 {
        return res;
    }
    res.push('.');
    let mut seen = HashMap::new();
    while num > 0 {
        if let Some(&prev) = seen.get(&num) {
            res.insert(prev, '(');
            res.push(')');
            return res;
        }
        seen.insert(num, res.len());
        num *= 10;
        res.push_str(&format!("{}", num / den));
        num %= den;
    }
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
    fn basics() {
        assert_eq!(fraction_to_decimal(1, 2), "0.5");
    }

    #[test]
    fn test() {}
}
