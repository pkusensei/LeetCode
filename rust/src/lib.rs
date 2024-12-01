mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    dfs(&arr, 0, n - 1, &mut vec![vec![-1; n]; n])
}

fn dfs(arr: &[i32], start: usize, end: usize, dp: &mut [Vec<i32>]) -> i32 {
    if start >= end {
        return 0;
    }
    if dp[start][end] > -1 {
        return dp[start][end];
    }
    let mut res = i32::MAX;
    for i in start..end {
        res = res.min(
            arr[start..=i].iter().max().unwrap_or(&0) * arr[1 + i..=end].iter().max().unwrap_or(&0)
                + dfs(arr, start, i, dp)
                + dfs(arr, 1 + i, end, dp),
        );
    }
    dp[start][end] = res;
    res
}

fn iterative(mut arr: Vec<i32>) -> i32 {
    let mut res = 0;
    while let Some((i, w)) = arr
        .windows(2)
        .enumerate()
        .min_by_key(|&(_i, w)| w[0] * w[1])
    {
        // Smaller values should always be used first
        // so that bigger values can be evaluated for less times
        // Thus, find smallest pair, put its product in result,
        // and discard the smaller one of the two
        res += w[0] * w[1];
        arr.remove(if w[0] < w[1] { i } else { 1 + i });
    }
    res
}

fn with_stack(arr: Vec<i32>) -> i32 {
    let mut stack = vec![];
    stack.push(i32::MAX);
    let mut res = 0;
    for &num in arr.iter() {
        // Remove all local minimums
        // e.g. [4, 2] with 3 coming in, remove 2
        // add in the smaller product, 2*3, into result
        while stack.last().is_some_and(|&v| v <= num) {
            let mid = stack.pop().unwrap();
            res += mid * stack.last().unwrap_or(&mid).min(&num);
        }
        stack.push(num);
    }
    // Evaluate all nums kept in stack, except sentinal i32::MAX
    res += stack.windows(2).skip(1).map(|w| w[0] * w[1]).sum::<i32>();
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(with_stack(vec![6, 2, 4]), 32);
        debug_assert_eq!(with_stack(vec![4, 11]), 44);
    }

    #[test]
    fn test() {}

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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
