mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_recolors(blocks: &str, k: i32) -> i32 {
    let mut res = k;
    let k = k as usize;
    let s = blocks.as_bytes();
    let mut curr = 0;
    for (idx, &b) in s.iter().enumerate() {
        curr += i32::from(b == b'W');
        if idx >= k {
            curr -= i32::from(s[idx - k] == b'W');
        }
        if idx >= k - 1 {
            res = res.min(curr);
        }
    }
    res
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
        assert_eq!(minimum_recolors("WBWBBBW", 2), 0);
    }

    #[test]
    fn test() {}
}
