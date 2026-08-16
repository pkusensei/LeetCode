mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_gap(skill: &str, station: &str) -> i32 {
    let (st, st_n) = (station.as_bytes(), station.len());
    let mut st_i = 0;
    let mut prefix = Vec::with_capacity(skill.len());
    for sk in skill.bytes() {
        while sk != st[st_i] {
            st_i += 1;
        }
        prefix.push(st_i);
        st_i += 1;
    }
    let mut suffix = Vec::with_capacity(skill.len());
    st_i = st_n - 1;
    for sk in skill.bytes().rev() {
        while st[st_i] != sk {
            st_i -= 1;
        }
        suffix.push(st_i);
        st_i -= 1;
    }
    suffix.reverse();
    let mut res = 0;
    for i in 0..skill.len() - 1 {
        res = res.max(suffix[1 + i] - prefix[i])
    }
    res as i32
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
        assert_eq!(maximum_gap("caa", "acaa"), 1)
    }

    #[test]
    fn test() {}
}
