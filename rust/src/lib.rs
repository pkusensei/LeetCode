mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::{HashMap, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn max_moves(kx: i32, ky: i32, positions: &[[i32; 2]]) -> i32 {
    let n = positions.len();
    let mut all_pos = Vec::with_capacity(1 + n);
    let mut pos_idx = HashMap::with_capacity(1 + n);
    for p in positions.iter() {
        let p = [p[0], p[1]];
        pos_idx.insert(p, all_pos.len());
        all_pos.push(p);
    }
    pos_idx.insert([kx, ky], all_pos.len());
    all_pos.push([kx, ky]);
    let dists = bfs(&all_pos, &pos_idx);
    dfs(&dists, n, 0, &mut vec![vec![-1; 1 << n]; 1 + n])
}

fn dfs(dists: &[Vec<i32>], curr: usize, mask: usize, memo: &mut [Vec<i32>]) -> i32 {
    let n = dists.len();
    if n - 1 == mask.count_ones() as usize {
        return 0;
    }
    if memo[curr][mask] > -1 {
        return memo[curr][mask];
    }
    // mask carries info of turn number
    let turn = mask.count_ones() & 1 == 0;
    let mut res = if turn { 0 } else { i32::MAX };
    for (next, d) in dists[curr].iter().enumerate().take(n - 1) {
        if (1 << next) & mask > 0 {
            continue;
        }
        let curr = d + dfs(dists, next, mask | (1 << next), memo);
        res = if turn { res.max(curr) } else { res.min(curr) };
    }
    memo[curr][mask] = res;
    res
}

const N: i32 = 50;

fn bfs(pos: &[[i32; 2]], pos_idx: &HashMap<[i32; 2], usize>) -> Vec<Vec<i32>> {
    let n = pos.len();
    let mut dists = vec![vec![i32::MAX; n]; n];
    for i in 0..n {
        dists[i][i] = 0;
    }
    for start in 0..n {
        let mut queue = VecDeque::from([(pos[start], 0)]);
        let mut seen = [[false; N as usize]; N as usize];
        seen[pos[start][0] as usize][pos[start][1] as usize] = true;
        while let Some((node, step)) = queue.pop_front() {
            if let Some(&i) = pos_idx.get(&node) {
                if dists[start][i] == i32::MAX {
                    dists[start][i] = step;
                }
            }
            for [nx, ny] in next(node[0], node[1]) {
                if !seen[nx as usize][ny as usize] {
                    seen[nx as usize][ny as usize] = true;
                    queue.push_back(([nx, ny], 1 + step));
                }
            }
        }
    }
    dists
}

fn next(x: i32, y: i32) -> impl Iterator<Item = [i32; 2]> {
    [
        [2, 1],
        [2, -1],
        [-2, 1],
        [-2, -1],
        [1, 2],
        [1, -2],
        [-1, 2],
        [-1, -2],
    ]
    .into_iter()
    .filter_map(move |[dx, dy]| {
        let nx = x + dx;
        let ny = y + dy;
        if (0..N).contains(&nx) && (0..N).contains(&ny) {
            Some([nx, ny])
        } else {
            None
        }
    })
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
    fn basics() {
        assert_eq!(max_moves(1, 1, &[[0, 0]]), 4);
        assert_eq!(max_moves(0, 2, &[[1, 1], [2, 2], [3, 3]]), 8);
        assert_eq!(max_moves(0, 0, &[[1, 2], [2, 4]]), 3);
    }

    #[test]
    fn test() {}
}
