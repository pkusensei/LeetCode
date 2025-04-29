mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_latest_time(s: String) -> String {
    let m = match &s.as_bytes()[3..] {
        b"??" => "59".to_string(),
        &[b'?', v] => String::from_utf8(vec![b'5', v]).unwrap(),
        &[v, b'?'] => String::from_utf8(vec![v, b'9']).unwrap(),
        v => String::from_utf8(v.to_vec()).unwrap(),
    };
    let h = match &s.as_bytes()[..2] {
        b"??" => "11".to_string(),
        &[b'?', v] if v > b'1' => String::from_utf8(vec![b'0', v]).unwrap(),
        &[b'?', v] => String::from_utf8(vec![b'1', v]).unwrap(),
        &[b'0', b'?'] => "09".to_string(),
        &[b'1', b'?'] => "11".to_string(),
        v => String::from_utf8(v.to_vec()).unwrap(),
    };
    format!("{h}:{m}")
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
