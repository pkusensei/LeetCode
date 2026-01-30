mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

use itertools::izip;

pub fn minimum_cost(
    source: &str,
    target: &str,
    original: &[&str],
    changed: &[&str],
    cost: &[i32],
) -> i64 {
    let n_seg = original.len() * 2;
    let mut mat = vec![vec![None; n_seg]; n_seg];
    for i in 0..n_seg {
        mat[i][i] = Some(0);
    }
    let mut global_id = 0;
    let mut trie = Trie::default();
    for (a, b, c) in izip!(original.iter(), changed.iter(), cost.iter()) {
        let [a, b] = [a, b].map(|s| trie.insert(s.bytes().rev(), &mut global_id));
        let v = mat[a][b].get_or_insert(i64::from(*c));
        *v = (*v).min(i64::from(*c));
    }
    for mid in 0..n_seg {
        for a in 0..n_seg {
            let Some(x) = mat[a][mid] else {
                continue; // SMH this is key to avoid TLE; WTF
            };
            for b in 0..n_seg {
                if let Some(y) = mat[mid][b] {
                    let v = mat[a][b].get_or_insert(x + y);
                    *v = (*v).min(x + y);
                }
            }
        }
    }
    let n_src = source.len();
    let mut dp = vec![i64::MAX >> 2; 1 + n_src];
    dp[0] = 0;
    for (idx, (src, tgt)) in izip!(source.bytes(), target.bytes()).enumerate() {
        if src == tgt {
            dp[1 + idx] = dp[1 + idx].min(dp[idx]);
        }
        let mut src_trie = &trie;
        let mut tgt_trie = &trie;
        for left in (0..=idx).rev() {
            if let Some(strie) = src_trie.find(source.as_bytes()[left])
                && let Some(ttrie) = tgt_trie.find(target.as_bytes()[left])
            {
                src_trie = strie;
                tgt_trie = ttrie;
                if let Some((a, b)) = strie.id.zip(ttrie.id)
                    && let Some(v) = mat[a][b]
                {
                    dp[1 + idx] = dp[1 + idx].min(dp[left] + v);
                }
            } else {
                break; // no more branch
            };
        }
    }
    if dp[n_src] >= i64::MAX >> 2 {
        -1
    } else {
        dp[n_src]
    }
}

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 26],
    id: Option<usize>,
}

impl Trie {
    fn insert(&mut self, it: impl Iterator<Item = u8>, global_id: &mut usize) -> usize {
        let mut curr = self;
        for b in it {
            let node = curr.nodes[usize::from(b - b'a')].get_or_insert_default();
            curr = node;
        }
        if curr.id.is_none() {
            curr.id = Some(*global_id);
            *global_id = *global_id + 1;
        }
        curr.id.unwrap()
    }

    fn find(&self, b: u8) -> Option<&Trie> {
        let i = usize::from(b - b'a');
        self.nodes[i].as_deref()
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
    fn basics() {
        assert_eq!(
            minimum_cost(
                "abcd",
                "acbe",
                &["a", "b", "c", "c", "e", "d"],
                &["b", "c", "b", "e", "b", "e"],
                &[2, 5, 5, 1, 2, 20]
            ),
            28
        );
    }

    #[test]
    fn test() {}
}
