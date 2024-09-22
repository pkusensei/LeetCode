mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
    if desired_total < 1 {
        return true;
    }
    if max_choosable_integer * (1 + max_choosable_integer) / 2 < desired_total {
        return false;
    }
    dfs(0, max_choosable_integer, desired_total, &mut HashMap::new())
}

fn dfs(
    used: i32,
    max_choosable_integer: i32,
    desired_total: i32,
    seen: &mut HashMap<i32, bool>,
) -> bool {
    if desired_total <= 0 {
        return false;
    }
    if let Some(v) = seen.get(&used) {
        return *v;
    }
    for trial in 1..=max_choosable_integer {
        if used & (1 << trial) > 0 {
            // each bit in used is a record of which number is 'used'
            // it can have at most 21 bit set
            continue;
        }
        let record = used | (1 << trial);
        // trial is the current player-chosen
        let v = if trial == desired_total {
            true
        } else {
            // let opponent choose => flip result
            !dfs(record, max_choosable_integer, desired_total - trial, seen)
        };
        if v {
            seen.insert(used, v);
            return true;
        }
    }
    seen.insert(used, false);
    false
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(can_i_win(10, 0));
        debug_assert!(!can_i_win(10, 11));
        debug_assert!(can_i_win(10, 1));
    }

    #[test]
    fn test() {
        debug_assert!(!can_i_win(10, 40));
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
