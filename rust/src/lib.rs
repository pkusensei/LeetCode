mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(s: String) -> i32 {
        let n = s.len();
        let [mut a, mut b] = [vec![], vec![]];
        while a.len() < n {
            a.extend_from_slice(b"01");
            b.extend_from_slice(b"10");
        }
        if a.len() > n {
            a.pop();
            b.pop();
        }
        s.bytes()
            .zip(a.into_iter().zip(b))
            .fold([0, 0], |[diffa, diffb], (bs, (ba, bb))| {
                [diffa + i32::from(bs != ba), diffb + i32::from(bs != bb)]
            })
            .into_iter()
            .min()
            .unwrap()
}

#[cfg(test)]
mod tests {

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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
