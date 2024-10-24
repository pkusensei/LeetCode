mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn soup_servings(n: i32) -> f64 {
    if n > 4800 {
        return 1.0;
    }
    dfs(n, n, &mut HashMap::new())
}

fn dfs(a: i32, b: i32, seen: &mut HashMap<[i32; 2], f64>) -> f64 {
    match (a <= 0, b <= 0) {
        (true, true) => 0.5,
        (true, false) => 1.0,
        (false, true) => 0.0,
        _ => {
            if let Some(&v) = seen.get(&[a, b]) {
                return v;
            }
            let res = 0.25
                * (dfs(a - 100, b, seen)
                    + dfs(a - 75, b - 25, seen)
                    + dfs(a - 50, b - 50, seen)
                    + dfs(a - 25, b - 75, seen));
            seen.insert([a, b], res);
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(soup_servings(50), 0.625);
        debug_assert_eq!(soup_servings(100), 0.71875);
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
}
