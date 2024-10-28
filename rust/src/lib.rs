mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn find_secret_word(words: &[&str], master: &Master) {
    // str_idx -> common count -> Vec<str_idx>
    let mut graph: HashMap<usize, HashMap<usize, HashSet<usize>>> = HashMap::new();
    for (i1, w1) in words.iter().enumerate() {
        for (i2, w2) in words.iter().enumerate().skip(1 + i1) {
            let s = similar(&w1, &w2);
            graph
                .entry(i1)
                .or_default()
                .entry(s)
                .or_default()
                .insert(i2);
            graph
                .entry(i2)
                .or_default()
                .entry(s)
                .or_default()
                .insert(i1);
        }
    }
    let mut candid: VecDeque<_> = (0..words.len()).collect();
    let mut seen = HashSet::new();
    while let Some(idx) = candid.pop_front() {
        if !seen.insert(idx) {
            continue;
        }
        match master.guess(words[idx].to_string()) {
            6 => break,
            n => {
                candid = graph[&idx][&(n as usize)].iter().copied().collect();
            }
        }
    }
}

fn similar(a: &str, b: &str) -> usize {
    a.bytes()
        .zip(b.bytes())
        .enumerate()
        .filter_map(|(i, (b1, b2))| if b1 == b2 { Some(i) } else { None })
        .count()
}

struct Master;
impl Master {
    fn guess(&self, word: String) -> i32 {
        0
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
