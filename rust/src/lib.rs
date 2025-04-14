mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_beautiful_substring(s: &str, k: i32) -> String {
    let s = s.as_bytes();
    let mut left = 0;
    let mut res = "".as_bytes();
    let mut count = 0;
    for (right, &b) in s.iter().enumerate() {
        count += i32::from(b == b'1');
        while count == k {
            let curr = &s[left..=right];
            if res.is_empty() || curr.len() < res.len() {
                res = curr;
            } else if curr.len() == res.len() {
                res = res.min(curr);
            }
            count -= i32::from(s[left] == b'1');
            left += 1;
        }
    }
    std::str::from_utf8(res).unwrap().to_string()
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
        assert_eq!(shortest_beautiful_substring("100011001", 3), "11001");
        assert_eq!(shortest_beautiful_substring("1011", 2), "11");
        assert_eq!(shortest_beautiful_substring("000", 1), "");
    }

    #[test]
    fn test() {}
}
