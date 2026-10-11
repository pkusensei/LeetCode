mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_inversions(words: &[&str], target: &[i32]) -> i32 {
    let n = target.len();
    let mut target_idx = vec![0; n];
    for (i, &v) in target.iter().enumerate() {
        target_idx[v as usize] = i;
    }
    let mut trie = Trie::new();
    for (i, w) in words.iter().enumerate() {
        trie.insert(w.as_bytes(), target_idx[i]);
    }
    trie.traverse().1
}

const N: usize = 10_001;
const MAX: usize = (b'j' - b'a') as usize;
#[derive(Clone)]
struct Trie {
    nodes: [Option<Box<Trie>>; 1 + MAX],
    pos: usize,  // position of word_idx in target
    full: usize, // All child nodes, <= 2^10
}

impl Trie {
    fn new() -> Self {
        Self {
            nodes: Default::default(),
            pos: N,
            full: 0,
        }
    }

    fn insert(&mut self, s: &[u8], pos: usize) {
        let Some(b) = s.first() else {
            self.pos = pos;
            return;
        };
        let bi = usize::from(b - b'a');
        let node = self.nodes[bi].get_or_insert(Box::new(Trie::new()));
        node.insert(&s[1..], pos);
        self.full |= 1 << bi;
    }

    fn traverse(&self) -> (Vec<usize>, i32) {
        if self.full == 0 {
            return (vec![self.pos], 0);
        }
        // Collect
        let mut children = vec![(vec![], 0); 1 + MAX];
        for (i, node) in self.nodes.iter().enumerate() {
            if let Some(node) = node.as_ref() {
                children[i] = node.traverse();
            }
        }
        // Set up DP
        let mut groups = vec![vec![]; 1 + self.full];
        if self.pos < N {
            groups[0].push(self.pos);
        }
        let mut min_invs = vec![i32::MAX >> 1; 1 + self.full];
        min_invs[0] = 0;
        // SoS
        for mask in 1..=self.full {
            if mask | self.full != self.full {
                continue;
            }
            for bit in 0..=MAX {
                if mask & (1 << bit) == 0 {
                    continue;
                }
                let prev = mask ^ (1 << bit);
                let new_gr = &children[bit].0;
                let prev_gr = &groups[prev];
                let mut curr = children[bit].1 + min_invs[prev];
                for &pos in new_gr {
                    curr += prev_gr.partition_point(|&v| v > pos) as i32;
                }
                min_invs[mask] = min_invs[mask].min(curr);
                if groups[mask].is_empty() {
                    let mut v = prev_gr.to_vec();
                    v.extend_from_slice(new_gr);
                    v.sort_unstable_by(|a, b| b.cmp(a));
                    groups[mask] = v;
                }
            }
        }
        (groups[self.full].clone(), min_invs[self.full])
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
        assert_eq!(min_inversions(&["ab", "ac", "ba", "bc"], &[0, 3, 1, 2]), 1);
    }

    #[test]
    fn test() {
        assert_eq!(min_inversions(&["fi", "gd", "f", "ja"], &[2, 3, 0, 1]), 1);
        assert_eq!(min_inversions(&["a", "b"], &[1, 0]), 0);
        assert_eq!(min_inversions(&["hhjfieaeig", "fbhdhchfhgdhd"], &[1, 0]), 0);
    }
}
