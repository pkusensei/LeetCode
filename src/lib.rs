use std::collections::{HashMap, HashSet, VecDeque};

pub fn min_days(grid: &[&[i32]]) -> i32 {
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
            let (island, graph) = find_island((y, x), grid);
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

type Coord = (usize, usize);

fn find_island((row, col): Coord, grid: &[&[i32]]) -> (HashSet<Coord>, HashMap<Coord, Vec<Coord>>) {
    let mut queue = VecDeque::from([(row, col)]);
    let mut seen = HashSet::from([(row, col)]);
    let mut graph: HashMap<Coord, Vec<_>> = HashMap::new();
    while let Some(curr) = queue.pop_front() {
        let neighbors = neighbors(curr).filter(|&(row, col)| {
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

fn neighbors((a, b): Coord) -> impl Iterator<Item = Coord> {
    [
        (a.saturating_sub(1), b),
        (a + 1, b),
        (a, b.saturating_sub(1)),
        (a, b + 1),
    ]
    .into_iter()
    .filter(move |p| *p != (a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_days(&[&[0, 1, 1, 0], &[0, 1, 1, 0], &[0, 0, 0, 0]]), 2);
        debug_assert_eq!(min_days(&[&[1, 1]]), 2);
        debug_assert_eq!(
            min_days(&[
                &[1, 1, 0, 1, 1],
                &[1, 1, 1, 1, 1],
                &[1, 1, 0, 1, 1],
                &[1, 1, 0, 1, 1]
            ]),
            1
        );
        debug_assert_eq!(
            min_days(&[
                &[1, 1, 0, 1, 1],
                &[1, 1, 1, 1, 1],
                &[1, 1, 0, 1, 1],
                &[1, 1, 1, 1, 1]
            ]),
            2
        );
        debug_assert_eq!(min_days(&[&[1, 1], &[1, 0]]), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(min_days(&[&[0, 0, 0], &[0, 1, 0], &[0, 0, 0]]), 1);
    }
}
