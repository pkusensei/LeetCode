mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_special_chars(word: String) -> i32 {
    let mut up = [None; 26];
    let mut low = [None; 26];
    for (idx, b) in word.bytes().enumerate() {
        if b.is_ascii_uppercase() {
            let i = usize::from(b - b'A');
            if up[i].is_none() {
                up[i] = Some(idx);
            }
        } else {
            low[usize::from(b - b'a')] = Some(idx);
        }
    }
    up.into_iter()
        .zip(low)
        .filter(|(first_up, last_low)| first_up.zip(*last_low).is_some_and(|(a, b)| a > b))
        .count() as i32
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
