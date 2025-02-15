mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn color_the_grid(m: i32, n: i32) -> i32 {
    dfs(
        m,
        n,
        0,
        0,
        0,
        0,
        &mut vec![vec![-1; 1 << (2 * m)]; n as usize],
    )
}

fn dfs(m: i32, n: i32, prev_col: i32, row: i32, col: i32, mask: i32, memo: &mut [Vec<i32>]) -> i32 {
    if col == n {
        return 1;
    }
    if row == m {
        return dfs(m, n, mask, 0, 1 + col, 0, memo);
    }
    if row == 0 && memo[col as usize][prev_col as usize] > -1 {
        // row==0 => starting a new col
        // But this prev_mask => col combo has been seen
        return memo[col as usize][prev_col as usize];
    }
    // prev_mask  01 01 01 01 01
    // row number 0  1  2  3  4
    let left = (prev_col >> ((m - row - 1) * 2)) & 0b11;
    let up = if row == 0 { 0 } else { mask & 0b11 };
    let mut res = 0;
    for color in 1..=3 {
        if color != left && color != up {
            res += dfs(m, n, prev_col, 1 + row, col, (mask << 2) | color, memo);
            res %= 1_000_000_007;
        }
    }
    if row == 0 {
        // Recursion works backwards, i.e
        // Row m-1 is calculated first, then m-2, .. down to 0
        // When row==0, this whole col is done.
        memo[col as usize][prev_col as usize] = res;
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
        assert_eq!(color_the_grid(1, 1), 3);
        assert_eq!(color_the_grid(1, 2), 6);
        assert_eq!(color_the_grid(5, 5), 580986);
    }

    #[test]
    fn test() {}
}
