mod dsu;
mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn min_push_box(grid: &[&[char]]) -> i32 {
    let (walls, [carton, start, target], borders) = build(grid);
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    for pos in immediate(carton, borders) {
        if !walls.contains(&pos) && reachable(&walls, carton, start, pos, borders) {
            queue.push_back((carton, pos, 0));
            seen.insert([carton, pos]);
        }
    }
    while let Some((carton, pos, push)) = queue.pop_front() {
        if carton == target {
            return push;
        }
        let dx = carton[0] - pos[0];
        let dy = carton[1] - pos[1];
        let new_carton = [carton[0] + dx, carton[1] + dy];
        if inside(new_carton, borders) && !walls.contains(&new_carton) {
            for new_pos in immediate(new_carton, borders) {
                if !walls.contains(&new_pos)
                    && reachable(&walls, new_carton, pos, new_pos, borders)
                    && seen.insert([new_carton, new_pos])
                {
                    queue.push_back((new_carton, new_pos, 1 + push));
                }
            }
        }
    }
    -1
}

fn reachable(
    walls: &HashSet<[i8; 2]>,
    carton: [i8; 2],
    start: [i8; 2],
    goal: [i8; 2],
    borders: [i8; 2],
) -> bool {
    let mut queue = VecDeque::from([start]);
    let mut seen = HashSet::from([start]);
    while let Some(node) = queue.pop_front() {
        if goal == node {
            return true;
        }
        for neighbor in immediate(node, borders) {
            if !walls.contains(&neighbor) && neighbor != carton && seen.insert(neighbor) {
                queue.push_back(neighbor);
            }
        }
    }
    false
}

fn immediate([x, y]: [i8; 2], borders: [i8; 2]) -> impl Iterator<Item = [i8; 2]> {
    const DELTAS: [[i8; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];
    DELTAS
        .map(|[dx, dy]| [x + dx, y + dy])
        .into_iter()
        .filter(move |&p| inside(p, borders))
}

fn inside([x, y]: [i8; 2], [xmax, ymax]: [i8; 2]) -> bool {
    (0..xmax).contains(&x) && (0..ymax).contains(&y)
}

fn build(grid: &[&[char]]) -> (HashSet<[i8; 2]>, [[i8; 2]; 3], [i8; 2]) {
    let [mut start, mut target, mut carton] = [[0, 0]; 3];
    let (rows, cols) = get_dimensions(grid);
    let mut walls = HashSet::new();
    for (row, r) in grid.iter().enumerate() {
        for (col, &ch) in r.iter().enumerate() {
            let curr = [col as i8, row as i8];
            match ch {
                '#' => {
                    walls.insert(curr);
                }
                'B' => carton = curr,
                'S' => start = curr,
                'T' => target = curr,
                _ => (),
            }
        }
    }
    (walls, [carton, start, target], [cols as i8, rows as i8])
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            min_push_box(&[
                &['#', '#', '#', '#', '#', '#'],
                &['#', 'T', '#', '#', '#', '#'],
                &['#', '.', '.', 'B', '.', '#'],
                &['#', '.', '#', '#', '.', '#'],
                &['#', '.', '.', '.', 'S', '#'],
                &['#', '#', '#', '#', '#', '#']
            ]),
            3
        );
        assert_eq!(
            min_push_box(&[
                &['#', '#', '#', '#', '#', '#'],
                &['#', 'T', '#', '#', '#', '#'],
                &['#', '.', '.', 'B', '.', '#'],
                &['#', '#', '#', '#', '.', '#'],
                &['#', '.', '.', '.', 'S', '#'],
                &['#', '#', '#', '#', '#', '#']
            ]),
            -1
        );
        assert_eq!(
            min_push_box(&[
                &['#', '#', '#', '#', '#', '#'],
                &['#', 'T', '.', '.', '#', '#'],
                &['#', '.', '#', 'B', '.', '#'],
                &['#', '.', '.', '.', '.', '#'],
                &['#', '.', '.', '.', 'S', '#'],
                &['#', '#', '#', '#', '#', '#']
            ]),
            5
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            min_push_box(&[
                &['#', '.', '.', '#', '#', '#', '#', '#'],
                &['#', '.', '.', 'T', '#', '.', '.', '#'],
                &['#', '.', '.', '.', '#', 'B', '.', '#'],
                &['#', '.', '.', '.', '.', '.', '.', '#'],
                &['#', '.', '.', '.', '#', '.', 'S', '#'],
                &['#', '.', '.', '#', '#', '#', '#', '#']
            ]),
            7
        );
    }

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
