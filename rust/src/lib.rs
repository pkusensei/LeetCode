mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut trie = Trie::default();
    for &num in &arr1 {
        trie.insert(num.to_string().bytes());
    }
    arr2.into_iter()
        .map(|num| trie.find(num.to_string().bytes()))
        .max()
        .unwrap_or(0)
}

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 10],
    len: i32,
}

impl Trie {
    fn insert(&mut self, it: impl Iterator<Item = u8>) {
        let mut curr = self;
        let mut len = 1;
        for i in it.map(|b| usize::from(b - b'0')) {
            curr = curr.nodes[i].get_or_insert(Default::default());
            curr.len = len;
            len += 1;
        }
    }

    fn find(&self, it: impl Iterator<Item = u8>) -> i32 {
        let mut curr = self;
        let mut res = 0;
        for i in it.map(|b| usize::from(b - b'0')) {
            let Some(ref v) = curr.nodes[i] else {
                break;
            };
            curr = v;
            res = res.max(curr.len);
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
