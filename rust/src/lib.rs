mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
    let s1: Vec<_> = sentence1.split_whitespace().collect();
    let s2: Vec<_> = sentence2.split_whitespace().collect();
    let (n1, n2) = (s1.len(), s2.len());
    let [mut prefix, mut suffix] = [0, 0];
    let [mut i1, mut i2] = [0, 0];
    while s1.get(i1).zip(s2.get(i2)).is_some_and(|(&a, &b)| a == b) {
        prefix += 1;
        i1 += 1;
        i2 += 1;
    }
    i1 = n1 - 1;
    i2 = n2 - 1;
    while s1.get(i1).zip(s2.get(i2)).is_some_and(|(&a, &b)| a == b) {
        suffix += 1;
        if i1 == 0 || i2 == 0 {
            break;
        }
        i1 -= 1;
        i2 -= 1;
    }
    n1.min(n2) <= prefix + suffix
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
    fn basics() {}

    #[test]
    fn test() {}
}
