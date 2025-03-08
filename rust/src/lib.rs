mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_integer(num: i32) -> i32 {
    let s = num.to_string();
    let n = s.len();
    let [mut even_digits, mut odd_digits] = [0, 1].map(|_| vec![]);
    let [mut even_indices, mut odd_indices] = [0, 1].map(|_| vec![]);
    for (i, b) in s.bytes().enumerate() {
        if (b - b'0') & 1 == 1 {
            odd_digits.push(b);
            odd_indices.push(i);
        } else {
            even_digits.push(b);
            even_indices.push(i);
        }
    }
    odd_digits.sort_unstable_by(|a, b| b.cmp(a));
    even_digits.sort_unstable_by(|a, b| b.cmp(a));
    let mut res = vec![0; n];
    for (i, v) in odd_indices.into_iter().zip(odd_digits) {
        res[i] = v;
    }
    for (i, v) in even_indices.into_iter().zip(even_digits) {
        res[i] = v;
    }
    String::from_utf8(res).unwrap().parse().unwrap()
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
