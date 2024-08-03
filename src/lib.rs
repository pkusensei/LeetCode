use std::collections::{BinaryHeap, HashMap};

pub fn min_path_sum(grid: &[&[i32]]) -> i32 {
    let row = grid.len();
    let col = grid.first().map(|r| r.len()).unwrap_or_default();
    if row * col == 0 {
        return 0;
    }

    let mut dp = vec![vec![i32::MAX; col + 1]; row + 1];
    dp[row][col - 1] = 0;
    dp[row - 1][col] = 0;
    for r in (0..row).rev() {
        for c in (0..col).rev() {
            dp[r][c] = grid[r][c] + dp[r][c + 1].min(dp[r + 1][c]);
        }
    }
    dp[0][0]

    // dijkstra(grid, (row - 1, col - 1))
}

#[allow(unused)]
fn dijkstra(grid: &[&[i32]], goal: (usize, usize)) -> i32 {
    let mut dist = HashMap::new();
    for r in 0..=goal.0 {
        for c in 0..=goal.1 {
            dist.insert((r, c), i32::MAX);
        }
    }
    let mut heap = BinaryHeap::new();
    let start = Node {
        row: 0,
        col: 0,
        cost: grid[0][0],
    };
    heap.push(start);
    while let Some(Node { row, col, cost }) = heap.pop() {
        if (row, col) == goal {
            return cost;
        }
        if dist.get(&(row, col)).is_some_and(|&c| c < cost) {
            continue;
        }
        for (r, c) in step(row, col, goal) {
            let node = Node {
                row: r,
                col: c,
                cost: cost + grid[r][c],
            };
            if dist.get(&(r, c)).is_some_and(|&cost| cost > node.cost) {
                heap.push(node);
                dist.insert((r, c), node.cost);
            }
        }
    }
    0
}

fn step(row: usize, col: usize, goal: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    [(row + 1, col), (row, col + 1)]
        .into_iter()
        .filter(move |&p| p.0 <= goal.0 && p.1 <= goal.1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    row: usize,
    col: usize,
    cost: i32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_path_sum(&[&[1, 3, 1], &[1, 5, 1], &[4, 2, 1]]), 7);
        debug_assert_eq!(min_path_sum(&[&[1, 2, 3], &[4, 5, 6]]), 12);
    }

    #[test]
    fn test() {}
}
