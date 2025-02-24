mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn count_combinations(pieces: &[&str], positions: &[[i32; 2]]) -> i32 {
    let mut all_dirs = vec![];
    // let positions: Vec<_> = positions.iter().map(|p| [p[0], p[1]]).collect();
    backtrack(pieces, &mut vec![], &mut all_dirs);
    let mut set = HashSet::new();
    for dirs in all_dirs {
        dfs(positions, &dirs, (1 << pieces.len()) - 1, &mut set); // exclude origin
    }
    set.len() as i32
}

// rook
const DIRS1: [[i32; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];
// bishop
const DIRS2: [[i32; 2]; 4] = [[1, 1], [1, -1], [-1, 1], [-1, -1]];
// queen
const DIRS3: [[i32; 2]; 8] = [
    [0, 1],
    [0, -1],
    [1, 0],
    [-1, 0],
    [1, 1],
    [1, -1],
    [-1, 1],
    [-1, -1],
];

fn backtrack(pieces: &[&str], curr: &mut Vec<[i32; 2]>, dirs: &mut Vec<Vec<[i32; 2]>>) {
    match pieces {
        [] => dirs.push(curr.clone()),
        [head, tail @ ..] => {
            let ds: &[[i32; 2]] = match *head {
                "rook" => &DIRS1,
                "bishop" => &DIRS2,
                _ => &DIRS3,
            };
            for d in ds {
                curr.push(*d);
                backtrack(tail, curr, dirs);
                curr.pop();
            }
        }
    }
}

fn dfs(positions: &[[i32; 2]], dirs: &[[i32; 2]], mask: i32, set: &mut HashSet<Vec<[i32; 2]>>) {
    if mask == 0 {
        return;
    }
    set.insert(positions.to_vec());
    for active in 0..1 << dirs.len() {
        if mask & active != active {
            continue;
        }
        let mut next = positions.to_vec();
        let next_mask = mask ^ active;
        for i in 0..next.len() {
            next[i][0] += dirs[i][0] * ((next_mask >> i) & 1);
            next[i][1] += dirs[i][1] * ((next_mask >> i) & 1);
        }
        if next.iter().copied().collect::<HashSet<_>>().len() < next.len() {
            continue;
        }
        if next
            .iter()
            .any(|p| !(1..=8).contains(&p[0]) || !(1..=8).contains(&p[1]))
        {
            continue;
        }
        dfs(&next, dirs, next_mask, set);
    }
}

// fn dfs(positions: &[[i32; 2]], dirs: &[[i32; 2]]) -> i32 {
//     for (i, a) in positions.iter().enumerate() {
//         if !(1..=8).contains(&a[0]) || !(1..=8).contains(&a[1]) {
//             return 0;
//         }
//         for b in positions.iter().skip(1 + i) {
//             if a == b {
//                 return 0;
//             }
//         }
//     }
//     let mut next = vec![];
//     for (pos, dir) in positions.iter().zip(dirs) {
//         next.push([pos[0] + dir[0], pos[1] + dir[1]]);
//     }
//     1 + dfs(&next, dirs)
// }

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
        assert_eq!(count_combinations(&["rook"], &[[1, 1]]), 15);
        assert_eq!(count_combinations(&["queen"], &[[1, 1]]), 22);
        assert_eq!(count_combinations(&["bishop"], &[[4, 3]]), 12);
    }

    #[test]
    fn test() {
        assert_eq!(
            count_combinations(&["rook", "rook"], &[[1, 1], [8, 8]]),
            223
        );
    }
}
