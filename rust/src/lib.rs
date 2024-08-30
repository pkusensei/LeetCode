mod helper;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn modified_graph_edges(
    n: i32,
    edges: Vec<[i32; 3]>,
    source: i32,
    destination: i32,
    target: i32,
) -> Vec<[i32; 3]> {
    solve1(
        n as usize,
        edges,
        source as usize,
        destination as usize,
        target,
    )
}

const INF: i32 = 2 * 10i32.pow(9);

fn solve1(
    n: usize,
    mut edges: Vec<[i32; 3]>,
    source: usize,
    destination: usize,
    target: i32,
) -> Vec<[i32; 3]> {
    let target = i64::from(target);
    let curr = dijkstra1(n, &edges, source, destination);
    if curr < target {
        return vec![];
    }
    let mut is_match = curr == target;
    for idx in 0..edges.len() {
        if edges[idx][2] > 0 {
            continue;
        }
        edges[idx][2] = if is_match { INF as _ } else { 1 };
        if !is_match {
            let dist = dijkstra1(n, &edges, source, destination);
            if dist <= target {
                is_match = true;
                edges[idx][2] += (target - dist) as i32;
            }
        }
    }
    if is_match {
        edges
    } else {
        vec![]
    }
}

fn dijkstra1(n: usize, edges: &[[i32; 3]], source: usize, destination: usize) -> i64 {
    let mut adj = vec![vec![i64::from(INF); n]; n];
    for edge in edges.iter().filter(|e| e[2] > 0) {
        adj[edge[0] as usize][edge[1] as usize] = edge[2].into();
        adj[edge[1] as usize][edge[0] as usize] = edge[2].into();
    }
    let mut dist = vec![i64::from(INF); n];
    dist[source] = 0;
    let mut visited = vec![false; n];

    for _ in 0..n {
        let nearest_unvisited = (0..n)
            .filter(|&node| !visited[node])
            .min_by_key(|&node| dist[node])
            .unwrap();
        visited[nearest_unvisited] = true;
        for node in 0..n {
            dist[node] = dist[node].min(dist[nearest_unvisited] + adj[nearest_unvisited][node]);
        }
    }

    dist[destination]
}

fn solve2(
    n: usize,
    mut edges: Vec<[i32; 3]>,
    source: usize,
    destination: usize,
    target: i32,
) -> Vec<[i32; 3]> {
    let mut graph = vec![vec![]; n];
    for edge in edges.iter().filter(|e| e[2] > 0) {
        graph[edge[0] as usize].push((edge[1] as usize, edge[2]));
        graph[edge[1] as usize].push((edge[0] as usize, edge[2]));
    }
    let curr = dijkstra2(n, source, destination, &graph);
    if curr < target {
        return vec![];
    }
    let mut is_match = curr == target;

    for edge in edges.iter_mut() {
        if edge[2] > 0 {
            continue;
        }
        edge[2] = if is_match { INF } else { 1 };
        graph[edge[0] as usize].push((edge[1] as usize, edge[2]));
        graph[edge[1] as usize].push((edge[0] as usize, edge[2]));

        if !is_match {
            let dist = dijkstra2(n, source, destination, &graph);
            if dist <= target {
                is_match = true;
                edge[2] += target - dist;
            }
        }
    }

    if is_match {
        edges
    } else {
        vec![]
    }
}

fn dijkstra2(n: usize, source: usize, destination: usize, graph: &[Vec<(usize, i32)>]) -> i32 {
    let mut dist = vec![INF; n];
    dist[source] = 0;
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), source));

    while let Some((Reverse(d), node)) = queue.pop() {
        if dist[node] < d {
            continue;
        }

        for &(neighbor, weight) in graph[node].iter() {
            let nd = d + weight;
            if nd < dist[neighbor] {
                dist[neighbor] = nd;
                queue.push((Reverse(nd), neighbor));
            }
        }
    }
    dist[destination]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(
        //     modified_graph_edges(
        //         5,
        //         vec![[4, 1, -1], [2, 0, -1], [0, 3, -1], [4, 3, -1]],
        //         0,
        //         1,
        //         5
        //     ),
        //     [[4, 1, 1], [2, 0, 1], [0, 3, 3], [4, 3, 1]]
        // );
        debug_assert!(modified_graph_edges(3, vec![[0, 1, -1], [0, 2, 5]], 0, 2, 6).is_empty());
        debug_assert_eq!(
            modified_graph_edges(
                4,
                vec![[1, 0, 4], [1, 2, 3], [2, 3, 5], [0, 3, -1]],
                0,
                2,
                6
            ),
            [[1, 0, 4], [1, 2, 3], [2, 3, 5], [0, 3, 1]]
        );
    }

    #[test]
    fn test() {}
}
