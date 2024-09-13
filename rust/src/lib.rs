mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn lexical_order(n: i32) -> Vec<i32> {
    let mut res = Vec::with_capacity(n as usize);
    // for curr in 1..=9 {
    //     dfs(curr, n, &mut res)
    // }
    let mut num = 1;
    res.push(num);
    loop {
        if 10 * num <= n {
            // 1, 10, 100 ...
            num *= 10;
            res.push(num);
            continue;
        }
        if num < n && num % 10 < 9 {
            // 10, 11, 12, ...19
            num += 1;
            res.push(num);
            continue;
        }
        while {
            num /= 10;
            num % 10 == 9
        } {}
        if num == 0 {
            break;
        }
        num += 1;
        res.push(num)
    }
    res
}

fn dfs(curr: i32, n: i32, res: &mut Vec<i32>) {
    if curr > n {
        return;
    }
    res.push(curr);
    for i in 0..=9 {
        dfs(10 * curr + i, n, res)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            lexical_order(13),
            [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
        );
        debug_assert_eq!(lexical_order(2), [1, 2]);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
