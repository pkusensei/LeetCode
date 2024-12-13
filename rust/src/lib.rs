mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_length(arr: &[&str]) -> i32 {
    dfs(arr, 0) as i32
}

fn dfs(arr: &[&str], curr: u32) -> u32 {
    match arr {
        [] => curr.count_ones(),
        [head, tail @ ..] => {
            let mut res = curr.count_ones();
            if let Some(mask) = unique_mask(head) {
                if curr & mask == 0 {
                    res = res.max(dfs(tail, curr | mask))
                }
            }
            res = res.max(dfs(tail, curr));
            res
        }
    }
}

fn unique_mask(s: &str) -> Option<u32> {
    let mut res = 0;
    for b in s.bytes() {
        let n = 1 << (b - b'a');
        if res & n != 0 {
            return None;
        }
        res |= n;
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_length(&["un", "iq", "ue"]), 4);
        assert_eq!(max_length(&["cha", "r", "act", "ers"]), 6);
        assert_eq!(max_length(&["abcdefghijklmnopqrstuvwxyz"]), 26);
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
