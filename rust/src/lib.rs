mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn cat_mouse_game(graph: &[&[i32]]) -> i32 {
    const DRAW: i32 = 0;
    const MOUSE: i32 = 1;
    const CAT: i32 = 2;
    let n = graph.len();

    let mut color = vec![vec![vec![0; 2]; n]; n];
    let mut degree = vec![vec![vec![0; 2]; n]; n];
    for mouse in 0..n {
        for cat in 0..n {
            degree[mouse][cat][1] = graph[mouse].len() as i32;
            degree[mouse][cat][0] = graph[cat].len() as i32;
            degree[mouse][cat][0] -= graph[cat].iter().filter(|&&v| v == 0).count() as i32;
        }
    }

    let mut queue = VecDeque::new();
    for node in 0..n {
        for turn in 0..=1 {
            color[0][node][turn] = MOUSE;
            queue.push_back((0, node, turn, MOUSE));
            if node > 0 {
                color[node][node][turn] = CAT;
                queue.push_back((node, node, turn, CAT));
            }
        }
    }
    // mouse location, cat location, turn, state
    while let Some((m, c, t, s)) = queue.pop_front() {
        for [m2, c2, t2] in parents(graph, m, c, t) {
            if color[m2][c2][t2] == DRAW {
                if t2 == 1 && s == MOUSE || t2 == 0 && s == CAT {
                    color[m2][c2][t2] = s;
                    queue.push_back((m2, c2, t2, s));
                } else {
                    degree[m2][c2][t2] -= 1;
                    if degree[m2][c2][t2] == 0 {
                        let temp = if t2 == 1 { CAT } else { MOUSE };
                        color[m2][c2][t2] = temp;
                        queue.push_back((m2, c2, t2, temp));
                    }
                }
            }
        }
    }
    color[1][2][1]
}

fn parents(graph: &[&[i32]], mouse: usize, cat: usize, turn: usize) -> Vec<[usize; 3]> {
    if turn == 0 {
        graph[mouse]
            .iter()
            .map(|&v| [v as usize, cat, 1 - turn])
            .collect()
    } else {
        graph[cat]
            .iter()
            .filter(|&&v| v > 0)
            .map(|&v| [mouse, v as usize, 1 - turn])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            cat_mouse_game(&[&[2, 5], &[3], &[0, 4, 5], &[1, 4, 5], &[2, 3], &[0, 2, 3]]),
            0
        );
        debug_assert_eq!(cat_mouse_game(&[&[1, 3], &[0], &[3], &[0, 2]]), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            cat_mouse_game(&[&[2, 3], &[3, 4], &[0, 4], &[0, 1], &[1, 2]]),
            1
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
