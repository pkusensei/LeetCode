mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
    let mut trie = Trie::default();
    for s in words.iter() {
        trie.insert(s);
    }
    words.iter().map(|s| trie.find(s)).collect()
}

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 26],
    count: i32,
}

impl Trie {
    fn insert(&mut self, s: &str) {
        let mut curr = self;
        for b in s.bytes() {
            let idx = usize::from(b - b'a');
            curr = curr.nodes[idx].get_or_insert(Box::new(Trie::default()));
            curr.count += 1;
        }
    }

    fn find(&self, s: &str) -> i32 {
        let mut curr = self;
        let mut res = curr.count;
        for b in s.bytes() {
            let idx = usize::from(b - b'a');
            let Some(ref node) = curr.nodes[idx] else {
                return 0;
            };
            curr = node;
            res += curr.count;
        }
        res
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
