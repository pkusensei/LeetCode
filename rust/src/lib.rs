mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
    let mut res = 'a';
    let mut time = 0;
    for (i, ch) in keys_pressed.char_indices() {
        if i == 0 {
            res = ch;
            time = release_times[0];
        } else {
            let t = release_times[i] - release_times[i - 1];
            match t.cmp(&time) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Equal => res = res.max(ch),
                std::cmp::Ordering::Greater => {
                    time = t;
                    res = ch;
                }
            }
        }
    }
    res
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
    fn basics() {}

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
