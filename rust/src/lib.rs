mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut memo = vec![vec![[[-1; 2]; 4]; cols]; rows];
    let mut res = 0;
    for (r, row) in (0..).zip(grid.iter()) {
        for (c, &v) in (0..).zip(row.iter()) {
            if v == 1 {
                for dir in 0..4 {
                    res = res.max(dfs(&grid, r, c, dir, 0, 1, &mut memo));
                }
            }
        }
    }
    res
}

const DIRS: [[i32; 2]; 4] = [[1, 1], [1, -1], [-1, -1], [-1, 1]];

fn dfs(
    grid: &[Vec<i32>],
    r: i32,
    c: i32,
    dir: usize,
    turned: usize,
    expect: i32,
    memo: &mut [Vec<[[i32; 2]; 4]>],
) -> i32 {
    let [rows, cols] = get_dimensions(grid).map(|v| v as i32);
    if !(0..rows).contains(&r) || !(0..cols).contains(&c) || grid[r as usize][c as usize] != expect
    {
        return 0;
    }
    if memo[r as usize][c as usize][dir][turned] > -1 {
        return memo[r as usize][c as usize][dir][turned];
    }
    let next_expect = if expect < 2 { 2 } else { 0 };
    let [dr, dc] = DIRS[dir];
    // stay in dir
    let mut res = dfs(grid, r + dr, c + dc, dir, turned, next_expect, memo);
    if turned == 0 {
        let next_dir = (1 + dir) % 4;
        let [dr, dc] = DIRS[next_dir];
        res = res.max(dfs(grid, r + dr, c + dc, next_dir, 1, next_expect, memo));
    }
    res += 1;
    memo[r as usize][c as usize][dir][turned] = res;
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
    fn basics() {
        assert_eq!(
            len_of_v_diagonal(vec![
                vec![2, 2, 1, 2, 2],
                vec![2, 0, 2, 2, 0],
                vec![2, 0, 1, 1, 0],
                vec![1, 0, 2, 2, 2],
                vec![2, 0, 0, 2, 2]
            ]),
            5
        );
        assert_eq!(
            len_of_v_diagonal(vec![
                vec![2, 2, 2, 2, 2],
                vec![2, 0, 2, 2, 0],
                vec![2, 0, 1, 1, 0],
                vec![1, 0, 2, 2, 2],
                vec![2, 0, 0, 2, 2]
            ]),
            4
        );
        assert_eq!(
            len_of_v_diagonal(vec![
                vec![1, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 0],
                vec![2, 0, 0, 0, 0],
                vec![0, 0, 2, 2, 2],
                vec![2, 0, 0, 2, 0]
            ]),
            5
        );
        assert_eq!(len_of_v_diagonal(vec![vec![1]]), 1);
    }

    #[test]
    fn test() {}
}
