mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn min_max_waiting_time(demand: &[i32], fuel: [i32; 2]) -> i32 {
    if fuel[0] < demand[0] && fuel[1] < demand[0] {
        return -1;
    }
    let mut memo = HashMap::new();
    dfs(
        &demand,
        State {
            idx: 0,
            fuel0: fuel[0],
            fuel1: fuel[1],
            wait0: 0,
            wait1: 0,
        },
        &mut memo,
    )
    .1
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    idx: usize,
    fuel0: i32,
    fuel1: i32,
    wait0: i32,
    wait1: i32,
}

// (number of cars, max wait)
fn dfs(demand: &[i32], state: State, memo: &mut HashMap<State, (i32, i32)>) -> (i32, i32) {
    let State {
        idx,
        fuel0,
        fuel1,
        wait0,
        wait1,
    } = state;
    if idx >= demand.len() {
        return (0, 0);
    }
    if let Some(&v) = memo.get(&state) {
        return v;
    }
    let mut res_count = [0, 0];
    let mut res_wait = [0, 0];
    if demand[idx] <= fuel0 {
        let (a, b) = dfs(
            demand,
            State {
                idx: 1 + idx,
                fuel0: fuel0 - demand[idx],
                fuel1,
                wait0: demand[idx],
                wait1: (wait1 - wait0).max(0),
            },
            memo,
        );
        res_count[0] = 1 + a;
        res_wait[0] = wait0.max(b);
    }
    if demand[idx] <= fuel1 {
        let (a, b) = dfs(
            demand,
            State {
                idx: 1 + idx,
                fuel0,
                fuel1: fuel1 - demand[idx],
                wait0: (wait0 - wait1).max(0),
                wait1: demand[idx],
            },
            memo,
        );
        res_count[1] = 1 + a;
        res_wait[1] = wait1.max(b);
    }
    let res = match res_count[0].cmp(&res_count[1]) {
        std::cmp::Ordering::Less => (res_count[1], res_wait[1]),
        std::cmp::Ordering::Equal => (res_count[0], res_wait[0].min(res_wait[1])),
        std::cmp::Ordering::Greater => (res_count[0], res_wait[0]),
    };
    memo.insert(state, res);
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
        assert_eq!(min_max_waiting_time(&[6, 8, 4, 6, 5], [16, 13]), 6);
    }

    #[test]
    fn test() {}
}
