mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_kth_number(n: i32, k: i32) -> i32 {
    let (mut res, mut k) = (1, i64::from(k - 1));
    let n = i64::from(n);
    while k > 0 {
        let steps = calc_steps(n, res, res + 1);
        if steps <= k {
            // skip the whole tree rooted by res
            // and jump to (res+1)
            k -= steps;
            res += 1;
        } else {
            k -= 1;
            res *= 10;
        }
    }
    res as _
}

fn calc_steps(n: i64, mut n1: i64, mut n2: i64) -> i64 {
    let mut steps = 0;
    while n1 <= n {
        // n2>n  ==> n2 is out side of path n1->n
        // n2<=n ==> we want to reach n2 first
        steps += (n + 1).min(n2) - n1;
        n1 *= 10;
        n2 *= 10;
    }
    steps
}

fn dfs(curr: i64, n: i64, res: &mut i32, count: &mut i32, k: i32) {
    if curr > n {
        return;
    }
    *count += 1;
    if *count == k {
        *res = curr as i32;
        return;
    }
    let next = curr * 10;
    for i in 0..=9 {
        dfs(next + i, n, res, count, k);
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_kth_number(13, 2), 10);
        debug_assert_eq!(find_kth_number(1, 1), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(find_kth_number(13, 9), 5);
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
}
