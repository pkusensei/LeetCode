mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_repeating(s: &str, query_characters: &str, query_indices: &[i32]) -> Vec<i32> {
    let n = s.len();
    let mut st = SegTree::new(n);
    for (i, b) in s.bytes().enumerate() {
        st.update(i, b);
    }
    let mut res = Vec::with_capacity(query_characters.len());
    for (b, &i) in query_characters.bytes().zip(query_indices.iter()) {
        st.update(i as usize, b);
        res.push(st.tree[1].max_len);
    }
    res
}

#[derive(Clone, Copy, Default)]
struct Node {
    len: i32,
    max_len: i32,
    left: u8,
    left_len: i32,
    right: u8,
    right_len: i32,
}

impl Node {
    fn merge(self: Node, other: Node) -> Self {
        if self.right == other.left {
            let mid = self.right_len + other.left_len;
            let max_len = self.max_len.max(other.max_len).max(mid);
            let left_len = if self.left_len == self.len {
                mid
            } else {
                self.left_len
            };
            let right_len = if other.right_len == other.len {
                mid
            } else {
                other.right_len
            };
            Self {
                len: self.len + other.len,
                max_len,
                left: self.left,
                left_len,
                right: other.right,
                right_len,
            }
        } else {
            Self {
                len: self.len + other.len,
                max_len: self.max_len.max(other.max_len),
                left: self.left,
                left_len: self.left_len,
                right: other.right,
                right_len: other.right_len,
            }
        }
    }
}

struct SegTree {
    tree: Vec<Node>,
    n: usize,
}

impl SegTree {
    fn new(n: usize) -> Self {
        SegTree {
            tree: vec![Node::default(); 4 * n],
            n,
        }
    }

    fn update(&mut self, idx: usize, byte: u8) {
        self._update(1, 0, self.n - 1, idx, byte);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, idx: usize, byte: u8) {
        if left == right {
            self.tree[node] = Node {
                len: 1,
                max_len: 1,
                left: byte,
                left_len: 1,
                right: byte,
                right_len: 1,
            };
            return;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self._update(2 * node, left, mid, idx, byte);
        } else {
            self._update(1 + 2 * node, 1 + mid, right, idx, byte);
        }
        self.tree[node] = self.tree[2 * node].merge(self.tree[1 + 2 * node]);
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
    fn basics() {
        assert_eq!(longest_repeating("babacc", "bcb", &[1, 3, 3]), [3, 3, 4])
    }

    #[test]
    fn test() {}
}
