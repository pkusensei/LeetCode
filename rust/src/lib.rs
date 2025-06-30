mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn generate_tag(caption: String) -> String {
    let mut res = vec![b'#'];
    'outer: for (i, s) in caption.split(' ').enumerate() {
        let mut cap = i > 0 && res.len() > 1;
        for b in s.bytes().filter(|&b| b.is_ascii_alphabetic()) {
            if cap {
                res.push(b.to_ascii_uppercase());
                cap = false;
            } else {
                res.push(b.to_ascii_lowercase());
            }
            if res.len() >= 100 {
                break 'outer;
            }
        }
    }
    while res.len() > 100 {
        res.pop();
    }
    String::from_utf8(res).unwrap()
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
