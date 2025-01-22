mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn tarjan(grid: &[&[i32]]) -> i32 {
    let ones: i32 = grid.iter().flat_map(|row| row.iter()).sum();
    if ones == 0 {
        return 0;
    }
    if ones == 1 {
        return 1;
    }
    for (y, row) in grid.iter().enumerate() {
        for (x, num) in row.iter().enumerate() {
            if *num == 0 {
                continue;
            }
            let (island, graph) = find_island([y, x], grid);
            if island.len() < ones as usize {
                return 0;
            }
            let ap = find_articulation_points(island, graph);
            if !ap.is_empty() {
                return 1;
            } else {
                return 2;
            }
        }
    }
    0
}

fn find_island([row, col]: Coord, grid: &[&[i32]]) -> (HashSet<Coord>, HashMap<Coord, Vec<Coord>>) {
    let mut queue = VecDeque::from([[row, col]]);
    let mut seen = HashSet::from([[row, col]]);
    let mut graph: HashMap<Coord, Vec<_>> = HashMap::new();
    while let Some(curr) = queue.pop_front() {
        let neighbors = neighbors(curr).filter(|&[row, col]| {
            grid.get(row)
                .is_some_and(|row| row.get(col).is_some_and(|&n| n == 1))
        });
        let v = graph.entry(curr).or_default();
        for n in neighbors {
            v.push(n);
            if seen.insert(n) {
                queue.push_back(n);
            }
        }
    }
    (seen, graph)
}

fn find_articulation_points(
    coords: HashSet<Coord>,
    graph: HashMap<Coord, Vec<Coord>>,
) -> HashSet<Coord> {
    let mut ap = HashSet::new();
    // since we are considering only one island
    if let Some(node) = coords.into_iter().next() {
        dfs(
            &graph,
            node,
            None,
            &mut HashMap::new(),
            &mut HashMap::new(),
            &mut ap,
            0,
        );
    }
    ap // should also consider isolated 1s
}

fn dfs(
    graph: &HashMap<Coord, Vec<Coord>>,
    curr: Coord,
    parent: Option<Coord>,
    lowest: &mut HashMap<Coord, i32>,
    disc_time: &mut HashMap<Coord, i32>,
    ap: &mut HashSet<Coord>,
    time: i32,
) {
    let mut children = 0;
    lowest.entry(curr).or_insert(time);
    disc_time.entry(curr).or_insert(time);

    for &neighbor in graph[&curr].iter() {
        if let Some(&v) = disc_time.get(&neighbor) {
            if parent.is_none() || parent.is_some_and(|p| p != neighbor) {
                lowest.insert(curr, lowest[&curr].min(v));
            }
        } else {
            children += 1;
            dfs(graph, neighbor, Some(curr), lowest, disc_time, ap, time + 1);
            lowest.insert(curr, lowest[&curr].min(lowest[&neighbor]));
            if parent.is_some() && lowest[&neighbor] >= disc_time[&curr] {
                ap.insert(curr);
            }
        }
    }
    if parent.is_none() && children > 1 {
        ap.insert(curr);
    }
}

pub fn brute_force(mut grid: Vec<Vec<i32>>) -> i32 {
    fn count(grid: &[Vec<i32>]) -> i32 {
        let [rows, cols] = get_dimensions(&grid);
        let mut seen = vec![vec![false; cols]; rows];
        let mut res = 0;
        for (row, r) in grid.iter().enumerate() {
            for (col, &v) in r.iter().enumerate() {
                if v == 1 && !seen[row][col] {
                    res += 1;
                    seen[row][col] = true;
                    let mut queue = VecDeque::from([[row, col]]);
                    while let Some([row, col]) = queue.pop_front() {
                        for [nr, nc] in neighbors([row, col]).filter(|&[nr, nc]| {
                            grid.get(nr)
                                .is_some_and(|r| r.get(nc).is_some_and(|&v| v == 1))
                        }) {
                            if !seen[nr][nc] {
                                seen[nr][nc] = true;
                                queue.push_back([nr, nc]);
                            }
                        }
                    }
                }
            }
        }
        res
    }

    let [rows, cols] = get_dimensions(&grid);
    if count(&grid) != 1 {
        return 0;
    }
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != 1 {
                continue;
            }
            grid[r][c] = 0;
            if count(&grid) != 1 {
                return 1;
            }
            grid[r][c] = 1;
        }
    }
    2
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
