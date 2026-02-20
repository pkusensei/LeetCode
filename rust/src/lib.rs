mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
    const DIRS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
    const MAX_TURN: i32 = 2;
    let mut curr = [r_start, c_start];
    let mut res = vec![curr.to_vec()];
    let mut idx = 0;
    let mut max_len = 1;
    let mut running = 0;
    let mut turns = 0;
    while res.len() < (rows * cols) as usize {
        curr[0] += DIRS[idx][0];
        curr[1] += DIRS[idx][1];
        if (0..rows).contains(&curr[0]) && (0..cols).contains(&curr[1]) {
            res.push(curr.to_vec());
        }
        running += 1;
        if running == max_len {
            running = 0;
            turns += 1;
            idx = (1 + idx) % 4;
            if turns == MAX_TURN {
                max_len += 1;
                turns = 0;
            }
        }
    }
    res
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
