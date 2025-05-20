mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn valid_sequence(word1: &str, word2: &str) -> Vec<i32> {
    let [(s1, _n1), (s2, n2)] = [&word1, &word2].map(|w| (w.as_bytes(), w.len()));
    let mut last = vec![-1; n2];
    let mut i2 = n2 - 1;
    // latest pos of any b2 in s1
    for (i1, &b1) in s1.iter().enumerate().rev() {
        if b1 == s2[i2] {
            last[i2] = i1 as i32;
            if i2 == 0 {
                break;
            }
            i2 -= 1;
        }
    }
    i2 = 0;
    let mut skipped = false;
    let mut res = Vec::with_capacity(n2);
    for (i1, &b1) in s1.iter().enumerate() {
        let b2 = s2[i2];
        // Greedy pick
        // 1) b1==b2
        // 2) skip early when next b2 is able to be picked up
        if b1 == b2 || (!skipped && last.get(1 + i2).is_none_or(|&v| (i1 as i32) < v)) {
            res.push(i1 as i32);
            skipped |= b1 != b2;
            i2 += 1;
            if i2 == n2 {
                break;
            }
        }
    }
    if i2 == n2 { res } else { vec![] }
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
        assert_eq!(valid_sequence("vbcca", "abc"), [0, 1, 2]);
        assert_eq!(valid_sequence("bacdc", "abc"), [1, 2, 4]);
        assert_eq!(valid_sequence("aaaaaa", "aaabc"), []);
    }

    #[test]
    fn test() {
        assert_eq!(valid_sequence("ccbccccbcc", "b"), [0]);
        assert_eq!(valid_sequence("bbeigiibhjafjig", "iihhj"), [3, 5, 6, 8, 9]);
    }
}
