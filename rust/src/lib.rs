mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn create_sorted_array(instructions: &[i32]) -> i32 {
    let max = instructions.iter().copied().max().unwrap() as usize;
    let mut ft = vec![0; 1 + max];
    let mut res = 0;
    for (i, &num) in instructions.iter().enumerate() {
        res += query(&mut ft, num as usize - 1).min(i as i32 - query(&mut ft, num as usize));
        res %= 1_000_000_007;
        update(&mut ft, num as _, max);
    }
    res
}

fn update(ft: &mut [i32], mut x: usize, max: usize) {
    while x <= max {
        ft[x] += 1;
        x += x & x.wrapping_neg();
    }
}

fn query(ft: &[i32], mut x: usize) -> i32 {
    let mut res = 0;
    while x > 0 {
        res += ft[x];
        x -= x & x.wrapping_neg()
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
    fn basics() {
        assert_eq!(create_sorted_array(&[1, 5, 6, 2]), 1);
        assert_eq!(create_sorted_array(&[1, 2, 3, 6, 5, 4]), 3);
        assert_eq!(create_sorted_array(&[1, 3, 3, 3, 2, 4, 2, 1, 2]), 4);
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
