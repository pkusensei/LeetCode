mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_smallest_string(s: String, mut k: i32) -> String {
        let mut s = s.into_bytes();
        for v in s.iter_mut() {
            for b in b'a'..=b'z' {
                let d = dist(*v, b);
                if d <= k {
                    k -= d;
                    *v = b;
                    break;
                }
            }
            if k == 0 {
                break;
            }
        }
        String::from_utf8(s).unwrap()
}

fn dist(b1: u8, b2: u8) -> i32 {
    let d = i32::from(b1.abs_diff(b2));
    d.min(26 - d)
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
