mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_length_after_removals(s: &str) -> i32 {
    let mut st = vec![];
    for b in s.bytes() {
        if st.last().is_some_and(|&top| top != b) {
            st.pop();
        } else {
            st.push(b);
        }
    }
    st.len() as i32
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
        assert_eq!(min_length_after_removals("aabbab"), 0);
        assert_eq!(min_length_after_removals("aaaa"), 4);
        assert_eq!(min_length_after_removals("aaabb"), 1);
    }

    #[test]
    fn test() {
        assert_eq!(min_length_after_removals("baaab"), 1);
    }
}
