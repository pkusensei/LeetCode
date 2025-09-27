mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn distinct_points(s: &str, k: i32) -> i32 {
    use std::collections::HashSet;
    let (s, n) = (s.as_bytes(), s.len());
    let k = k as usize;
    if n == k {
        return 1;
    }
    let mut end = [0, 0];
    for &b in s {
        match b {
            b'U' => end[1] += 1,
            b'D' => end[1] -= 1,
            b'L' => end[0] -= 1,
            b'R' => end[0] += 1,
            _ => (),
        }
    }
    let mut seen = HashSet::new();
    let mut acc_x = 0;
    let mut acc_y = 0;
    for (right, &b) in s.iter().enumerate() {
        match b {
            b'U' => acc_y += 1,
            b'D' => acc_y -= 1,
            b'L' => acc_x -= 1,
            b'R' => acc_x += 1,
            _ => (),
        }
        if right >= k {
            match s[right - k] {
                b'U' => acc_y -= 1,
                b'D' => acc_y += 1,
                b'L' => acc_x += 1,
                b'R' => acc_x -= 1,
                _ => (),
            }
        }
        if right >= k - 1 {
            let mut curr = end;
            curr[0] -= acc_x;
            curr[1] -= acc_y;
            seen.insert(curr);
        }
    }
    seen.len() as i32
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
    fn basics() {
        assert_eq!(distinct_points("LUL", 1), 2);
        assert_eq!(distinct_points("UDLR", 4), 1);
        assert_eq!(distinct_points("UU", 1), 1);
    }

    #[test]
    fn test() {}
}
