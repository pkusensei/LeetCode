mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_equivalent_string(s1: &str, s2: &str, base_str: &str) -> String {
    let mut dsu = DSU::new();
    for (a, b) in s1.bytes().zip(s2.bytes()) {
        dsu.union((a - b'a').into(), (b - b'a').into());
    }
    base_str
        .bytes()
        .map(|b| char::from(dsu.find((b - b'a').into()) as u8 + b'a'))
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct DSU {
    parent: [usize; 26],
}

impl DSU {
    fn new() -> Self {
        let mut parent = [0; 26];
        for (i, v) in parent.iter_mut().enumerate() {
            *v = i
        }
        Self { parent }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x])
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        match rx.cmp(&ry) {
            std::cmp::Ordering::Less => self.parent[ry] = rx,
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => self.parent[rx] = ry,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            smallest_equivalent_string("parker", "morris", "parser"),
            "makkek"
        );
        debug_assert_eq!(smallest_equivalent_string("hello", "world", "hold"), "hdld");
        debug_assert_eq!(
            smallest_equivalent_string("leetcode", "programs", "sourcecode"),
            "aauaaaaada"
        );
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
