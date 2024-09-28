mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn count_arrangement(n: i32) -> i32 {
    dfs(n, 1, 0, &mut HashMap::new())
}

fn dfs(n: i32, num: i32, bits: i32, seen: &mut HashMap<(i32, i32), i32>) -> i32 {
    if num > n {
        return 1;
    }
    if let Some(&v) = seen.get(&(num, bits)) {
        return v;
    }
    let mut res = 0;
    // try all 1..=n on current bit idx
    // which is tracked using bits
    for idx in 1..=n {
        let curr = bits & (1 << idx);
        // current bit is empty and num fits on idx
        if curr == 0 && (num % idx == 0 || idx % num == 0) {
            res += dfs(n, num + 1, bits | (1 << idx), seen);
        }
    }
    seen.insert((num, bits), res);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(count_arrangement(2), 2);
        debug_assert_eq!(count_arrangement(1), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(count_arrangement(3), 3);
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
