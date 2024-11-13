mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
    let mut res = vec![];
    for d in 1..10 {
        backtrack(n - 1, k, d, d, &mut res);
    }
    res
}

fn backtrack(n: i32, k: i32, acc: i32, digit: i32, res: &mut Vec<i32>) {
    if n == 0 {
        res.push(acc);
        return;
    }
    for d in 0i32..10 {
        if d.abs_diff(digit) == k as u32 {
            backtrack(n - 1, k, acc * 10 + d, d, res);
        }
    }
}

fn with_bfs(n: i32, k: i32) -> Vec<i32> {
    let mut queue: Vec<i32> = (1..10).collect();
    for _ in 1..n {
        let mut next = vec![];
        for num in queue {
            let tail = num % 10;
            for d in 0..10 {
                if tail.abs_diff(d) == k as u32 {
                    next.push(num * 10 + d);
                }
            }
        }
        queue = next;
    }
    queue
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(with_bfs(3, 7), [181, 292, 707, 818, 929]);
        sort_eq(
            with_bfs(2, 1),
            [
                10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98,
            ],
        );
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
