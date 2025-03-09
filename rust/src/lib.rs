mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn digit_sum(s: String, k: i32) -> String {
    let k = k as usize;
    let mut s: Vec<_> = s.bytes().map(|b| u16::from(b - b'0')).collect();
    while s.len() > k {
        s = s
            .chunks(k)
            .flat_map(|w| {
                let mut sum: u16 = w.iter().sum();
                let mut digits = vec![];
                while sum > 0 {
                    digits.push(sum % 10);
                    sum /= 10;
                }
                if digits.is_empty() {
                    digits.push(0);
                }
                digits.into_iter().rev()
            })
            .collect();
    }
    s.into_iter().map(|v| char::from(v as u8 + b'0')).collect()
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
