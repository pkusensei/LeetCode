mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_envelopes(mut envelopes: Vec<[i32; 2]>) -> i32 {
    envelopes.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
    let mut lis = vec![envelopes[0][1]];
    for e in &envelopes {
        let i = lis.partition_point(|&v| v < e[1]);
        if i >= lis.len() {
            lis.push(e[1]);
        } else {
            lis[i] = e[1];
        }
    }
    lis.len() as i32
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
        assert_eq!(max_envelopes(vec![[5, 4], [6, 4], [6, 7], [2, 3]]), 3);
    }

    #[test]
    fn test() {}
}
