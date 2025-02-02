mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn can_mouse_win(grid: &[&str], cat_jump: i32, mouse_jump: i32) -> bool {
    let grid: Vec<&[u8]> = grid.iter().map(|s| s.as_bytes()).collect();
    let [mut cat, mut mouse, mut food] = [[0, 0]; 3];
    for (r, row) in grid.iter().enumerate() {
        for (c, &val) in row.iter().enumerate() {
            match val {
                b'C' => cat = [r, c],
                b'F' => food = [r, c],
                b'M' => mouse = [r, c],
                _ => (),
            }
        }
    }
    dfs(
        &grid,
        cat_jump,
        mouse_jump,
        cat,
        mouse,
        food,
        100,
        &mut HashMap::new(),
    )
}

fn dfs(
    grid: &[&[u8]],
    cat_jump: i32,
    mouse_jump: i32,
    cat: Coord,
    mouse: Coord,
    food: Coord,
    turn: i32,
    memo: &mut HashMap<(Coord, Coord, i32), bool>,
) -> bool {
    const DIRS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];
    if turn <= 0 || cat == mouse || cat == food {
        return false;
    }
    if mouse == food {
        return true;
    }
    let key = (cat, mouse, turn);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    if turn & 1 == 0 {
        for [dr, dc] in DIRS {
            for step in 0..=mouse_jump {
                let nr = mouse[0] as i32 + dr * step;
                let nc = mouse[1] as i32 + dc * step;
                let Some(valid) = validate(grid, nr, nc) else {
                    break;
                };
                if dfs(grid, cat_jump, mouse_jump, cat, valid, food, turn - 1, memo) {
                    memo.insert(key, true);
                    return true;
                }
            }
        }
        memo.insert(key, false);
        false
    } else {
        for [dr, dc] in DIRS {
            for step in 0..=cat_jump {
                let nr = cat[0] as i32 + dr * step;
                let nc = cat[1] as i32 + dc * step;
                let Some(valid) = validate(grid, nr, nc) else {
                    break;
                };
                if !dfs(
                    grid,
                    cat_jump,
                    mouse_jump,
                    valid,
                    mouse,
                    food,
                    turn - 1,
                    memo,
                ) {
                    memo.insert(key, false);
                    return false;
                }
            }
        }
        memo.insert(key, true);
        true
    }
}

fn validate(grid: &[&[u8]], row: i32, col: i32) -> Option<Coord> {
    if row < 0 || col < 0 {
        None
    } else if grid
        .get(row as usize)
        .is_some_and(|r| r.get(col as usize).is_some_and(|&v| v != b'#'))
    {
        Some([row as usize, col as usize])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert!(can_mouse_win(&["####F", "#C...", "M...."], 1, 2));
        assert!(can_mouse_win(&["M.C...F"], 1, 4));
        assert!(!can_mouse_win(&["M.C...F"], 1, 3));
    }

    #[test]
    fn test() {}
}
