mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn valid_sequence(word1: &str, word2: &str) -> Vec<i32> {
    let (s1, s2) = (word1.as_bytes(), word2.as_bytes());
    let (n1, n2) = (word1.len(), word2.len());
    let mut suffix = vec![n1; n2];
    let mut i2 = n2 - 1;
    for (i1, &b1) in s1.iter().enumerate().rev() {
        if s2[i2] == b1 {
            suffix[i2] = i1;
            let Some(v) = i2.checked_sub(1) else {
                break;
            };
            i2 = v;
        }
    }
    i2 = 0;
    let mut skipped = false;
    let mut res = Vec::with_capacity(n2);
    for (i1, &b1) in s1.iter().enumerate() {
        let Some(&b2) = s2.get(i2) else {
            break;
        };
        if b1 == b2 {
            res.push(i1 as i32);
            i2 += 1;
        } else if !skipped && suffix.get(1 + i2).is_none_or(|&v| v > i1 && v < n1) {
            skipped = true;
            res.push(i1 as i32);
            i2 += 1;
        }
    }
    if i2 >= n2 { res } else { vec![] }
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
        assert_eq!(valid_sequence("abc", "ab"), [0, 1]);
        assert_eq!(valid_sequence("b", "a"), [0]);
    }

    #[test]
    fn test() {}
}
