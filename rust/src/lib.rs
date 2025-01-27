mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn kth_smallest_path(destination: [i32; 2], k: i32) -> String {
    let [rows, cols] = [0, 1].map(|v| destination[v]);
    let mut res = dfs(cols, rows, k);
    res.reverse();
    String::from_utf8(res).unwrap()
}

fn dfs(hs: i32, vs: i32, k: i32) -> Vec<u8> {
    if hs == 0 {
        return vec![b'V'; vs as usize];
    }
    if vs == 0 {
        return vec![b'H'; hs as usize];
    }
    // Put 'H' here => it's possible to generate this many combos
    // Otherwise, pick 'V'
    let comb = n_choose_k(hs + vs - 1, hs - 1);
    if k <= comb {
        let mut res = dfs(hs - 1, vs, k);
        res.push(b'H');
        res
    } else {
        let mut res = dfs(hs, vs - 1, k - comb);
        res.push(b'V');
        res
    }
}

fn n_choose_k(n: i32, k: i32) -> i32 {
    let k = k.min(n - k);
    (0..k).fold(1, |acc, i| acc * (n - i) / (1 + i))
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

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

    #[test]
    fn basics() {
        assert_eq!(kth_smallest_path([2, 3], 1), "HHHVV");
        assert_eq!(kth_smallest_path([2, 3], 2), "HHVHV");
        assert_eq!(kth_smallest_path([2, 3], 3), "HHVVH");
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
