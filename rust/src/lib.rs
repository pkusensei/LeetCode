mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct CombinationIterator {
    queue: std::collections::VecDeque<String>,
}

impl CombinationIterator {
    fn new(chs: String, length: i32) -> Self {
        fn backtrack(chs: &[u8], curr: &mut Vec<u8>, length: usize, res: &mut Vec<String>) {
            if curr.len() == length {
                let mut temp = curr.clone();
                temp.sort_unstable();
                res.push(String::from_utf8(temp).unwrap());
                return;
            }
            match chs {
                [] => (),
                [head, tail @ ..] => {
                    backtrack(tail, curr, length, res);
                    curr.push(*head);
                    backtrack(tail, curr, length, res);
                    curr.pop();
                }
            }
        }

        let mut res = vec![];
        let length = length as usize;
        backtrack(
            chs.as_bytes(),
            &mut Vec::with_capacity(length),
            length,
            &mut res,
        );
        res.sort_unstable();
        Self { queue: res.into() }
    }

    fn next(&mut self) -> String {
        self.queue.pop_front().unwrap()
    }

    fn has_next(&self) -> bool {
        !self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut it = CombinationIterator::new("abc".into(), 2);
        assert_eq!(it.next(), "ab"); // return "ab"
        assert!(it.has_next()); // return True
        assert_eq!(it.next(), "ac"); // return "ac"
        assert!(it.has_next()); // return True
        assert_eq!(it.next(), "bc"); // return "bc"
        assert!(!it.has_next()); // return False
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

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
