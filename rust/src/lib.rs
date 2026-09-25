mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::BTreeSet;

#[allow(unused_imports)]
use helper::*;

pub fn brace_expansion_ii(expression: String) -> Vec<String> {
    dfs(expression.as_bytes())
        .into_iter()
        .map(|v| String::from_utf8(v).unwrap())
        .collect()
}

fn dfs(s: &[u8]) -> BTreeSet<Vec<u8>> {
    let mut left = 0;
    let mut open = 0;
    let mut block = vec![];
    let mut res = BTreeSet::new();
    for (right, &b) in s.iter().enumerate() {
        match b {
            b'{' => {
                open += 1;
                if open == 1 {
                    left = 1 + right;
                }
            }
            b'}' => {
                open -= 1;
                if open == 0 {
                    block.push(dfs(&s[left..right]));
                }
            }
            // {...},{...}
            // Left and right are unioned
            b',' if open == 0 => {
                res.extend(process(block));
                block = vec![];
            }
            // a, b, c
            _ if open == 0 => {
                block.push(BTreeSet::from([vec![b]]));
            }
            _ => (),
        }
    }
    res.extend(process(block));
    res
}

fn process(block: Vec<BTreeSet<Vec<u8>>>) -> Vec<Vec<u8>> {
    use itertools::Itertools;

    let mut res = vec![vec![]];
    for set in block {
        let temp = res.iter().cartesian_product(set.iter()).map(|(a, b)| {
            let mut a = a.to_vec();
            a.extend_from_slice(b);
            a
        });
        res = temp.collect();
    }
    res
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
    fn basics() {}

    #[test]
    fn test() {}
}
