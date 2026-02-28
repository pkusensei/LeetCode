mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn merge_characters(s: String, k: i32) -> String {
    let k = k as usize;
    let mut res = vec![];
    let mut window = [false; 26];
    for b in s.bytes() {
        let bi = usize::from(b - b'a');
        if window[bi] {
            continue;
        }
        window[bi] = true;
        res.push(b);
        if res.len() > k {
            let temp = res[res.len() - k - 1];
            window[usize::from(temp - b'a')] = false;
        }
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
