mod dsu;
mod helper;
mod trie;

use std::collections::{BTreeSet, HashMap};

#[allow(unused_imports)]
use helper::*;

struct MovieRentingSystem {
    // movie -> [price, shop]
    data: HashMap<i32, BTreeSet<[i32; 2]>>,
    // [shop, movie] -> price
    prices: HashMap<[i32; 2], i32>,
    // [price, shop, movie]
    out: BTreeSet<[i32; 3]>,
}

impl MovieRentingSystem {
    fn new(_n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut data: HashMap<i32, BTreeSet<[i32; 2]>> = HashMap::new();
        let mut prices = HashMap::new();
        for e in entries.into_iter() {
            let [shop, movie, price] = e[..] else {
                unreachable!()
            };
            data.entry(movie).or_default().insert([price, shop]);
            prices.insert([shop, movie], price);
        }
        Self {
            data,
            prices,
            out: BTreeSet::new(),
        }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        self.data
            .get(&movie)
            .map(|set| set.iter().map(|&[_p, s]| s).take(5).collect())
            .unwrap_or_default()
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        let Some(set) = self.data.get_mut(&movie) else {
            return;
        };
        let price = self.prices[&[shop, movie]];
        set.remove(&[price, shop]);
        self.out.insert([price, shop, movie]);
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        let price = self.prices[&[shop, movie]];
        self.out.remove(&[price, shop, movie]);
        self.data.entry(movie).or_default().insert([price, shop]);
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.out.iter().map(|v| vec![v[1], v[2]]).take(5).collect()
    }
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
