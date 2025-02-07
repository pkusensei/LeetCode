mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reinitialize_permutation(n: i32) -> i32 {
    let perm: Vec<_> = (0..n).collect();
    let mut curr = perm.clone();
    for count in 0..n {
        let mut next = Vec::with_capacity(n as usize);
        for i in 0..n as usize {
            if i & 1 == 0 {
                next.push(curr[i / 2]);
            } else {
                next.push(curr[n as usize / 2 + (i - 1) / 2]);
            }
        }
        curr = next;
        if curr == perm {
            return count;
        }
    }
    -1
}

pub const fn track_idx(n: i32) -> i32 {
    let mut res = 0;
    let mut idx = 1;
    while res == 0 || idx > 1 {
        // if idx < n / 2 {
        //     idx *= 2;
        // } else {
        //     idx = 2 * idx - (n - 1);
        // }
        idx = 2 * idx % (n - 1);
        res += 1;
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
        assert_eq!(track_idx(2), 1);
        assert_eq!(track_idx(4), 2);
        assert_eq!(track_idx(6), 4);
    }

    #[test]
    fn test() {}
}
