mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn validate_coupons(
    code: Vec<String>,
    business_line: Vec<String>,
    is_active: Vec<bool>,
) -> Vec<String> {
    use itertools::{Itertools, izip};
    izip!(code, business_line, is_active)
        .filter_map(|(c, b, act)| {
            if !c.is_empty()
                && c.bytes().all(|v| v.is_ascii_alphanumeric() || v == b'_')
                && matches!(
                    b.as_ref(),
                    "electronics" | "grocery" | "pharmacy" | "restaurant"
                )
                && act
            {
                Some((b, c))
            } else {
                None
            }
        })
        .sorted_unstable()
        .map(|(_, c)| c)
        .collect()
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
