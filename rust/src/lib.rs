mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_magical_pairs(s: &str) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let mut masks = vec![vec![0; n]; n];
    for left in 0..n {
        let mut mask = 0_i32;
        for right in left..n {
            mask |= 1 << (s[right] - b'a');
            masks[left][right] = mask.count_ones();
        }
    }
    let mut res = 0;
    for left in 0..n {
        for mid in left..n {
            for right in 1 + mid..n {
                if masks[left][mid] == masks[1 + mid][right] {
                    res += 1
                } else if masks[left][mid] < masks[1 + mid][right] {
                    break;
                }
            }
        }
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
    fn basics() {
        assert_eq!(count_magical_pairs("aba"), 2)
    }

    #[test]
    fn test() {}
}
