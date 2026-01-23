mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_pair_removal(nums: &[i32]) -> i32 {
    use std::collections::BinaryHeap;

    let n = nums.len();
    let mut nodes = Vec::with_capacity(n);
    let mut merged = vec![false; n];
    let mut heap = BinaryHeap::new();
    let mut bad_count = 0;
    for (idx, &num) in nums.iter().enumerate() {
        nodes.push(Node {
            val: num.into(),
            prev: idx.checked_sub(1),
            next: if 1 + idx < n { Some(1 + idx) } else { None },
        });
        if idx > 0 {
            heap.push(Item {
                fst: idx - 1,
                snd: idx,
                cost: i64::from(nums[idx - 1]) + i64::from(num),
            });
            bad_count += i32::from(nums[idx - 1] > num);
        }
    }
    let mut res = 0;
    while let Some(Item { fst, snd, cost }) = heap.pop()
        && bad_count > 0
    {
        if merged[fst] || merged[snd] || nodes[fst].val + nodes[snd].val != cost {
            continue;
        }
        res += 1;
        if nodes[fst].val > nodes[snd].val {
            bad_count -= 1; // this is a bad pair
        }
        let prev = nodes[fst].prev;
        let next = nodes[snd].next;
        nodes[fst].next = next;
        if let Some(i) = next {
            nodes[i].prev = Some(fst);
        }
        if let Some(prev) = prev {
            if (1 + nodes[fst].val..=cost).contains(&nodes[prev].val) {
                bad_count -= 1;
            } else if (1 + cost..=nodes[fst].val).contains(&nodes[prev].val) {
                bad_count += 1;
            }
            heap.push(Item {
                fst: prev,
                snd: fst,
                cost: nodes[prev].val + cost,
            });
        }
        if let Some(next) = next {
            if nodes[next].val < nodes[snd].val && cost <= nodes[next].val {
                bad_count -= 1;
            } else if nodes[snd].val <= nodes[next].val && nodes[next].val < cost {
                bad_count += 1;
            }
            heap.push(Item {
                fst: fst,
                snd: next,
                cost: nodes[next].val + cost,
            });
        }
        nodes[fst].val = cost;
        merged[snd] = true;
    }
    res
}

#[derive(Default, Clone, Copy)]
struct Node {
    val: i64,
    prev: Option<usize>,
    next: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
struct Item {
    fst: usize,
    snd: usize,
    cost: i64,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// To make min heap
impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.fst.cmp(&self.fst))
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
        assert_eq!(minimum_pair_removal(&[5, 2, 3, 1]), 2);
        assert_eq!(minimum_pair_removal(&[1, 2, 2]), 0);
    }

    #[test]
    fn test() {
        assert_eq!(
            minimum_pair_removal(&[2, 2, -1, 3, -2, 2, 1, 1, 1, 0, -1]),
            9
        );
    }
}
