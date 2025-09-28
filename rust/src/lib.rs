mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn zig_zag_arrays_2(n: i32, l: i32, r: i32) -> i32 {
    use matrix::Matrix;
    let width = (r + 1 - l) as usize;
    // Lower [..width], the increasing half
    // Upper [width..], the decreasing half
    let mut state = Matrix::with_default(2 * width, 1, 1);
    let mut transition = Matrix::new(2 * width, 2 * width);
    for prev in 0..width {
        for curr in 0..prev {
            // prev > curr => inc to dec
            transition.assign(prev, curr + width, 1);
        }
        for curr in 1 + prev..width {
            // prev < curr => dec to inc
            transition.assign(prev + width, curr, 1);
        }
    }
    transition = transition.pow(n - 1);
    // (2*w, 2*w) * (2*w, 1) => (2*w, 1)
    state = transition.mul(&state);
    (0..2 * width).fold(0, |acc, row| (acc + state.get(row, 0).unwrap_or(0) % M)) as i32
}

pub fn zig_zag_arrays_1(n: i32, l: i32, r: i32) -> i32 {
    let [n, low, high] = [n, l - l, r - l].map(|v| v as usize);
    let width = high + 1 - low;
    let mut dp_inc = vec![1; width];
    let mut dp_dec = vec![1; width];
    for _ in 1..n {
        let mut curr_inc = vec![0; width];
        let mut curr_dec = vec![0; width];
        for i in 1 + low..=high {
            curr_inc[i] = (dp_dec[i - 1] + curr_inc[i - 1]) % M;
        }
        for i in (low..high).rev() {
            curr_dec[i] = (dp_inc[1 + i] + curr_dec[1 + i]) % M;
        }
        dp_inc = curr_inc;
        dp_dec = curr_dec;
    }
    dp_inc
        .into_iter()
        .chain(dp_dec)
        .fold(0, |acc, v| (acc + v) % M) as i32
    // let [n, left, right] = [n, l, r].map(|v| v as usize);
    // let width = right - left + 1;
    // let mut memo = vec![vec![vec![-1; width]; 2]; n];
    // (left..=right)
    //     .map(|curr| {
    //         (dfs(n, left, right, 0, curr, &mut memo) + dfs(n, left, right, 1, curr, &mut memo)) % M
    //     })
    //     .fold(0, |acc, v| (acc + v) % M)
}

const M: i64 = 1_000_000_007;

// tle's
fn dfs(
    n: usize,
    left: usize,
    right: usize,
    dir: usize,
    curr: usize,
    memo: &mut [Vec<Vec<i64>>],
) -> i64 {
    if n == 1 {
        return 1;
    }
    if memo[n - 1][dir][curr - left] > -1 {
        return memo[n - 1][dir][curr - left];
    }
    let range = if dir == 1 {
        left..curr
    } else {
        1 + curr..1 + right
    };
    let res = range
        .map(|val| dfs(n - 1, left, right, 1 ^ dir, val, memo))
        .fold(0, |acc, v| (acc + v) % M);
    memo[n - 1][dir][curr - left] = res;
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
        assert_eq!(zig_zag_arrays_1(3, 1, 3), 10);
        assert_eq!(zig_zag_arrays_1(3, 4, 5), 2);

        assert_eq!(zig_zag_arrays_2(3, 4, 5), 2);
        assert_eq!(zig_zag_arrays_2(3, 1, 3), 10);
    }

    #[test]
    fn test() {}
}
