mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_groups(words: Vec<String>) -> i32 {
    use itertools::Itertools;
    words.iter().map(|s| process(s)).unique().count() as i32
}

fn process(s: &str) -> [i64; 2] {
    let [mut odd, mut even] = [const { vec![] }; 2];
    for (i, b) in s.bytes().enumerate() {
        if i & 1 == 1 {
            odd.push(i64::from(b - b'a'));
        } else {
            even.push(i64::from(b - b'a'));
        }
    }
    [f(odd), f(even)]
}

const M: i64 = 1_000_000_007;
const BASE: i64 = 37;
fn f(mut s: Vec<i64>) -> i64 {
    if s.is_empty() {
        return 0;
    }
    let n = s.len();
    s.extend_from_within(..);
    let mut curr = 0;
    let mut pow = 1;
    for b in s[..n].iter() {
        curr = (curr * BASE % M + b) % M;
        pow = pow * BASE % M;
    }
    let mut res = curr; // min hash
    for i in n..2 * n {
        curr = (curr * BASE % M + s[i]) % M;
        curr = (curr - s[i - n] * pow % M).rem_euclid(M);
        res = res.min(curr);
    }
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
    fn basics() {}

    #[test]
    fn test() {}
}
