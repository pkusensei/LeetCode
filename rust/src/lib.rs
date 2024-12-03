mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_decomposition(text: &str) -> i32 {
    solve(text)
}

fn solve(text: &str) -> i32 {
    let n = text.len();
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut len = 1;
            while len <= n / 2 {
                if text.ends_with(&text[..len]) {
                    return 2 + solve(&text[len..n - len]);
                }
                len += 1;
            }
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(longest_decomposition("ghiabcdefhelloadamhelloabcdefghi"), 7);
        assert_eq!(longest_decomposition("merchant"), 1);
        assert_eq!(longest_decomposition("antaprezatepzapreanta"), 11);
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
