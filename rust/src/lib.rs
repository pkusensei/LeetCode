mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct OrderedStream {
    data: Vec<String>,
    ptr: usize,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        Self {
            data: vec!["".to_string(); n as usize],
            ptr: 0,
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let id = id_key as usize - 1;
        self.data[id] = value;
        if id == self.ptr {
            let res: Vec<_> = self.data[id..]
                .iter()
                .take_while(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();
            self.ptr += res.len();
            res
        } else {
            return vec![];
        }
    }
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
