mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_number(num: String, change: Vec<i32>) -> String {
    let Some(left) = num
        .bytes()
        .position(|b| change[usize::from(b - b'0')] > (b - b'0') as i32)
    else {
        return num;
    };
    let right = num[left..]
        .bytes()
        .enumerate()
        .find_map(|(i, b)| {
            if change[usize::from(b - b'0')] < (b - b'0') as i32 {
                Some(i + left)
            } else {
                None
            }
        })
        .unwrap_or(num.len());
    let mut s = num.into_bytes();
    for v in s[left..right].iter_mut() {
        let temp = change[usize::from(*v - b'0')] as u8 + b'0';
        *v = temp;
    }
    String::from_utf8(s).unwrap()
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
        assert_eq!(
            maximum_number("132".into(), vec![9, 8, 5, 0, 3, 6, 4, 2, 6, 8]),
            "832"
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            maximum_number("214010".into(), vec![6, 7, 9, 7, 4, 0, 3, 4, 4, 7]),
            "974676"
        );
    }
}
