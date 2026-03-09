mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
    let n = graph.len();
    // degree[mouse][cat][turn]
    let mut degree = vec![vec![[0; 2]; n]; n];
    // for each (mouse_pos, cat_pos)
    for mouse in 0..n {
        for cat in 0..n {
            degree[mouse][cat][MOUSE] = graph[mouse].len();
            degree[mouse][cat][CAT] = graph[cat].iter().filter(|&&v| v != 0).count();
        }
    }
    // states[mouse][cat][turn]
    let mut states = vec![vec![[DRAW; 2]; n]; n];
    let mut queue = VecDeque::new();
    for node in 1..n {
        for turn in [MOUSE, CAT] {
            states[0][node][turn] = MOUSE; // Mouse reaches 0 and wins
            queue.push_back([0, node, turn]);
            states[node][node][turn] = CAT; // Both reach `node` and cat wins
            queue.push_back([node, node, turn]);
        }
    }
    while let Some([mouse, cat, turn]) = queue.pop_front() {
        let curr = states[mouse][cat][turn];
        for [prev_m, prev_c, prev_t] in prev_states(&graph, mouse, cat, turn) {
            if states[prev_m][prev_c][prev_t] != DRAW {
                continue;
            }
            degree[prev_m][prev_c][prev_t] -= 1;
            let mover_winnning =
                (curr == MOUSE && prev_t == MOUSE) || (curr == CAT && prev_t == CAT);
            if mover_winnning || degree[prev_m][prev_c][prev_t] == 0 {
                states[prev_m][prev_c][prev_t] = curr;
                queue.push_back([prev_m, prev_c, prev_t]);
            }
        }
    }
    match states[1][2][MOUSE] {
        MOUSE => 1,
        CAT => 2,
        _ => 0,
    }
}

const MOUSE: usize = 0;
const CAT: usize = 1;
const DRAW: usize = 2;

fn prev_states(graph: &[Vec<i32>], mouse: usize, cat: usize, turn: usize) -> Vec<[usize; 3]> {
    if turn == MOUSE {
        graph[cat]
            .iter()
            .filter_map(|&c| {
                if c > 0 {
                    Some([mouse, c as usize, CAT])
                } else {
                    None
                }
            })
            .collect()
    } else {
        graph[mouse]
            .iter()
            .map(|&m| [m as usize, cat, MOUSE])
            .collect()
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
    fn basics() {}

    #[test]
    fn test() {}
}
