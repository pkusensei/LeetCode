mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn highest_ranked_k_items(
    mut grid: Vec<Vec<i32>>,
    pricing: Vec<i32>,
    start: Vec<i32>,
    k: i32,
) -> Vec<Vec<i32>> {
    use std::collections::{BinaryHeap, VecDeque};
    let [low, high] = pricing[..] else {
        unreachable!()
    };
    let [row, col] = [0, 1].map(|i| start[i] as usize);
    let mut queue = VecDeque::from([([row, col], 0)]);
    let mut heap = BinaryHeap::new();
    if (low..=high).contains(&grid[row][col]) {
        heap.push(Item {
            dist: 0,
            price: grid[row][col],
            row,
            col,
        });
    }
    grid[row][col] = 0;
    while let Some(([row, col], dist)) = queue.pop_front() {
        for [nr, nc] in neighbors([row, col]) {
            let Some(&v) = grid.get(nr).and_then(|r| r.get(nc)) else {
                continue;
            };
            match v.cmp(&1) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Equal => queue.push_back(([nr, nc], 1 + dist)),
                std::cmp::Ordering::Greater => {
                    if (low..=high).contains(&v) {
                        heap.push(Item {
                            dist: 1 + dist,
                            price: v,
                            row: nr,
                            col: nc,
                        });
                    }
                    queue.push_back(([nr, nc], 1 + dist));
                }
            }
            grid[nr][nc] = 0;
        }
        while heap.len() > k as usize {
            heap.pop();
        }
    }
    heap.into_sorted_vec()
        .iter()
        .map(|v| vec![v.row as i32, v.col as i32])
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    dist: i32,
    price: i32,
    row: usize,
    col: usize,
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
