mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn merge_alternately(word1: String, word2: String) -> String {
    let (s1, s2) = (word1.as_bytes(), word2.as_bytes());
    let mut res = vec![];
    let [mut i1, mut i2] = [0, 0];
    while let (Some(&b1), Some(&b2)) = (s1.get(i1), s2.get(i2)) {
        res.push(b1);
        res.push(b2);
        i1 += 1;
        i2 += 1;
    }
    res.extend_from_slice(&s1[i1..]);
    res.extend_from_slice(&s2[i2..]);
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
