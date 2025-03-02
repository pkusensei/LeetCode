mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn longest_palindrome(words: &[&str]) -> i32 {
    let mut sym = HashMap::new(); // 5 3 3 3
    let mut asym = HashMap::<[u8; 2], [i32; 2]>::new();
    for w in words.iter() {
        let [a, b] = w.as_bytes()[..] else {
            unreachable!()
        };
        if a == b {
            *sym.entry([a; 2]).or_insert(0) += 1;
        } else if let Some(v) = asym.get_mut(&[b, a]) {
            v[1] += 1;
        } else {
            asym.entry([a, b]).or_insert([0, 0])[0] += 1;
        }
    }
    // Even syms are all added in
    // Odd syms => keep the max one, take all others as (val-1)
    let mut res: i32 = sym.values().sum();
    let odd_count = sym.values().filter(|&&v| v & 1 == 1).count() as i32;
    if odd_count > 0 {
        res -= odd_count;
        res += 1;
    }
    res += asym.values().map(|v| 2 * v[0].min(v[1])).sum::<i32>();
    2 * res
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
    fn test() {
        assert_eq!(
            longest_palindrome(&[
                "dd", "aa", "bb", "dd", "aa", "dd", "bb", "dd", "aa", "cc", "bb", "cc", "dd", "cc"
            ]),
            22
        );
    }
}
