mod dsu;
mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
    let n = s.len();
    let mut seen = HashSet::from([s.clone().into_bytes()]);
    let mut queue = VecDeque::from([s.clone().into_bytes()]);
    let mut res = s.into_bytes();
    while let Some(curr) = queue.pop_front() {
        if res > curr {
            res = curr.clone();
        }
        // rotate
        let mut temp = curr.clone();
        temp.rotate_right(b as usize);
        if seen.insert(temp.clone()) {
            queue.push_back(temp);
        }
        temp = curr;
        for i in (0..n).filter(|i| i & 1 == 1) {
            temp[i] = (temp[i] - b'0' + a as u8) % 10 + b'0';
        }
        if seen.insert(temp.clone()) {
            queue.push_front(temp.clone());
        }
    }
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

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

    #[test]
    fn basics() {}

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
