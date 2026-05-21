mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut trie = Trie::default();
    for v in arr1 {
        trie.insert(v.to_string().bytes());
    }
    arr2.iter()
        .map(|v| trie.find(v.to_string().bytes()))
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
        let mut len = 0;
        for b in it {
            let i = usize::from(b - b'0');
            len += 1;
            curr = curr.nodes[i].get_or_insert_default();
            curr.len = len;
        }
    }

    fn find(&self, it: impl Iterator<Item = u8>) -> i32 {
        let mut curr = self;
        for b in it {
            let i = usize::from(b - b'0');
            let Some(v) = curr.nodes[i].as_ref() else {
                break;
            };
            curr = v;
        }
        curr.len
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
