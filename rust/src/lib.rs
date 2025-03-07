mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_ways(s: &str) -> i64 {
    let prefix = s.bytes().fold(vec![], |mut acc, b| {
        acc.push(i64::from(b - b'0') + acc.last().unwrap_or(&0));
        acc
    });
    let (s, n) = (s.as_bytes(), s.len());
    let mut res = 0;
    for i in 1..n - 1 {
        res += if s[i] == b'0' {
            let left = prefix[i - 1];
            let right = prefix[n - 1] - prefix[i];
            left * right
        } else {
            let left = i as i64 - prefix[i - 1];
            let right = (n - i - 1) as i64 - (prefix[n - 1] - prefix[i]);
            left * right
        }
    }
    res
}

pub fn one_pass(s: &str) -> i64 {
    let mut res = 0;
    let [mut one, mut zero, mut zero_one, mut one_zero] = [0; 4];
    for b in s.bytes() {
        if b == b'0' {
            zero += 1;
            one_zero += one;
            res += zero_one;
        } else {
            one += 1;
            zero_one += zero;
            res += one_zero;
        }
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
        assert_eq!(number_of_ways("001101"), 6);
        assert_eq!(number_of_ways("11100"), 0);

        assert_eq!(one_pass("001101"), 6);
        assert_eq!(one_pass("11100"), 0);
    }

    #[test]
    fn test() {}
}
