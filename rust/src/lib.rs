mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_integers(n: i32) -> i32 {
    // 1 + solve(1, n)
    let mut f = [0; 32];
    f[0] = 1;
    f[1] = 2;
    for i in 2..32 {
        // For f[i-1], append 0 and all are valid
        // For f[i-2], append 01
        f[i] = f[i - 1] + f[i - 2];
    }
    let mut bit = 30;
    let mut res = 1;
    let mut prev = 0;
    loop {
        // Scan from most significant bit
        if n & (1 << bit) > 0 {
            // add in counts to reach this bit
            res += f[bit];
            // prev==1 => ..11..
            if prev == 1 {
                res -= 1;
                break;
            }
            prev = 1;
        } else {
            prev = 0;
        }
        if bit == 0 {
            break;
        }
        bit -= 1;
    }
    res
}

fn solve(curr: i32, n: i32) -> i32 {
    if curr > n {
        return 0;
    }
    if curr & 1 == 1 {
        // odd; can only append 0
        1 + solve(curr << 1, n)
    } else {
        // even; can append 1 and 0
        1 + solve(curr << 1, n) + solve((curr << 1) | 1, n)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_integers(5), 5);
        debug_assert_eq!(find_integers(1), 2);
        debug_assert_eq!(find_integers(2), 3);
    }

    #[test]
    fn test() {
        debug_assert_eq!(find_integers(4), 4);
    }

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
}
