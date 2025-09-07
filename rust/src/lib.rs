mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_binary_palindromes(n: i64) -> i32 {
    if n < 1 {
        return 1;
    }
    let bits = {
        let mut bits = vec![];
        let mut temp = n;
        while temp > 0 {
            bits.push((temp & 1) as i8);
            temp >>= 1;
        }
        bits.reverse();
        bits
    };
    let len = bits.len();
    let mut res = 1 + P[len - 1];
    let half = len.div_ceil(2);
    for (i, &bit) in bits[..half].iter().enumerate().skip(1) {
        // ..,1,...
        //    i     half
        //     <---> this length is half-i-1, count all combinations here
        // 2^(half-i-1)
        if bit == 1 {
            res += 1 << (half - i - 1);
        }
    }
    let rev: Vec<_> = bits[..half].iter().rev().copied().collect();
    res += i32::from(rev.as_slice() <= &bits[len - half..]);
    res
}

const N: usize = 52;
// For "shorter" numbers, let k = half width
// All together there are 2^(k-1) choices, -1 to exclude leading zero
const P: [i32; N] = {
    let mut p = [0; 52];
    let mut i = 1;
    while i < N {
        p[i] = 1 << (i.div_ceil(2) - 1);
        i += 1;
    }
    i = 1;
    while i < N {
        p[i] += p[i - 1];
        i += 1;
    }
    p
};

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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
        assert_eq!(count_binary_palindromes(4), 3);
        assert_eq!(count_binary_palindromes(9), 6);
    }

    #[test]
    fn test() {}
}
