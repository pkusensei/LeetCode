mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_array(pref: &[i32]) -> Vec<i32> {
    let mut prev = 0;
    let mut res = Vec::with_capacity(pref.len());
    for &p in pref.iter() {
        res.push(prev ^ p);
        prev = p;
    }
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
        assert_eq!(find_array(&[5, 2, 0, 3, 1]), [5, 7, 2, 3, 2]);
        assert_eq!(find_array(&[12]), [12]);
    }

    #[test]
    fn test() {}
}
