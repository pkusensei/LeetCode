mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
    let n = lcp.len();
    let mut res = vec![0; n];
    let mut byte = b'a';
    for (r, row) in lcp.iter().enumerate() {
        if res[r] > 0 {
            continue;
        }
        for (c, &v) in row.iter().enumerate() {
            if v > 0 {
                if byte > b'z' {
                    return "".to_string();
                }
                res[c] = byte;
            }
        }
        byte += 1;
    }
    for r in 0..n {
        for c in 0..n {
            let one_after = *lcp.get(1 + r).and_then(|rr| rr.get(1 + c)).unwrap_or(&0);
            let temp = if res[r] == res[c] { 1 + one_after } else { 0 };
            if temp != lcp[r][c] {
                return "".to_string();
            }
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
