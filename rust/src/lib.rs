mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mut trie = Trie::default();
    for &num in &nums {
        trie.insert(num);
    }
    nums.into_iter().map(|v| trie.find(v)).max().unwrap_or(0)
}

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 2],
}

impl Trie {
    fn insert(&mut self, num: i32) {
        let mut curr = self;
        for bit in (0..32).rev() {
            let i = (num >> bit) & 1;
            curr = curr.nodes[i as usize].get_or_insert_default();
        }
    }

    fn find(&self, num: i32) -> i32 {
        let mut curr = self;
        let mut res = 0;
        for bit in (0..32).rev() {
            let target = 1 - ((num >> bit) & 1);
            if let Some(v) = curr.nodes[target as usize].as_ref() {
                curr = v;
                res |= 1 << bit;
            } else if let Some(v) = curr.nodes[1 - target as usize].as_ref() {
                curr = v;
            } else {
                break;
            }
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
