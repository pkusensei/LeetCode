mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn count_no_zero_pairs(n: i64) -> i64 {
    let s: Vec<u8> = n.to_string().bytes().map(|b| b - b'0').rev().collect();
    let len = s.len();
    let mut res = 0;
    for len_a in 1..=len {
        for len_b in 1..=len {
            res += dfs(&s, 0, 0, len_a, len_b, &mut HashMap::new());
        }
    }
    res
}

fn dfs(
    s: &[u8],
    idx: usize,
    carry: u8,
    len_a: usize,
    len_b: usize,
    memo: &mut HashMap<(usize, u8, usize, usize), i64>,
) -> i64 {
    if s.len() <= idx {
        return (carry == 0).into();
    }
    let k = (idx, carry, len_a, len_b);
    if let Some(&v) = memo.get(&k) {
        return v;
    }
    // While building actual numbers, i.e idx<len, consider only 1..=9
    // 0's could be used only as leading zeros
    let [range_a, range_b] = [len_a, len_b].map(|x| if idx < x { 1..=9 } else { 0..=0 });
    let mut res = 0;
    for da in range_a {
        for db in range_b.clone() {
            let sum = da + db + carry;
            if sum % 10 == s[idx] {
                res += dfs(s, 1 + idx, sum / 10, len_a, len_b, memo)
            }
        }
    }
    memo.insert(k, res);
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
        assert_eq!(count_no_zero_pairs(11), 8);
    }

    #[test]
    fn test() {}
}
