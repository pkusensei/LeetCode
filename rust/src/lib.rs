mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_of_subsequences(s: &str) -> i64 {
    let n = s.len();
    if n < 2 {
        return 0;
    }
    let [mut prefix_l, mut suffix_t] = [vec![], vec![]];
    for b in s.bytes() {
        prefix_l.push(i64::from(b == b'L'));
        suffix_t.push(i64::from(b == b'T'));
    }
    for i in 1..n {
        prefix_l[i] += prefix_l[i - 1];
    }
    for i in (0..n - 1).rev() {
        suffix_t[i] += suffix_t[1 + i];
    }
    let mut base = 0;
    let [mut add_l, mut add_c, mut add_t] = [0; 3];
    for (i, b) in s.bytes().enumerate() {
        if b == b'C' {
            base += prefix_l[i] * suffix_t[i];
            add_l += suffix_t[i];
            add_t += prefix_l[i];
        } else {
            add_c = add_c.max(prefix_l[i] * suffix_t[i]);
        }
    }
    base + add_l.max(add_c).max(add_t)
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
        assert_eq!(num_of_subsequences("LMCT"), 2);
        assert_eq!(num_of_subsequences("LCCT"), 4);
        assert_eq!(num_of_subsequences("L"), 0);
    }

    #[test]
    fn test() {}
}
