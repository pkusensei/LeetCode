mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn count_pairs(nums: &[i32], low: i32, high: i32) -> i32 {
    let mut res = 0;
    for (i, a) in nums.iter().enumerate() {
        for b in nums.iter().skip(1 + i) {
            res += i32::from((low..=high).contains(&(a ^ b)));
        }
    }
    res
}

pub fn with_trie(nums: &[i32], low: i32, high: i32) -> i32 {
    #[derive(Debug, Clone, Default)]
    struct Trie {
        nodes: [Option<Box<Trie>>; 2],
        count: i32,
    }

    impl Trie {
        fn insert(&mut self, num: i32) {
            let mut curr = self;
            for i in (0..15).rev() {
                let bit = ((num >> i) & 1) as usize;
                curr = curr.nodes[bit].get_or_insert(Box::new(Trie::default()));
                curr.count += 1;
            }
        }

        fn count(&self, num: i32, high: i32) -> i32 {
            let mut res = 0;
            let mut curr = self;
            for i in (0..15).rev() {
                if curr.nodes.iter().all(|v| v.is_none()) {
                    break;
                }
                let [bit_num, bit_high] = [num, high].map(|v| ((v >> i) & 1) as usize);
                if bit_high == 1 {
                    if let Some(ref v) = curr.nodes[bit_num] {
                        // xor results in 0 on this bit => smaller value
                        res += v.count;
                    }
                    // Now count the other branch
                    curr = match curr.nodes[1 - bit_num] {
                        Some(ref v) => v,
                        _ => break,
                    }
                } else {
                    // The other branch certainly yields 1 => bigger value
                    // ignore those
                    curr = match curr.nodes[bit_num] {
                        Some(ref v) => v,
                        _ => break,
                    }
                }
            }
            res
        }
    }

    let mut trie = Trie::default();
    let mut res = 0;
    for &num in nums.iter() {
        res += trie.count(num, 1 + high) - trie.count(num, low);
        trie.insert(num);
    }
    res
}

pub fn with_sorcery(nums: &[i32], low: i32, high: i32) -> i32 {
    let mut freqs = nums.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    let [mut low, mut high] = [low, 1 + high];
    let [mut count1, mut count2] = [0, 0];
    while low > 0 || high > 0 {
        let mut freqs2 = HashMap::new();
        for (&k, &v) in freqs.iter() {
            *freqs2.entry(k >> 1).or_insert(0) += v;
            if (low & 1) == 1 {
                count1 += v * freqs.get(&((low ^ 1) ^ k)).unwrap_or(&0);
            }
            if (high & 1) == 1 {
                count2 += v * freqs.get(&((high ^ 1) ^ k)).unwrap_or(&0);
            }
        }
        freqs = freqs2;
        low >>= 1;
        high >>= 1;
    }
    (count2 >> 1) - (count1 >> 1)
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
    fn basics() {
        assert_eq!(count_pairs(&[1, 4, 2, 7], 2, 6), 6);
        assert_eq!(count_pairs(&[9, 8, 4, 2, 1], 5, 14), 8);

        assert_eq!(with_trie(&[1, 4, 2, 7], 2, 6), 6);
        assert_eq!(with_trie(&[9, 8, 4, 2, 1], 5, 14), 8);

        assert_eq!(with_sorcery(&[1, 4, 2, 7], 2, 6), 6);
        assert_eq!(with_sorcery(&[9, 8, 4, 2, 1], 5, 14), 8);
    }

    #[test]
    fn test() {}
}
