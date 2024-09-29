mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct Codec {
    data: Vec<String>,
}

impl Codec {
    fn new() -> Self {
        Self { data: vec![] }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, s: String) -> String {
        self.data.push(s);
        (self.data.len() - 1).to_string()
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, s: String) -> String {
        let i = s.parse::<usize>().unwrap();
        self.data[i].clone()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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
}
